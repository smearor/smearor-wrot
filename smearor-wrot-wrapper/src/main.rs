//! smearor-wrot-wrapper: CLI application providing the complete window solution

pub mod args;
pub mod config;
pub mod icon;
pub mod keyboard_layout;
pub mod screenshot;
pub mod settings;
pub mod socket;

use crate::args::Arguments;
use crate::keyboard_layout::detect_keyboard_layout;
use crate::keyboard_layout::KeyboardLayout;
use crate::screenshot::ScreenshotManager;
use crate::socket::build_socket_path;
use crate::socket::check_socket_exists;
use crate::socket::generate_unique_socket_name;
use clap::Parser;
use gtk4::gdk::Display;
use gtk4::gdk::Toplevel;
use gtk4::gio::ApplicationFlags;
use gtk4::glib;
use gtk4::glib::ControlFlow;
use gtk4::prelude::*;
use gtk4::subclass::prelude::ObjectSubclassIsExt;
use gtk4_layer_shell::LayerShell;
use smearor_wrot_core::DEFAULT_WINDOW_HEIGHT;
use smearor_wrot_core::DEFAULT_WINDOW_WIDTH;
use smearor_wrot_core::DoubleBuffer;
use smearor_wrot_core::ObjectId;
use smearor_wrot_core::background::subsurface::SubsurfaceBackground;
use smearor_wrot_core::background::toplevel::ToplevelBackground;
use smearor_wrot_core::callback::commit::CommitCallbackAware;
use smearor_wrot_core::color_mask::mask::ColorMask;
use smearor_wrot_core::color_mask::subsurface::SubSurfaceColorMask;
use smearor_wrot_core::color_mask::toplevel::TopLevelColorMask;
use smearor_wrot_core::dma::buffer::DmaBuffer;
use smearor_wrot_core::frame::limit::FrameLimiter;
use smearor_wrot_core::init_logging;
use smearor_wrot_core::margin::handler::MarginHandler;
use smearor_wrot_core::message::compositor_message::CompositorMessage;
use smearor_wrot_core::message::sender::CompositorMessageSender;
use smearor_wrot_core::windows::decoration::ClientDecorationAware;
use smearor_wrot_core::windows::title::WindowTitle;
use smearor_wrot_gtk::CompositorWidget;
use smearor_wrot_gtk::clipboard::sync_manager::SyncManager;
use smearor_wrot_gtk::event_handler::keyboard::KeyboardInputEventHandler;
use smearor_wrot_gtk::widget::compositor::handler::CompositorHandler;
use smearor_wrot_gtk::widget::config::handler::ConfigHandler;
use smearor_wrot_gtk::widget::resize::handler::ResizeHandler;
use smearor_wrot_gtk::widget::shutdown::handler::ShutdownHandler;
use smearor_wrot_gtk::widget::socket::handler::SocketHandler;
use smearor_wrot_gtk::widget::window_state::handler::WindowStateHandler;
use smearor_wrot_model::color::RgbColor;
use smearor_wrot_model::color::rgba::RgbaColor;
use smearor_wrot_model::geometry::size::Size;
use smearor_wrot_model::margin::Margins;
use smearor_wrot_pie_menu::PieMenuMessage;
use smearor_wrot_pie_menu::PieMenuOverlayWidget;
use smearor_wrot_pie_menu::RotationHandler;
use smearor_wrot_pie_menu::overlay_widget::message::handler::PieMenuMessageSender;
use smearor_wrot_rotation::RotationControlHandler;
use smearor_wrot_rotation::RotationWidget;
use smearor_wrot_rotation::SmearorRotation;
use smearor_wrot_rotation::layer::SmearorLayer;
use std::cell::RefCell;
use std::error::Error;
use std::ffi::OsStr;
use std::io::BufRead;
use std::path::PathBuf;
use std::process::Command;
use std::process::Stdio;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use tracing::debug;
use tracing::error;
use tracing::info;
use tracing::warn;
use which::which;

