//! Winit backend integration for the compositor

use std::time::Duration;

use smithay::backend::renderer::damage::OutputDamageTracker;
use smithay::backend::renderer::element::surface::WaylandSurfaceRenderElement;
use smithay::backend::renderer::gles::GlesRenderer;
use smithay::backend::winit::{self, WinitEvent};
use smithay::output::{Mode, Output, PhysicalProperties, Subpixel};
use smithay::reexports::calloop::EventLoop;
use smithay::utils::{Rectangle, Transform};

use crate::compositor::{CalloopData, SmearorCompositor};
use crate::error::{CoreError, Result};
use crate::input::InputProcessing;

use tracing::error;

/// Initialize the winit backend
pub fn init_winit(
    event_loop: &mut EventLoop<CalloopData>,
    data: &mut CalloopData,
) -> Result<()> {
    let display_handle = &mut data.display_handle;
    let state = &mut data.state;

    let (mut backend, winit) = winit::init()
        .map_err(|e| CoreError::compositor(format!("Failed to init winit: {}", e)))?;

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

    unsafe { std::env::set_var("WAYLAND_DISPLAY", &state.lock().unwrap().socket_name); }

    event_loop.handle().insert_source(winit, move |event, _, data| {
        let display = &mut data.display_handle;
        let state = &mut data.state;

        match event {
            WinitEvent::Resized { size, .. } => {
                output.change_current_state(
                    Some(Mode {
                        size,
                        refresh: 60_000,
                    }),
                    None,
                    None,
                    None,
                );
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

                    if smithay::desktop::space::render_output::<
                        _,
                        WaylandSurfaceRenderElement<GlesRenderer>,
                        _,
                        _,
                    >(
                        &output,
                        renderer,
                        &mut framebuffer,
                        1.0,
                        0,
                        [&state.lock().unwrap().space],
                        &[],
                        &mut damage_tracker,
                        [0.1, 0.1, 0.1, 1.0],
                    ).is_err() {
                        error!("Failed to render output");
                    }
                }

                if backend.submit(Some(&[damage])).is_err() {
                    error!("Failed to submit frame");
                }

                state.lock().unwrap().space.elements().for_each(|window| {
                    window.send_frame(
                        &output,
                        state.lock().unwrap().start_time.elapsed(),
                        Some(Duration::ZERO),
                        |_, _| Some(output.clone()),
                    )
                });

                state.lock().unwrap().space.refresh();
                state.lock().unwrap().popups.cleanup();
                let _ = display.flush_clients();

                // Ask for redraw to schedule new frame.
                backend.window().request_redraw();
            }
            WinitEvent::CloseRequested => {
                state.lock().unwrap().loop_signal.stop();
            }
            _ => (),
        };
    })
    .map_err(|e| CoreError::compositor(format!("Failed to insert winit source: {}", e)))?;

    Ok(())
}
