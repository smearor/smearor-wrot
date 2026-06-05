//! Winit backend integration for the compositor

use crate::compositor::CalloopData;
use crate::compositor::SmearorCompositor;
use crate::error::CoreError;
use crate::error::Result;
use crate::frame::limit::FrameLimiter;
use crate::input::InputProcessing;
use smithay::backend::renderer::damage::OutputDamageTracker;
use smithay::backend::renderer::element::surface::WaylandSurfaceRenderElement;
use smithay::backend::renderer::gles::GlesRenderer;
use smithay::backend::winit::WinitEvent;
use smithay::backend::winit::{self};
use smithay::output::Mode;
use smithay::output::Output;
use smithay::output::PhysicalProperties;
use smithay::output::Subpixel;
use smithay::reexports::calloop::EventLoop;
use smithay::utils::Rectangle;
use smithay::utils::Transform;
use std::sync::atomic::Ordering;
use std::time::Duration;
use tracing::error;

/// Initialize the winit backend
pub fn init_winit(event_loop: &mut EventLoop<CalloopData>, data: &mut CalloopData) -> Result<()> {
    let display_handle = &mut data.display_handle;
    let state = &mut data.state;

    let (mut backend, winit) = winit::init().map_err(|e| CoreError::compositor(format!("Failed to init winit: {}", e)))?;

    let mode = Mode {
        size: backend.window_size(),
        refresh: 60_000,
    };

    let output = Output::new(
        "winit".to_string(),
        PhysicalProperties {
            size: (0, 0).into(),
            subpixel: Subpixel::Unknown,
            make: "Smearor".into(),
            model: "Winit".into(),
        },
    );
    let _global = output.create_global::<SmearorCompositor>(display_handle);
    output.change_current_state(Some(mode), Some(Transform::Flipped180), None, Some((0, 0).into()));
    output.set_preferred(mode);

    state.lock().unwrap().space.map_output(&output, (0, 0));

    let mut damage_tracker = OutputDamageTracker::from_output(&output);

    unsafe {
        std::env::set_var("WAYLAND_DISPLAY", &state.lock().unwrap().socket_name);
    }

    event_loop
        .handle()
        .insert_source(winit, move |event, _, data| {
            let display = &mut data.display_handle;
            let state = &mut data.state;

            match event {
                WinitEvent::Resized { size, .. } => {
                    output.change_current_state(Some(Mode { size, refresh: 60_000 }), None, None, None);
                }
                WinitEvent::Input(event) => state.lock().unwrap().process_input_event(event),
                WinitEvent::Redraw => {
                    let size = backend.window_size();
                    let damage = Rectangle::from_size(size);

                    {
                        let (renderer, mut framebuffer) = match backend.bind() {
                            Ok(result) => result,
                            Err(e) => {
                                error!("Failed to bind backend: {}", e);
                                return;
                            }
                        };

                        if smithay::desktop::space::render_output::<_, WaylandSurfaceRenderElement<GlesRenderer>, _, _>(
                            &output,
                            renderer,
                            &mut framebuffer,
                            1.0,
                            0,
                            [&state.lock().unwrap().space],
                            &[],
                            &mut damage_tracker,
                            [0.1, 0.1, 0.1, 1.0],
                        )
                        .is_err()
                        {
                            error!("Failed to render output");
                        }
                    }

                    if backend.submit(Some(&[damage])).is_err() {
                        error!("Failed to submit frame");
                    }

                    let elapsed_since_start = state.lock().map(|state| state.elapsed_since_start()).unwrap_or(Duration::ZERO);
                    let should_send_frame = state.lock().map(|state| state.should_send_frame()).unwrap_or(true);

                    if should_send_frame {
                        if let Ok(state) = state.lock() {
                            state
                                .space
                                .elements()
                                .for_each(|window| window.send_frame(&output, elapsed_since_start, Some(Duration::ZERO), |_, _| Some(output.clone())));
                        }
                    }

                    if let Ok(mut state) = state.lock() {
                        state.space.refresh();
                    }
                    // state.lock().unwrap().space.refresh();
                    if let Ok(mut state) = state.lock() {
                        state.popups.cleanup();
                    }

                    let _ = display.flush_clients();

                    let frame_rate_limit_ms = state.lock().ok().map(|state| state.frame_rate_limit_ms.load(Ordering::Relaxed)).unwrap_or(-1);
                    if frame_rate_limit_ms <= 0 {
                        // Ask for redraw to schedule new frame.
                        backend.window().request_redraw();
                    } else {
                        // TODO: Add timer to schedule new frame.
                    }
                }
                WinitEvent::CloseRequested => {
                    if let Ok(state) = state.lock() {
                        state.loop_signal.stop();
                    }
                }
                _ => (),
            };
        })
        .map_err(|e| CoreError::compositor(format!("Failed to insert winit source: {}", e)))?;

    Ok(())
}