/// Parse a hex color string (e.g., "#FF0000" or "FF0000") into RGB values (0.0-1.0 range)
fn parse_hex_color(hex: &str) -> Result<RgbaColor, String> {
    let hex = hex.trim_start_matches('#');

    if hex.len() != 6 && hex.len() != 8 {
        return Err("Hex color must be 6 characters (e.g., #FF0000) or 8 characters with alpha (e.g., #FF000077)".to_string());
    }

    let r = u8::from_str_radix(&hex[0..2], 16).map_err(|e| format!("Failed to parse red component: {}", e))?;
    let g = u8::from_str_radix(&hex[2..4], 16).map_err(|e| format!("Failed to parse green component: {}", e))?;
    let b = u8::from_str_radix(&hex[4..6], 16).map_err(|e| format!("Failed to parse blue component: {}", e))?;

    // Alpha channel is optional, default to 1.0 (fully opaque)
    let a = if hex.len() == 8 {
        u8::from_str_radix(&hex[6..8], 16).map_err(|e| format!("Failed to parse alpha component: {}", e))? as f32 / 255.0
    } else {
        1.0
    };

    Ok(RgbaColor::new(RgbColor::new_from_u8(r, g, b), a))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Note: GSK_RENDERER is not set here to allow users to set it via environment variable
    // If GSK_RENDERER is set in the parent process, it will be inherited by child processes

    init_logging();

    let mut command_line_arguments = Arc::new(Arguments::parse());

    // Determine the socket name
    let socket_name = if let Some(ref socket) = command_line_arguments.socket {
        // User specified a socket explicitly
        check_socket_exists(socket)?;
        socket.clone()
    } else {
        // Generate a unique socket name automatically
        generate_unique_socket_name("smearor-wrot")?
    };

    // Build the full socket path from the relative name in XDG_RUNTIME_DIR
    let socket_path = build_socket_path(&socket_name)?;
    let socket_path_str = socket_path.to_string_lossy().to_string();

    // Update command_line_arguments with the full socket path for compositor initialization
    let mut args = (*command_line_arguments).clone();
    args.socket = Some(socket_path_str.clone());
    command_line_arguments = Arc::new(args);

    // Store the full socket path for WAYLAND_DISPLAY environment variable
    // This is necessary for confined environments like Snap which have different XDG_RUNTIME_DIR
    let socket_name_for_env = socket_path_str.clone();

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

    let application_id = command_line_arguments.id.clone().unwrap_or(format!("io.smearor.wrot.p{}", std::process::id()));

    debug!("Starting smearor-wrot GTK4 application with application id {application_id}");

    // Override WAYLAND_DISPLAY if specified via CLI argument
    if let Some(override_wayland_display) = &command_line_arguments.override_wayland_display {
        debug!("Overriding WAYLAND_DISPLAY to: {override_wayland_display}");
        unsafe {
            std::env::set_var("WAYLAND_DISPLAY", override_wayland_display);
        }
    }

    // Clone command_line_arguments for use in closure and later
    let command_line_arguments_for_closure = command_line_arguments.clone();

    // Initialize GTK4 application
    // Use a unique application ID to avoid D-Bus conflicts
    let app = gtk4::Application::builder()
        .application_id(&application_id)
        .flags(ApplicationFlags::CAN_OVERRIDE_APP_ID | ApplicationFlags::NON_UNIQUE)
        .build();

    app.connect_activate(move |app| {
        debug!("Application activate callback called");

        // Create the main window
        let window = gtk4::ApplicationWindow::builder()
            .application(app)
            .title(command_line_arguments_for_closure.title.as_deref().unwrap_or("Smearor Compositor"))
            .startup_id("org.gnome.Chess")
            .default_width(command_line_arguments_for_closure.width)
            .default_height(command_line_arguments_for_closure.height)
            .resizable(command_line_arguments_for_closure.resizable)
            .decorated(command_line_arguments_for_closure.decorated)
            .build();

        // Initialize layer shell if layer is specified
        if let Some(layer) = command_line_arguments_for_closure.layer.as_ref() {
            if gtk4_layer_shell::is_supported() {
                window.init_layer_shell();
                let gtk_layer = gtk4_layer_shell::Layer::from(*layer);
                window.set_layer(gtk_layer);
                debug!("Layer shell initialized with layer: {:?}", layer);

                if let Some(namespace) = command_line_arguments_for_closure.namespace.as_ref() {
                    window.set_namespace(Some(namespace.as_str()));
                    debug!("Layer shell namespace set to: {}", namespace);
                }
            } else {
                debug!("Layer shell protocol not supported, falling back to regular window");
            }
        }

        // Create header bar with rotation buttons
        let header_bar = gtk4::HeaderBar::builder().show_title_buttons(true).build();

        let title = command_line_arguments_for_closure.title.as_deref().unwrap_or("Smearor Compositor");
        let title_label = gtk4::Label::builder().label(title).build();
        header_bar.set_title_widget(Some(&title_label));

        // Create rotation buttons
        let rotate_counter_clockwise_button = gtk4::Button::builder()
            .icon_name("object-rotate-left-symbolic")
            .tooltip_text("Rotate Counter Clockwise")
            .css_classes(["large-button"])
            .build();

        let rotate_clockwise_button = gtk4::Button::builder()
            .icon_name("object-rotate-right-symbolic")
            .tooltip_text("Rotate Clockwise")
            .css_classes(["large-button"])
            .build();

        let reset_rotation_button = gtk4::Button::builder()
            .icon_name("view-refresh-symbolic")
            .tooltip_text("Reset Rotation")
            .css_classes(["large-button"])
            .build();

        // Create clipboard buttons
        let paste_button = gtk4::Button::builder()
            .icon_name("edit-paste-symbolic")
            .tooltip_text("Paste from Host System to Compositor")
            .css_classes(["large-button"])
            .build();

        let copy_button = gtk4::Button::builder()
            .icon_name("edit-copy-symbolic")
            .tooltip_text("Copy from Compositor to Host System")
            .css_classes(["large-button"])
            .build();

        // Group rotate buttons together
        let rotate_box = gtk4::Box::builder().orientation(gtk4::Orientation::Horizontal).spacing(4).build();
        rotate_box.append(&rotate_counter_clockwise_button);
        rotate_box.append(&rotate_clockwise_button);
        rotate_box.append(&reset_rotation_button);

        // Add separator between button groups
        let separator = gtk4::Separator::builder()
            .orientation(gtk4::Orientation::Horizontal)
            .margin_start(8)
            .margin_end(8)
            .build();

        // Group clipboard buttons together
        let clipboard_box = gtk4::Box::builder().orientation(gtk4::Orientation::Horizontal).spacing(4).build();
        clipboard_box.append(&paste_button);
        clipboard_box.append(&copy_button);

        // Add separator between button groups
        let settings_separator = gtk4::Separator::builder()
            .orientation(gtk4::Orientation::Horizontal)
            .margin_start(8)
            .margin_end(8)
            .build();

        // Create settings button
        let settings_button = gtk4::Button::builder()
            .icon_name("preferences-system-symbolic")
            .tooltip_text("Settings")
            .css_classes(["large-button"])
            .build();

        // Create screenshot button
        let screenshot_button = gtk4::Button::builder()
            .icon_name("camera-photo-symbolic")
            .tooltip_text("Save screenshot")
            .css_classes(["large-button"])
            .build();

        // Add button groups to header bar
        header_bar.pack_start(&rotate_box);
        header_bar.pack_start(&separator);
        header_bar.pack_start(&clipboard_box);
        header_bar.pack_start(&settings_separator);
        header_bar.pack_start(&settings_button);
        header_bar.pack_start(&screenshot_button);

        // Set header bar to window
        window.set_titlebar(Some(&header_bar));

        // Apply initial position if both x and y are provided
        if let (Some(x), Some(y)) = (command_line_arguments_for_closure.position_x, command_line_arguments_for_closure.position_y) {
            // Note: GTK4 doesn't have a direct set_position method for ApplicationWindow
            // Position is typically managed by the window manager
            debug!("Initial position requested: ({}, {})", x, y);
        }

        // Apply minimum size constraints if provided
        if let Some(min_width) = command_line_arguments_for_closure.min_width {
            if let Some(min_height) = command_line_arguments_for_closure.min_height {
                window.set_size_request(min_width, min_height);
            } else {
                window.set_size_request(min_width, window.default_height());
            }
        } else if let Some(min_height) = command_line_arguments_for_closure.min_height {
            window.set_size_request(window.default_width(), min_height);
        }

        // Apply maximum size constraints if provided
        if let Some(max_width) = command_line_arguments_for_closure.max_width {
            if let Some(max_height) = command_line_arguments_for_closure.max_height {
                window.set_size_request(max_width, max_height);
            } else {
                window.set_size_request(max_width, window.default_height());
            }
        } else if let Some(max_height) = command_line_arguments_for_closure.max_height {
            window.set_size_request(window.default_width(), max_height);
        }

        // Create channel for communication between compositor core and GTK wrapper
        let (compositor_message_sender, compositor_message_receiver) = mpsc::channel::<CompositorMessage>();

        // Create channel for communication between pie menu and main application
        let (pie_menu_sender, pie_menu_receiver) = mpsc::channel::<PieMenuMessage>();

        // Create the compositor widget
        let compositor_widget = CompositorWidget::new();
        // compositor_widget.set_hexpand(true);
        // compositor_widget.set_vexpand(true);
        // compositor_widget.set_visible(true);
        // Don't set size_request on widget - it sets minimum size, preventing resize
        // compositor_widget.set_size_request(command_line_arguments.width, command_line_arguments.height);

        debug!("Created compositor widget");

        // Disable automatic resize handling to prevent conflicts with rotation
        compositor_widget.set_auto_resize_handling(true);

        // Sync manager will be created after compositor is initialized
        let sync_manager: Arc<Mutex<Option<Arc<SyncManager>>>> = Arc::new(Mutex::new(None));

        // Wrap compositor widget in rotation widget if rotation is enabled
        let rotation_widget: gtk4::Widget = if command_line_arguments_for_closure.disable_rotation {
            compositor_widget.clone().upcast()
        } else {
            let rotation = SmearorRotation::Deg(command_line_arguments_for_closure.rotation);
            let rotation_widget = RotationWidget::new(rotation);
            rotation_widget.set_child(Some(&compositor_widget));
            rotation_widget.set_animation_speed(command_line_arguments_for_closure.animation_speed);
            rotation_widget.set_animations_enabled(!command_line_arguments_for_closure.disable_animations);
            rotation_widget.set_animation_overshoot(command_line_arguments_for_closure.animation_overshoot);

            // Set touch transform callback
            let rotation_widget_clone = rotation_widget.clone();
            compositor_widget.set_touch_transform_callback(move |_sequence, x, y| {
                if let Some(rotation_widget) = rotation_widget_clone.downcast_ref::<RotationWidget>() {
                    rotation_widget.input_transform(x, y)
                } else {
                    (x, y)
                }
            });

            // Set pointer transform callback
            let rotation_widget_clone = rotation_widget.clone();
            compositor_widget.set_pointer_transform_callback(move |x, y| {
                if let Some(rotation_widget) = rotation_widget_clone.downcast_ref::<RotationWidget>() {
                    rotation_widget.input_transform(x, y)
                } else {
                    (x, y)
                }
            });

            rotation_widget.upcast()
        };

        debug!("Rotation widget created with rotation: {}", command_line_arguments_for_closure.rotation);

        // Connect rotation buttons to rotation widget
        if !command_line_arguments_for_closure.disable_rotation {
            let initial_rotation = command_line_arguments_for_closure.rotation;
            let rotation_widget_clone = rotation_widget.clone();
            let compositor_widget_clone = compositor_widget.clone();
            rotate_counter_clockwise_button.connect_clicked(move |_| {
                if let Some(rotation_widget) = rotation_widget_clone.downcast_ref::<RotationWidget>() {
                    let current_rotation = rotation_widget.rotation();
                    let rotation_deg = (current_rotation % 360.0).abs();

                    // Find next standard rotation in counter-clockwise direction
                    let new_rotation = if rotation_deg > 360.0 || (rotation_deg > 0.0 && rotation_deg <= 90.0) {
                        0.0
                    } else if rotation_deg > 90.0 && rotation_deg <= 180.0 {
                        90.0
                    } else if rotation_deg > 180.0 && rotation_deg <= 270.0 {
                        180.0
                    } else {
                        //  if rotation_deg <= 0.0 || (rotation_deg > 270.0 && rotation_deg <= 360.0)
                        270.0
                    };

                    rotation_widget.set_rotation_with_animation(new_rotation);

                    // Inform compositor about size change using actual widget size
                    let last_width = compositor_widget_clone.width();
                    let last_height = compositor_widget_clone.height();
                    let new_size = if (new_rotation - 90.0).abs() < 1.0 || (new_rotation - 270.0).abs() < 1.0 {
                        // 90 or 270 degrees: swap width and height
                        Size::new(last_height, last_width)
                    } else {
                        // 0 or 180 degrees: keep original width and height
                        Size::new(last_width, last_height)
                    };
                    compositor_widget_clone.handle_resize(new_size);
                }
            });

            let rotation_widget_clone = rotation_widget.clone();
            let compositor_widget_clone = compositor_widget.clone();
            rotate_clockwise_button.connect_clicked(move |_| {
                if let Some(rotation_widget) = rotation_widget_clone.downcast_ref::<RotationWidget>() {
                    let current_rotation = rotation_widget.rotation();
                    let rotation_deg = (current_rotation % 360.0).abs();

                    // Find next standard rotation in clockwise direction
                    let new_rotation = if rotation_deg >= 0.0 && rotation_deg < 90.0 {
                        90.0
                    } else if rotation_deg >= 90.0 && rotation_deg < 180.0 {
                        180.0
                    } else if rotation_deg >= 180.0 && rotation_deg < 270.0 {
                        270.0
                    } else {
                        //  if rotation_deg >= 270.0 || rotation_deg < 0.0
                        0.0
                    };

                    rotation_widget.set_rotation_with_animation(new_rotation);

                    // Inform compositor about size change using actual widget size
                    let last_width = compositor_widget_clone.width();
                    let last_height = compositor_widget_clone.height();
                    let new_size = if (new_rotation - 90.0).abs() < 1.0 || (new_rotation - 270.0).abs() < 1.0 {
                        // 90 or 270 degrees: swap width and height
                        Size::new(last_height, last_width)
                    } else {
                        // 0 or 180 degrees: keep original width and height
                        Size::new(last_width, last_height)
                    };
                    compositor_widget_clone.handle_resize(new_size);
                }
            });

            let rotation_widget_clone = rotation_widget.clone();
            let compositor_widget_clone = compositor_widget.clone();
            reset_rotation_button.connect_clicked(move |_| {
                if let Some(rotation_widget) = rotation_widget_clone.downcast_ref::<RotationWidget>() {
                    rotation_widget.set_rotation_with_animation(initial_rotation.into());

                    // Inform compositor about size change using actual widget size
                    let last_width = compositor_widget_clone.width();
                    let last_height = compositor_widget_clone.height();
                    let rotation_deg = (initial_rotation % 360.0).abs();
                    let new_size = if (rotation_deg - 90.0).abs() < 1.0 || (rotation_deg - 270.0).abs() < 1.0 {
                        // 90 or 270 degrees: swap width and height
                        Size::new(last_height, last_width)
                    } else {
                        // 0 or 180 degrees: keep original width and height
                        Size::new(last_width, last_height)
                    };
                    compositor_widget_clone.handle_resize(new_size);
                }
            });
        } else {
            // Disable buttons if rotation is disabled
            rotate_counter_clockwise_button.set_sensitive(false);
            rotate_clockwise_button.set_sensitive(false);
            reset_rotation_button.set_sensitive(false);
        }

        // Connect clipboard buttons
        let sync_manager_clone = sync_manager.clone();
        paste_button.connect_clicked(move |_| {
            debug!("Paste button clicked");
            if let Ok(manager_opt) = sync_manager_clone.lock() {
                if let Some(ref manager) = *manager_opt {
                    let manager_clone = manager.clone();
                    glib::MainContext::default().spawn_local(async move {
                        if let Err(e) = manager_clone.manual_paste().await {
                            error!("Failed to paste from host clipboard: {}", e);
                        }
                    });
                } else {
                    warn!("Sync manager not available, paste operation skipped");
                }
            }
        });

        let sync_manager_clone = sync_manager.clone();
        copy_button.connect_clicked(move |_| {
            debug!("Copy button clicked");
            if let Ok(manager_opt) = sync_manager_clone.lock() {
                if let Some(ref manager) = *manager_opt {
                    if let Err(e) = manager.manual_copy() {
                        error!("Failed to copy to host clipboard: {}", e);
                    }
                } else {
                    warn!("Sync manager not available, copy operation skipped");
                }
            }
        });

        let screenshot_manager = Arc::new(ScreenshotManager::new(app.clone(), compositor_widget.clone()));

        // Connect screenshot button
        // let compositor_widget_for_screenshot = compositor_widget.clone();
        let screenshot_manager_clone = screenshot_manager.clone();
        screenshot_button.connect_clicked(move |_| {
            let _ = screenshot_manager_clone.screenshot();
            // compositor_widget_for_screenshot.save_buffer_to_png();
        });

        // Connect settings button
        let compositor_widget_for_settings = compositor_widget.clone();
        let window_for_settings = window.clone();
        let command_line_arguments_for_settings = command_line_arguments_for_closure.clone();
        let rotation_widget_for_settings = rotation_widget.clone();
        settings_button.connect_clicked(move |_| {
            settings::show_settings_dialog(
                (&window_for_settings).as_ref(),
                &compositor_widget_for_settings,
                &rotation_widget_for_settings,
                command_line_arguments_for_settings.disable_dma_buf,
            );
        });

        // Configure the compositor widget from command line arguments
        let title = command_line_arguments_for_closure.title.clone();
        let initial_width = command_line_arguments_for_closure.width;
        let initial_height = command_line_arguments_for_closure.height;
        info!("Configuring compositor with DMA-BUF enabled: {}", !command_line_arguments_for_closure.disable_dma_buf);
        
        // Use CLI parameters if provided, otherwise detect keyboard layout
        let keyboard_layout = if command_line_arguments_for_closure.keyboard_layout.is_some() 
            || command_line_arguments_for_closure.keyboard_variant.is_some() {
            info!("Using CLI keyboard layout parameters");
            Some(KeyboardLayout::new(
                command_line_arguments_for_closure.keyboard_layout.clone().unwrap_or_default(),
                command_line_arguments_for_closure.keyboard_variant.clone()
            ))
        } else {
            info!("Detecting keyboard layout automatically");
            detect_keyboard_layout()
        };
        
        let config = smearor_wrot_gtk::CompositorWidgetConfig {
            show_decorations: command_line_arguments_for_closure.decorated,
            fullscreen: command_line_arguments_for_closure.fullscreen,
            initial_width,
            initial_height,
            title,
            dma_buf_enabled: !command_line_arguments_for_closure.disable_dma_buf,
            min_width: command_line_arguments_for_closure.min_width.unwrap_or(100),
            min_height: command_line_arguments_for_closure.min_height.unwrap_or(100),
            debug_touch: command_line_arguments_for_closure.debug_touch,
            debug_pointer: command_line_arguments_for_closure.debug_pointer,
            auto_color_mask: command_line_arguments_for_closure.auto_color_mask,
            auto_subsurface_color_mask: command_line_arguments_for_closure.auto_subsurface_color_mask,
            color_mask_tolerance: command_line_arguments_for_closure.color_mask_tolerance,
            resizable: command_line_arguments_for_closure.resizable,
            disable_client_decorations: command_line_arguments_for_closure.disable_client_decorations,
            color_mask_shader: command_line_arguments_for_closure.color_mask_shader,
            animations: !command_line_arguments_for_closure.disable_animations,
            max_fps: command_line_arguments_for_closure.max_fps,
            keyboard_layout: keyboard_layout.as_ref().map(|layout| layout.layout.clone()),
            keyboard_variant: keyboard_layout.as_ref().and_then(|layout| layout.variant.clone()),
            ..Default::default()
        };
        compositor_widget.set_config(config);

        debug!("Compositor widget configured with initial size {}x{}", initial_width, initial_height);

        // Set the socket path (this initializes the compositor)
        if let Some(ref socket) = command_line_arguments_for_closure.socket {
            compositor_widget.set_socket_path(socket.clone());
            debug!("Set socket path to: {}", socket);
        } else {
            error!("Socket path not set");
            return;
        }

        // Apply config to compositor after it has been initialized
        let _ = compositor_widget.apply_config_to_compositor();

        // Apply double buffering setting
        if let Ok(compositor) = compositor_widget.compositor() {
            if let Ok(guard) = compositor.lock() {
                guard.set_message_sender(compositor_message_sender);
                guard.set_double_buffer_enabled(command_line_arguments_for_closure.double_buffer);
                guard.set_dma_buf_enabled(!command_line_arguments_for_closure.disable_dma_buf);
                guard.set_client_decorations_enabled(!command_line_arguments_for_closure.disable_client_decorations);

                // Apply margin shortcut if specified
                let (margin_left, margin_right, margin_top, margin_bottom) = if let Some(margin) = command_line_arguments_for_closure.margin {
                    (margin, margin, margin, margin)
                } else {
                    (
                        command_line_arguments_for_closure.margin_left,
                        command_line_arguments_for_closure.margin_right,
                        command_line_arguments_for_closure.margin_top,
                        command_line_arguments_for_closure.margin_bottom,
                    )
                };

                guard.set_margins(
                    Margins::builder()
                        .left(margin_left)
                        .right(margin_right)
                        .top(margin_top)
                        .bottom(margin_bottom)
                        .build(),
                );
                guard.set_dialog_margin(command_line_arguments_for_closure.dialog_margin);
                guard.set_opacity(command_line_arguments_for_closure.opacity);
                guard.set_max_fps(command_line_arguments_for_closure.max_fps);
                guard.set_color_mask_tolerance(command_line_arguments_for_closure.color_mask_tolerance);

                // Parse background color from hex string if provided
                if let Some(hex_color) = &command_line_arguments_for_closure.background_color {
                    if let Ok(rgba_color) = parse_hex_color(hex_color) {
                        let _ = guard.set_background_color(rgba_color);
                    } else {
                        error!("Invalid hex color format: {}", hex_color);
                    }
                }

                // Parse subsurface background color from hex string if provided
                if let Some(hex_color) = &command_line_arguments_for_closure.subsurface_background_color {
                    if let Ok(rgba_color) = parse_hex_color(hex_color) {
                        let _ = guard.set_subsurface_background_color(rgba_color);
                    } else {
                        error!("Invalid hex color format for subsurface background color: {}", hex_color);
                    }
                }

                // Parse color mask from hex string if provided
                if let Some(hex_color) = &command_line_arguments_for_closure.color_mask {
                    if let Ok(rgba_color) = parse_hex_color(hex_color) {
                        let _ = guard.set_color_mask(ColorMask::new(rgba_color.color, command_line_arguments_for_closure.color_mask_tolerance));
                        debug!(
                            "Manual color mask set to {} with tolerance {}",
                            hex_color, command_line_arguments_for_closure.color_mask_tolerance
                        );
                        // Disable DMA-BUF since color mask only works with SHM
                        guard.set_dma_buf_enabled(false);
                        debug!("DMA-BUF disabled because color mask is set");
                    } else {
                        error!("Invalid hex color format for color mask: {}", hex_color);
                    }
                }

                // Auto-detect color mask from first frame if enabled
                if command_line_arguments_for_closure.auto_color_mask {
                    debug!("Auto color mask detection enabled - will detect dominant color from first frame");
                    guard.set_auto_color_mask(true);
                }

                // Parse subsurface color mask from hex string if provided
                if let Some(hex_color) = &command_line_arguments_for_closure.subsurface_color_mask {
                    if let Ok(rgba_color) = parse_hex_color(hex_color) {
                        let _ = guard.set_subsurface_color_mask(ColorMask::new(rgba_color.color, command_line_arguments_for_closure.color_mask_tolerance));
                        debug!(
                            "Manual subsurface color mask set to {} with tolerance {}",
                            hex_color, command_line_arguments_for_closure.color_mask_tolerance
                        );
                    } else {
                        error!("Invalid hex color format for subsurface color mask: {}", hex_color);
                    }
                }

                // Auto-detect subsurface color mask from first frame if enabled
                if command_line_arguments_for_closure.auto_subsurface_color_mask {
                    debug!("Auto subsurface color mask detection enabled - will detect dominant color from subsurfaces");
                    guard.set_auto_subsurface_color_mask(true);
                    // Disable DMA-BUF since color mask only works with SHM
                    guard.set_dma_buf_enabled(false);
                    debug!("DMA-BUF disabled because auto subsurface color mask is enabled");
                }
            }

            // Create sync manager after compositor is initialized
            match SyncManager::new_with_widget(compositor_widget.clone()) {
                Ok(manager) => {
                    let manager = Arc::new(manager);
                    if let Err(e) = manager.start_polling() {
                        error!("Failed to start clipboard polling: {}", e);
                    } else {
                        if let Ok(mut guard) = sync_manager.lock() {
                            *guard = Some(manager);
                        }
                    }
                }
                Err(e) => {
                    error!("Failed to create sync manager: {}", e);
                }
            }
        }

        // Monitor message receiver for maximize/unmaximize requests
        let app_clone = app.clone();
        let window_clone = window.clone();
        let compositor_widget_clone = compositor_widget.clone();
        let sync_manager_clone = sync_manager.clone();
        let initial_window_opacity = command_line_arguments_for_closure.window_opacity as f64;
        let rotation_widget_clone = rotation_widget.clone();
        let command_line_arguments_clone = command_line_arguments_for_closure.clone();
        let application_id_clone = application_id.clone();

        glib::timeout_add_local(Duration::from_millis(16), move || {
            if let Ok(message) = compositor_message_receiver.try_recv() {
                debug!("Received compositor message: {:?}", message);
                match message {
                    CompositorMessage::Maximize => {
                        info!("Received Maximize message from compositor core");
                        window_clone.maximize();
                    }
                    CompositorMessage::Unmaximize => {
                        info!("Received Unmaximize message from compositor core");
                        window_clone.unmaximize();
                    }
                    CompositorMessage::Minimize => {
                        info!("Received Minimize message from compositor core");
                        window_clone.minimize();
                    }
                    CompositorMessage::Fullscreen => {
                        info!("Received Fullscreen message from compositor core");
                        window_clone.set_fullscreened(true);
                    }
                    CompositorMessage::Unfullscreen => {
                        info!("Received Unfullscreen message from compositor core");
                        window_clone.unfullscreen();
                    }
                    CompositorMessage::Resize(width, height) => {
                        info!("Received Resize message from compositor core: {}x{}", width, height);
                        window_clone.set_default_size(width, height);
                    }
                    CompositorMessage::Shutdown => {
                        info!("Received Shutdown message from compositor core");
                        window_clone.close();
                    }
                    CompositorMessage::TitleChanged(title) => {
                        info!("Received TitleChanged message from compositor core: {}", title);
                        compositor_widget_clone.set_header_bar_title(&title);
                    }
                    CompositorMessage::AppIdChanged(app_id) => {
                        info!("Received AppIdChanged message from compositor core: {:?}", app_id);
                        app_clone.set_application_id(Some(&app_id));
                        // window_clone.set_icon_name(Some(&icon_name));
                    }
                    CompositorMessage::WindowMapped => {
                        info!("Received WindowMapped message from compositor core");
                        compositor_widget_clone.notify_window_mapped();
                    }
                    CompositorMessage::FirstCommit => {
                        info!("Received FirstCommit message from compositor core");
                        window_clone.set_opacity(initial_window_opacity);
                    }
                    CompositorMessage::MoveRequest(serial) => {
                        info!("Received MoveRequest message from compositor core");
                        let Some(surface) = window_clone.surface() else {
                            return ControlFlow::Continue;
                        };
                        let Ok(toplevel) = surface.downcast::<Toplevel>() else {
                            return ControlFlow::Continue;
                        };
                        let Some(display) = Display::default() else {
                            return ControlFlow::Continue;
                        };
                        let Some(seat) = display.default_seat() else {
                            return ControlFlow::Continue;
                        };
                        let Some(device) = seat.pointer() else {
                            return ControlFlow::Continue;
                        };
                        toplevel.begin_move(&device, 1, 0.0, 0.0, serial);
                    }
                    CompositorMessage::ResizeRequest(serial) => {
                        info!("Received ResizeRequest message from compositor core");
                        let Some(surface) = window_clone.surface() else {
                            return ControlFlow::Continue;
                        };
                        let Ok(toplevel) = surface.downcast::<Toplevel>() else {
                            return ControlFlow::Continue;
                        };
                        let Some(display) = Display::default() else {
                            return ControlFlow::Continue;
                        };
                        let Some(seat) = display.default_seat() else {
                            return ControlFlow::Continue;
                        };
                        let Some(device) = seat.pointer() else {
                            return ControlFlow::Continue;
                        };
                        // Use all edges for resize
                        let edge = gtk4::gdk::SurfaceEdge::SouthEast;
                        toplevel.begin_resize(edge, Some(&device), 1, 0.0, 0.0, serial);
                    }
                    CompositorMessage::WaylandSelectionChanged => {
                        info!("Received WaylandSelectionChanged message from compositor core");
                        if let Ok(guard) = sync_manager_clone.lock() {
                            if let Some(manager) = guard.as_ref() {
                                if let Err(e) = manager.extract_and_sync_wayland_selection() {
                                    error!("Failed to sync clipboard: {e}");
                                }
                            }
                        }
                    }
                }
            }

            // Check pie menu messages
            // Process all pending messages, but only apply the last rotation value
            let mut last_rotation_message: Option<f32> = None;
            loop {
                match pie_menu_receiver.try_recv() {
                    Ok(message) => {
                        match message {
                            PieMenuMessage::RotateCw => {
                                info!("Received RotateCw message from pie menu");
                                if let Some(rotation_widget) = rotation_widget_clone.downcast_ref::<RotationWidget>() {
                                    let current_rotation = rotation_widget.rotation();
                                    let new_rotation = (current_rotation + 90.0) % 360.0;
                                    rotation_widget.set_rotation_with_animation(new_rotation as f64);
                                }
                            }
                            PieMenuMessage::RotateCcw => {
                                info!("Received RotateCcw message from pie menu");
                                if let Some(rotation_widget) = rotation_widget_clone.downcast_ref::<RotationWidget>() {
                                    let current_rotation = rotation_widget.rotation();
                                    let new_rotation = (current_rotation - 90.0 + 360.0) % 360.0;
                                    rotation_widget.set_rotation_with_animation(new_rotation as f64);
                                }
                            }
                            PieMenuMessage::Rotate(rotation) => {
                                // Store the rotation value, will apply the last one after loop
                                last_rotation_message = Some(rotation);
                            }
                            PieMenuMessage::Settings => {
                                info!("Received Settings message from pie menu");
                                settings::show_settings_dialog(
                                    window_clone.as_ref(),
                                    &compositor_widget_clone,
                                    &rotation_widget_clone,
                                    command_line_arguments_clone.disable_dma_buf,
                                );
                            }
                            PieMenuMessage::Screenshot => {
                                info!("Received Screenshot message from pie menu");
                                if let Some(compositor_widget) = compositor_widget_clone.downcast_ref::<CompositorWidget>() {
                                    let _ = screenshot_manager.screenshot();
                                }
                            }
                            PieMenuMessage::Exit => {
                                info!("Received Exit message from pie menu");
                                window_clone.close();
                            }
                            PieMenuMessage::ToggleMaximize => {
                                info!("Received ToggleMaximize message from pie menu");
                                if window_clone.is_maximized() {
                                    window_clone.unmaximize();
                                } else {
                                    window_clone.maximize();
                                }
                                if let Some(compositor_widget) = compositor_widget_clone.downcast_ref::<CompositorWidget>() {
                                    let _ = compositor_widget.toggle_maximize_first_toplevel();
                                }
                            }
                            PieMenuMessage::Minimize => {
                                info!("Received Minimize message from pie menu");
                                window_clone.minimize();
                            }
                            PieMenuMessage::ToggleFullscreen => {
                                info!("Received ToggleFullscreen message from pie menu");
                                window_clone.set_fullscreened(!window_clone.is_fullscreen());
                                if let Some(compositor_widget) = compositor_widget_clone.downcast_ref::<CompositorWidget>() {
                                    let _ = compositor_widget.toggle_fullscreen_first_toplevel();
                                }
                            }
                            PieMenuMessage::Custom(event) => {
                                info!("Received Custom message from pie menu: {}", event);
                            }
                        }
                    }
                    Err(_) => break,
                }
            }

            // Apply the last rotation message if any
            if let Some(rotation) = last_rotation_message {
                debug!("Applying last rotation message: {} degrees", rotation);
                if let Some(rotation_widget) = rotation_widget_clone.downcast_ref::<RotationWidget>() {
                    rotation_widget.set_rotation(SmearorRotation::Deg(rotation));
                }
            }

            // Check if there are no more surfaces and request shutdown
            let _ = compositor_widget_clone.check_and_request_shutdown();
            ControlFlow::Continue
        });

        // Apply fullscreen if requested
        if command_line_arguments_for_closure.fullscreen {
            window.set_fullscreened(true);
        }

        // Apply maximized if requested
        if command_line_arguments_for_closure.maximized {
            window.maximize();
        }

        // // Apply window opacity if requested
        // if command_line_arguments_for_closure.window_opacity < 1.0 {
        //     window.set_opacity(command_line_arguments_for_closure.window_opacity as f64);
        // }

        // Make window background transparent using CSS
        let provider = gtk4::CssProvider::new();
        provider.load_from_data("window { background-color: transparent; } ");
        if let Some(display) = gtk4::gdk::Display::default() {
            gtk4::style_context_add_provider_for_display(&display, &provider, gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION);

            // Detect keyboard layout
            if let Some(keyboard_layout) = detect_keyboard_layout() {
                info!("Detected keyboard layout: {}", keyboard_layout.full_name());
            } else {
                info!("Could not detect keyboard layout");
            }
        }

        // Note: size-allocate signal handling is implemented in CompositorWidget
        // The widget's size_allocate method override handles actual resize events
        // However, we also need to handle window resize events at the window level
        // because the widget's size_allocate may not be triggered in all cases

        // Add rotation widget (or compositor widget if rotation disabled) to window
        // window.set_child(Some(&rotation_widget));

        let pie_menu_widget = PieMenuOverlayWidget::new(Some(&rotation_widget));
        pie_menu_widget.set_message_sender(pie_menu_sender);
        pie_menu_widget.set_rotation(command_line_arguments_for_closure.rotation);

        // Synchronize rotation between RotationWidget and PieMenuOverlayWidget
        let rotation_widget_clone = rotation_widget.clone();
        let pie_menu_widget_clone = pie_menu_widget.clone();
        let last_rotation = Rc::new(RefCell::new(command_line_arguments_for_closure.rotation));

        let tick_callback = move || {
            if let Some(rotation_widget) = rotation_widget_clone.downcast_ref::<RotationWidget>() {
                let current_rotation = rotation_widget.rotation();
                let mut last_rotation_ref = last_rotation.borrow_mut();
                if (current_rotation - *last_rotation_ref).abs() > 0.1 {
                    *last_rotation_ref = current_rotation;
                    pie_menu_widget_clone.set_rotation(current_rotation);
                }
            }
            glib::ControlFlow::Continue
        };

        glib::timeout_add_local(Duration::from_millis(16), tick_callback);

        window.set_child(Some(&pie_menu_widget));

        // Configure widget to expand and fill the window
        // rotation_widget.set_hexpand(true);
        // rotation_widget.set_vexpand(true);
        // rotation_widget.set_visible(true);

        // TODO: Phase 3 - Implement keyboard event forwarding
        // Add key controller to window to receive keyboard events
        let compositor_widget_clone_press = compositor_widget.clone();
        let compositor_widget_clone_release = compositor_widget.clone();
        let key_controller = gtk4::EventControllerKey::new();
        key_controller.connect_key_pressed(move |_controller, keyval, keycode, _state| {
            debug!("Key pressed in GTK window: keyval={:?} keycode={}", keyval, keycode);
            let _ = compositor_widget_clone_press.handle_key_press(keyval, keycode);
            glib::Propagation::Proceed
        });
        key_controller.connect_key_released(move |_controller, keyval, keycode, _state| {
            debug!("Key released in GTK window: keyval={:?} keycode={}", keyval, keycode);
            let _ = compositor_widget_clone_release.handle_key_release(keyval, keycode);
        });
        window.add_controller(key_controller);

        // // Add motion controller to window to receive mouse motion events
        // let compositor_widget_clone_motion = compositor_widget.clone();
        // let motion_controller = gtk4::EventControllerMotion::new();
        // motion_controller.set_propagation_phase(gtk4::PropagationPhase::);
        // motion_controller.connect_motion(move |_controller, x, y| {
        //     info!("main.rs Mouse motion event received in GTK window: x={}, y={}", x, y);
        //     compositor_widget_clone_motion.handle_mouse_motion(x, y);
        // });
        // window.add_controller(motion_controller);

        // Show the window
        window.set_opacity(0.0);
        window.present();

        // Sync header bar title and window title with Wayland client window title if no custom title is set
        if command_line_arguments_for_closure.title.is_none() {
            let compositor_widget_clone = compositor_widget.clone();
            let window_clone = window.clone();
            let header_bar_clone = header_bar.clone();

            // Periodically check for title changes from the Wayland client window

            glib::timeout_add_local(Duration::from_millis(100), move || {
                let Ok(compositor) = compositor_widget_clone.compositor() else {
                    return ControlFlow::Continue;
                };
                let Ok(compositor_guard) = compositor.lock() else {
                    return ControlFlow::Continue;
                };
                if let Some(client_title) = compositor_guard.get_active_window_title() {
                    compositor_widget_clone.set_header_bar_title(&client_title);
                    window_clone.set_title(Some(&client_title));
                    title_label.set_label(&client_title);
                }
                ControlFlow::Continue
            });
        }

        // Launch the specified application if command arguments are provided
        // Launch after compositor is initialized so the socket is ready
        if !command_line_arguments_for_closure.command_arguments.is_empty() {
            let socket_clone = socket_name_for_env.clone();
            let command_arguments_clone = command_line_arguments_for_closure.command_arguments.clone();
            let shell_clone = command_line_arguments_for_closure.shell;
            let wayland_debug_clone = command_line_arguments_for_closure.wayland_debug;
            let gsk_renderer_gl_clone = command_line_arguments_for_closure.gsk_renderer_gl;
            let program_name = command_arguments_clone[0].to_string_lossy().to_string();

            // Create channel for error communication
            let (error_sender, error_receiver) = std::sync::mpsc::channel::<String>();

            // let handle = tokio::spawn(async move {
            //     debug!("Launching child application in background thread");
            //     debug!("Setting WAYLAND_DISPLAY environment variable in child process: {}", socket_clone);
            // });
            thread::spawn(move || {
                debug!("Launching child application in background thread");
                debug!("Setting WAYLAND_DISPLAY environment variable in child process: {}", socket_clone);
                // Set environment variables in the current thread before launching
                unsafe {
                    std::env::set_var("WAYLAND_DISPLAY", &socket_clone);
                    std::env::set_var("GDK_BACKEND", "wayland");
                    if wayland_debug_clone {
                        std::env::set_var("WAYLAND_DEBUG", "1");
                    }
                    if gsk_renderer_gl_clone {
                        std::env::set_var("GSK_RENDERER", "gl");
                    }
                }
                if let Err(e) = launch_application(&socket_clone, &command_arguments_clone, shell_clone, wayland_debug_clone, gsk_renderer_gl_clone) {
                    error!("Failed to launch child application: {}", e);
                    // Send error through channel
                    let _ = error_sender.send(program_name);
                }
            });

            // Receive error in main thread and update UI
            let compositor_widget_clone = compositor_widget.clone();
            glib::idle_add_local(move || {
                if let Ok(program_name) = error_receiver.try_recv() {
                    compositor_widget_clone.set_application_error(Some(program_name));
                }
                ControlFlow::Break
            });
        } else {
            // No application specified, set error state
            compositor_widget.set_application_not_specified();
        }
    });

    // Run the application without command line arguments.
    // Arguments are already parsed by the CLI argument parser
    debug!("About to run GTK application");
    app.run_with_args::<&str>(&[]);
    debug!("GTK application finished");

    Ok(())
}

