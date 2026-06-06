//! smearor-wrot-wrapper: CLI application providing the complete window solution

pub mod cli;
pub mod config;

use crate::cli::args::Arguments;
use clap::Parser;
use smearor_wrot_application::CompositorApplication;
use smearor_wrot_application::CompositorApplicationConfig;
use smearor_wrot_application::DEFAULT_WINDOW_HEIGHT;
use smearor_wrot_application::DEFAULT_WINDOW_WIDTH;
use smearor_wrot_application::init_logging;
use smearor_wrot_application::SmearorLayer;
use std::error::Error;
use std::sync::Arc;
use tracing::debug;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Note: GSK_RENDERER is not set here to allow users to set it via environment variable
    // If GSK_RENDERER is set in the parent process, it will be inherited by child processes

    init_logging();

    let mut command_line_arguments = Arc::new(Arguments::parse());

    // Load configuration file if provided
    if let Some(config_path) = &command_line_arguments.config {
        match config::load_config(config_path) {
            Ok(config) => {
                debug!("Loaded configuration from: {:?}", config_path);

                // Merge configuration with CLI arguments (CLI takes precedence)
                let mut args = (*command_line_arguments).clone();

                // Window configuration
                if args.title.is_none() && config.window.title.is_some() {
                    args.title = config.window.title;
                }
                if args.width == DEFAULT_WINDOW_WIDTH && config.window.width.is_some() {
                    args.width = config.window.width.unwrap();
                }
                if args.height == DEFAULT_WINDOW_HEIGHT && config.window.height.is_some() {
                    args.height = config.window.height.unwrap();
                }
                if args.decorated && config.window.decorated.is_some() {
                    args.decorated = config.window.decorated.unwrap();
                }
                if args.resizable && config.window.resizable.is_some() {
                    args.resizable = config.window.resizable.unwrap();
                }
                if args.position_x.is_none() && config.window.position_x.is_some() {
                    args.position_x = config.window.position_x;
                }
                if args.position_y.is_none() && config.window.position_y.is_some() {
                    args.position_y = config.window.position_y;
                }
                if args.min_width.is_none() && config.window.min_width.is_some() {
                    args.min_width = config.window.min_width;
                }
                if args.min_height.is_none() && config.window.min_height.is_some() {
                    args.min_height = config.window.min_height;
                }
                if args.max_width.is_none() && config.window.max_width.is_some() {
                    args.max_width = config.window.max_width;
                }
                if args.max_height.is_none() && config.window.max_height.is_some() {
                    args.max_height = config.window.max_height;
                }
                if args.aspect_ratio.is_none() && config.window.aspect_ratio.is_some() {
                    args.aspect_ratio = config.window.aspect_ratio;
                }
                if !args.fullscreen && config.window.fullscreen.is_some() {
                    args.fullscreen = config.window.fullscreen.unwrap();
                }
                if !args.maximized && config.window.maximized.is_some() {
                    args.maximized = config.window.maximized.unwrap();
                }

                // Compositor configuration
                if args.double_buffer && config.compositor.double_buffer.is_some() {
                    args.double_buffer = config.compositor.double_buffer.unwrap();
                }
                if !args.disable_rotation && config.compositor.disable_rotation.is_some() {
                    args.disable_rotation = config.compositor.disable_rotation.unwrap();
                }
                if args.rotation == 0.0 && config.compositor.rotation.is_some() {
                    args.rotation = config.compositor.rotation.unwrap();
                }
                if args.socket.as_deref() == Some("/tmp/io.smearor.wrot.sock") && config.compositor.socket.is_some() {
                    args.socket = config.compositor.socket;
                }
                if args.layer.is_none() && config.compositor.layer.is_some() {
                    if let Some(layer_str) = config.compositor.layer.as_ref() {
                        args.layer = Some(SmearorLayer::from(layer_str.as_str()));
                    }
                }
                if args.namespace.is_none() && config.compositor.namespace.is_some() {
                    args.namespace = config.compositor.namespace;
                }
                if !args.shell && config.compositor.shell.is_some() {
                    args.shell = config.compositor.shell.unwrap();
                }
                if !args.disable_dma_buf && config.compositor.disable_dma_buf.is_some() {
                    args.disable_dma_buf = config.compositor.disable_dma_buf.unwrap();
                }

                if !args.disable_client_decorations && config.compositor.disable_client_decorations.is_some() {
                    args.disable_client_decorations = config.compositor.disable_client_decorations.unwrap();
                }
                if args.margin_left == 0 && config.compositor.margin_left.is_some() {
                    args.margin_left = config.compositor.margin_left.unwrap();
                }
                if args.margin_right == 0 && config.compositor.margin_right.is_some() {
                    args.margin_right = config.compositor.margin_right.unwrap();
                }
                if args.margin_top == 0 && config.compositor.margin_top.is_some() {
                    args.margin_top = config.compositor.margin_top.unwrap();
                }
                if args.margin_bottom == 0 && config.compositor.margin_bottom.is_some() {
                    args.margin_bottom = config.compositor.margin_bottom.unwrap();
                }
                if args.dialog_margin == 0 && config.compositor.dialog_margin.is_some() {
                    args.dialog_margin = config.compositor.dialog_margin.unwrap();
                }
                if args.opacity == 1.0 && config.compositor.opacity.is_some() {
                    args.opacity = config.compositor.opacity.unwrap();
                }
                if args.background_color.is_none() && config.compositor.background_color.is_some() {
                    args.background_color = config.compositor.background_color.clone();
                }
                if args.window_opacity == 1.0 && config.compositor.window_opacity.is_some() {
                    args.window_opacity = config.compositor.window_opacity.unwrap();
                }
                if args.max_fps == 60 && config.compositor.max_fps.is_some() {
                    args.max_fps = config.compositor.max_fps.unwrap();
                }
                if !args.color_mask_shader && config.compositor.color_mask_shader.is_some() {
                    args.color_mask_shader = config.compositor.color_mask_shader.unwrap();
                }
                if !args.disable_animations && config.compositor.disable_animations.is_some() {
                    args.disable_animations = config.compositor.disable_animations.unwrap();
                }

                command_line_arguments = Arc::new(args);
            }
            Err(e) => {
                eprintln!("Failed to load configuration file: {}", e);
                std::process::exit(1);
            }
        }
    }

    // Disable DMA-BUF if color mask is set (color mask only works with SHM)
    // This must happen after config loading to override config settings
    let mut args = (*command_line_arguments).clone();
    command_line_arguments = Arc::new(args);

    let application_config = CompositorApplicationConfig::builder()
        .disable_rotation(command_line_arguments.disable_rotation)
        .rotation(command_line_arguments.rotation)
        .width(command_line_arguments.width)
        .height(command_line_arguments.height)
        .decorated(command_line_arguments.decorated)
        .resizable(command_line_arguments.resizable)
        .position_x(command_line_arguments.position_x)
        .position_y(command_line_arguments.position_y)
        .min_width(command_line_arguments.min_width)
        .min_height(command_line_arguments.min_height)
        .max_width(command_line_arguments.max_width)
        .max_height(command_line_arguments.max_height)
        .aspect_ratio(command_line_arguments.aspect_ratio)
        .fullscreen(command_line_arguments.fullscreen)
        .maximized(command_line_arguments.maximized)
        .double_buffer(command_line_arguments.double_buffer)
        .disable_dma_buf(command_line_arguments.disable_dma_buf)
        .id(command_line_arguments.id.clone())
        .title(command_line_arguments.title.clone())
        .layer(command_line_arguments.layer)
        .namespace(command_line_arguments.namespace.clone())
        .shell(command_line_arguments.shell)
        .socket(command_line_arguments.socket.clone())
        .config_path(command_line_arguments.config.clone())
        .wayland_debug(command_line_arguments.wayland_debug)
        .gsk_renderer_gl(command_line_arguments.gsk_renderer_gl)
        .disable_client_decorations(command_line_arguments.disable_client_decorations)
        .margin(command_line_arguments.margin)
        .margin_left(command_line_arguments.margin_left)
        .margin_right(command_line_arguments.margin_right)
        .margin_top(command_line_arguments.margin_top)
        .margin_bottom(command_line_arguments.margin_bottom)
        .opacity(command_line_arguments.opacity)
        .background_color(command_line_arguments.background_color.clone())
        .subsurface_background_color(command_line_arguments.subsurface_background_color.clone())
        .color_mask(command_line_arguments.color_mask.clone())
        .auto_color_mask(command_line_arguments.auto_color_mask)
        .subsurface_color_mask(command_line_arguments.subsurface_color_mask.clone())
        .auto_subsurface_color_mask(command_line_arguments.auto_subsurface_color_mask)
        .color_mask_tolerance(command_line_arguments.color_mask_tolerance)
        .color_mask_shader(command_line_arguments.color_mask_shader)
        .window_opacity(command_line_arguments.window_opacity)
        .max_fps(command_line_arguments.max_fps)
        .dialog_margin(command_line_arguments.dialog_margin)
        .animation_speed(command_line_arguments.animation_speed)
        .animation_overshoot(command_line_arguments.animation_overshoot)
        .disable_animations(command_line_arguments.disable_animations)
        .debug_touch(command_line_arguments.debug_touch)
        .debug_pointer(command_line_arguments.debug_pointer)
        .override_wayland_display(command_line_arguments.override_wayland_display.clone())
        .keyboard_layout(command_line_arguments.keyboard_layout.clone())
        .keyboard_variant(command_line_arguments.keyboard_variant.clone())
        .command_arguments(command_line_arguments.command_arguments.clone())
        .build();

    let application = CompositorApplication::builder()
        .config(application_config)
        .build();

    application.run()?;

    Ok(())
}