/// Launch an application with the Wayland socket set in environment
///
/// # Arguments
///
/// * `socket` - Path to the Wayland socket
/// * `command_arguments` - Command and arguments to execute
/// * `shell` - Whether to run in a shell
/// * `wayland_debug` - Whether to enable WAYLAND_DEBUG=1
/// * `gsk_renderer_gl` - Whether to enable GSK_RENDERER=gl
fn launch_application(
    socket: &str,
    command_arguments: &[std::ffi::OsString],
    shell: bool,
    wayland_debug: bool,
    gsk_renderer_gl: bool,
) -> Result<(), Box<dyn Error>> {
    debug!("Launching application with arguments: {:?}", command_arguments);
    debug!("Setting WAYLAND_DISPLAY to: {}", socket);

    let mut command = if shell {
        // Run in shell
        let mut cmd = Command::new("sh");
        cmd.arg("-c");
        cmd.arg(command_arguments.join(OsStr::new(" ")).to_string_lossy().to_string());
        cmd
    } else {
        // Run directly
        let executable_name = command_arguments[0].to_string_lossy().to_string();

        // Resolve the absolute path of the executable
        let resolved_path = match which(&executable_name) {
            Ok(path) => {
                debug!("Resolved executable '{}' to: {}", executable_name, path.display());
                path.to_string_lossy().to_string()
            }
            Err(_) => {
                // Return error if executable not found in PATH
                return Err(format!("Executable '{}' not found in PATH", executable_name).into());
            }
        };

        let mut cmd = Command::new(&resolved_path);
        if command_arguments.len() > 1 {
            cmd.args(&command_arguments[1..]);
        }
        cmd
    };

    // Set Wayland environment variables
    command.env("WAYLAND_DISPLAY", socket);
    debug!("WAYLAND_DISPLAY set to: {}", socket);

    // Enable Wayland protocol debugging for child process if requested
    if wayland_debug {
        command.env("WAYLAND_DEBUG", "1");
    }

    // Force OpenGL renderer for GTK applications if requested via CLI argument
    // Otherwise, inherit GSK_RENDERER from parent process (which is set to "gl" in main())
    if gsk_renderer_gl {
        command.env("GSK_RENDERER", "gl");
    }

    // Force Wayland backend for GTK applications
    command.env("GDK_BACKEND", "wayland");

    if let Ok(xdg_runtime_dir) = std::env::var("XDG_RUNTIME_DIR") {
        command.env("XDG_RUNTIME_DIR", xdg_runtime_dir);
    }

    if let Ok(session_bus) = std::env::var("DBUS_SESSION_BUS_ADDRESS") {
        command.env("DBUS_SESSION_BUS_ADDRESS", session_bus);
    }

    // Log environment variables for debugging
    if wayland_debug {
        debug!("Child process environment variables:");
        debug!("  WAYLAND_DISPLAY: {}", socket);
        debug!("  GDK_BACKEND: wayland");
        if gsk_renderer_gl {
            debug!("  GSK_RENDERER: gl");
        }

        // Print environment variables for child process to verify
        command.env("PRINT_ENV", "1");
    }

    for (key, value) in command.get_envs() {
        info!("{}={}", key.to_string_lossy().to_string(), value.map(|v| v.to_string_lossy().to_string()).unwrap_or_default());
    }

    // Capture stdout and stderr for logging
    command.stdin(Stdio::inherit());
    command.stdout(Stdio::piped());
    command.stderr(Stdio::piped());

    debug!("WAYLAND_DISPLAY and GDK_BACKEND=wayland set for child process");

    // Spawn the process in background
    let mut child = command.spawn()?;
    let pid = child.id();
    debug!("Application launched with PID: {}", pid);
    debug!("Application should connect to Wayland socket: {}", socket);

    // Read stdout and stderr in separate threads and log to tracing
    if let Some(stdout) = child.stdout.take() {
        let reader = std::io::BufReader::new(stdout);
        thread::spawn(move || {
            for line in reader.lines().flatten() {
                info!("[CHILD STDOUT] {}", line);
            }
        });
    }

    if let Some(stderr) = child.stderr.take() {
        let reader = std::io::BufReader::new(stderr);
        thread::spawn(move || {
            for line in reader.lines() {
                if let Ok(line) = line {
                    info!("[CHILD STDERR] {}", line);
                }
            }
        });
    }

    // Note: We don't wait for the child process to complete
    // The child process will continue running in the background
    // The threads will continue logging output until the process completes

    Ok(())
}
