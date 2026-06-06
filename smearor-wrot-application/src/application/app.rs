use crate::KeyboardLayout;
use crate::ScreenshotManager;
use crate::SocketBuilder;
use crate::SocketManager;
use crate::application::config::CompositorApplicationConfig;
use crate::set_program_icon;
use crate::show_settings_dialog;
use gtk4::ApplicationWindow;
use gtk4::Box as GtkBox;
use gtk4::Button;
use gtk4::CssProvider;
use gtk4::EventControllerKey;
use gtk4::HeaderBar;
use gtk4::Label;
use gtk4::Orientation;
use gtk4::Separator;
use gtk4::Widget;
use gtk4::gdk::Display;
use gtk4::gdk::Toplevel;
use gtk4::gio::ApplicationFlags;
use gtk4::glib;
use gtk4::glib::ControlFlow;
use gtk4::prelude::*;
use gtk4_layer_shell::LayerShell;
use smearor_wrot_compositor::DoubleBuffer;
use smearor_wrot_compositor::background::subsurface::SubsurfaceBackground;
use smearor_wrot_compositor::background::toplevel::ToplevelBackground;
use smearor_wrot_compositor::color_mask::mask::ColorMask;
use smearor_wrot_compositor::color_mask::subsurface::SubSurfaceColorMask;
use smearor_wrot_compositor::color_mask::toplevel::TopLevelColorMask;
use smearor_wrot_compositor::dma::buffer::DmaBuffer;
use smearor_wrot_compositor::frame::limit::FrameLimiter;
use smearor_wrot_compositor::margin::handler::MarginHandler;
use smearor_wrot_compositor::message::compositor_message::CompositorMessage;
use smearor_wrot_compositor::message::sender::CompositorMessageSender;
use smearor_wrot_compositor::windows::decoration::ClientDecorationAware;
use smearor_wrot_compositor::windows::title::WindowTitle;
use smearor_wrot_compositor_widget::CompositorWidget;
use smearor_wrot_compositor_widget::CompositorWidgetConfig;
use smearor_wrot_compositor_widget::clipboard::sync_manager::SyncManager;
use smearor_wrot_compositor_widget::event_handler::keyboard::KeyboardInputEventHandler;
use smearor_wrot_compositor_widget::widget::compositor::handler::CompositorHandler;
use smearor_wrot_compositor_widget::widget::config::handler::ConfigHandler;
use smearor_wrot_compositor_widget::widget::debug_overlay::config::DebugOverlayConfig;
use smearor_wrot_compositor_widget::widget::debug_overlay::handler::DebugOverlayHandler;
use smearor_wrot_compositor_widget::widget::resize::handler::ResizeHandler;
use smearor_wrot_compositor_widget::widget::shutdown::handler::ShutdownHandler;
use smearor_wrot_compositor_widget::widget::socket::handler::SocketHandler;
use smearor_wrot_compositor_widget::widget::window_state::handler::WindowStateHandler;
use smearor_wrot_model::Socket;
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
use std::ffi::OsString;
use std::io::BufRead;
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
use typed_builder::TypedBuilder;
use which::which;

#[derive(Debug, Clone, TypedBuilder)]
pub struct CompositorApplication {
    pub config: CompositorApplicationConfig,
}

impl CompositorApplication {
    pub fn run(self) -> Result<(), Box<dyn Error>> {
        let app_config = self.config.clone();
        let application_id = app_config.id.clone().unwrap_or_else(|| {
            format!("io.smearor.wrot.p{}", std::process::id())
        });

        debug!("Starting smearor-wrot GTK4 application with application id {application_id}");

        if let Some(override_wayland_display) = &app_config.override_wayland_display {
            debug!("Overriding WAYLAND_DISPLAY to: {override_wayland_display}");
            unsafe {
                std::env::set_var("WAYLAND_DISPLAY", override_wayland_display);
            }
        }

        let app = gtk4::Application::builder()
            .application_id(&application_id)
            .flags(ApplicationFlags::CAN_OVERRIDE_APP_ID | ApplicationFlags::NON_UNIQUE)
            .build();

        let socket = SocketBuilder::build(&app_config.socket)?;
        let socket_manager = Arc::new(SocketManager::new(socket));

        let application_ref = Arc::new(self);

        app.connect_activate(move |app| {
            debug!("Application activate callback called");

            // 1. Create the main window
            let window = application_ref.build_window(app);

            // 2. Set program icon
            set_program_icon(&window);

            // 3. Setup communication channels
            let (compositor_message_sender, compositor_message_receiver) = mpsc::channel::<CompositorMessage>();
            let (pie_menu_sender, pie_menu_receiver) = mpsc::channel::<PieMenuMessage>();

            // 4. Create the compositor widget
            let compositor_widget = application_ref.setup_compositor_widget(&socket_manager);

            // 5. Wrap inside rotation widget
            let rotation_widget = application_ref.setup_rotation_widget(&compositor_widget);

            // 6. Connect window actions & buttons (build HeaderBar)
            let sync_manager = Arc::new(Mutex::new(None));
            let header_bar = application_ref.setup_header_bar(
                app,
                &window,
                &compositor_widget,
                &rotation_widget,
                &sync_manager,
            );
            window.set_titlebar(Some(&header_bar));

            // 7. Setup screenshot manager
            let screenshot_manager = Arc::new(ScreenshotManager::new(app.clone(), compositor_widget.clone()));

            // 8. Setup pie menu overlay
            let pie_menu_widget = application_ref.setup_pie_menu_overlay(&rotation_widget, pie_menu_sender);

            // 9. Sync rotation between RotationWidget and PieMenuOverlayWidget
            application_ref.setup_rotation_sync(&rotation_widget, &pie_menu_widget);

            window.set_child(Some(&pie_menu_widget));

            // 10. Configure and initialize the compositor core
            application_ref.initialize_compositor_core(
                &compositor_widget,
                compositor_message_sender,
                &sync_manager,
            );

            // 11. Connect event controllers (Keyboard, etc.)
            application_ref.setup_event_forwarding(&window, &compositor_widget);

            // 12. Run the message checking loops
            application_ref.setup_message_loops(
                app,
                &window,
                &compositor_widget,
                &rotation_widget,
                &pie_menu_widget,
                compositor_message_receiver,
                pie_menu_receiver,
                &screenshot_manager,
                &sync_manager,
            );

            // 13. Synchronize window/header title
            application_ref.setup_title_sync(&window, &compositor_widget, &header_bar);

            // 14. Present the window (before launching child process, as in original code)
            window.set_opacity(0.0);
            window.present();

            // 15. Launch child process if specified
            application_ref.launch_child_process(&socket_manager.socket());
        });

        app.run_with_args::<&str>(&[]);

        Ok(())
    }

    fn build_window(&self, app: &gtk4::Application) -> ApplicationWindow {
        let window = ApplicationWindow::builder()
            .application(app)
            .title(self.config.title.as_deref().unwrap_or("Smearor Compositor"))
            .startup_id("org.gnome.Chess")
            .default_width(self.config.width)
            .default_height(self.config.height)
            .resizable(self.config.resizable)
            .decorated(self.config.decorated)
            .build();

        if let Some(layer) = self.config.layer.as_ref() {
            if gtk4_layer_shell::is_supported() {
                window.init_layer_shell();
                let gtk_layer = gtk4_layer_shell::Layer::from(*layer);
                window.set_layer(gtk_layer);
                debug!("Layer shell initialized with layer: {:?}", layer);

                if let Some(namespace) = self.config.namespace.as_ref() {
                    window.set_namespace(Some(namespace.as_str()));
                    debug!("Layer shell namespace set to: {}", namespace);
                }
            } else {
                debug!("Layer shell protocol not supported, falling back to regular window");
            }
        }

        if let Some(min_width) = self.config.min_width {
            if let Some(min_height) = self.config.min_height {
                window.set_size_request(min_width, min_height);
            } else {
                window.set_size_request(min_width, window.default_height());
            }
        } else if let Some(min_height) = self.config.min_height {
            window.set_size_request(window.default_width(), min_height);
        }

        if let Some(max_width) = self.config.max_width {
            if let Some(max_height) = self.config.max_height {
                window.set_size_request(max_width, max_height);
            } else {
                window.set_size_request(max_width, window.default_height());
            }
        } else if let Some(max_height) = self.config.max_height {
            window.set_size_request(window.default_width(), max_height);
        }

        if self.config.fullscreen {
            window.set_fullscreened(true);
        }

        if self.config.maximized {
            window.maximize();
        }

        let provider = CssProvider::new();
        provider.load_from_data("window { background-color: transparent; } ");
        if let Some(display) = Display::default() {
            gtk4::style_context_add_provider_for_display(
                &display,
                &provider,
                gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );

            if let Some(keyboard_layout) = KeyboardLayout::detect() {
                info!("Detected keyboard layout: {}", keyboard_layout.full_name());
            } else {
                info!("Could not detect keyboard layout");
            }
        }

        window
    }

    fn setup_compositor_widget(&self, socket_manager: &Arc<SocketManager>) -> CompositorWidget {
        let compositor_widget = CompositorWidget::new();
        compositor_widget.set_auto_resize_handling(true);

        let keyboard_layout = if self.config.keyboard_layout.is_some() || self.config.keyboard_variant.is_some() {
            info!("Using CLI keyboard layout parameters");
            Some(KeyboardLayout::new(
                self.config.keyboard_layout.clone().unwrap_or_default(),
                self.config.keyboard_variant.clone(),
            ))
        } else {
            info!("Detecting keyboard layout automatically");
            KeyboardLayout::detect()
        };

        let config = CompositorWidgetConfig {
            show_decorations: self.config.decorated,
            fullscreen: self.config.fullscreen,
            initial_width: self.config.width,
            initial_height: self.config.height,
            title: self.config.title.clone(),
            dma_buf_enabled: !self.config.disable_dma_buf,
            min_width: self.config.min_width.unwrap_or(100),
            min_height: self.config.min_height.unwrap_or(100),
            auto_color_mask: self.config.auto_color_mask,
            auto_subsurface_color_mask: self.config.auto_subsurface_color_mask,
            color_mask_tolerance: self.config.color_mask_tolerance,
            resizable: self.config.resizable,
            disable_client_decorations: self.config.disable_client_decorations,
            color_mask_shader: self.config.color_mask_shader,
            animations: !self.config.disable_animations,
            max_fps: self.config.max_fps,
            keyboard_layout: keyboard_layout.as_ref().map(|layout| layout.layout.clone()),
            keyboard_variant: keyboard_layout.as_ref().and_then(|layout| layout.variant.clone()),
            ..Default::default()
        };
        compositor_widget.set_config(config);

        let debug_overlay_config = DebugOverlayConfig {
            debug_pointer: self.config.debug_pointer,
            debug_touch: self.config.debug_touch,
        };
        compositor_widget.set_debug_overlay_config(debug_overlay_config);

        let socket = socket_manager.socket();
        debug!("Set socket path to: {socket}");
        let _ = compositor_widget.initialize_socket(socket);
        compositor_widget.initialize_compositor();

        let _ = compositor_widget.apply_config_to_compositor();

        compositor_widget
    }

    fn setup_rotation_widget(&self, compositor_widget: &CompositorWidget) -> Widget {
        if self.config.disable_rotation {
            compositor_widget.clone().upcast()
        } else {
            let rotation = SmearorRotation::Deg(self.config.rotation);
            let rotation_widget = RotationWidget::new(rotation);
            rotation_widget.set_child(Some(compositor_widget));
            rotation_widget.set_animation_speed(self.config.animation_speed);
            rotation_widget.set_animations_enabled(!self.config.disable_animations);
            rotation_widget.set_animation_overshoot(self.config.animation_overshoot);

            let rotation_widget_clone = rotation_widget.clone();
            compositor_widget.set_touch_transform_callback(move |_sequence, position| {
                if let Some(rotation_widget) = rotation_widget_clone.downcast_ref::<RotationWidget>() {
                    rotation_widget.input_transform(position.x, position.y).into()
                } else {
                    position
                }
            });

            let rotation_widget_clone = rotation_widget.clone();
            compositor_widget.set_pointer_transform_callback(move |position| {
                if let Some(rotation_widget) = rotation_widget_clone.downcast_ref::<RotationWidget>() {
                    rotation_widget.input_transform(position.x, position.y).into()
                } else {
                    position
                }
            });

            rotation_widget.upcast()
        }
    }

    fn setup_header_bar(
        &self,
        app: &gtk4::Application,
        window: &ApplicationWindow,
        compositor_widget: &CompositorWidget,
        rotation_widget: &Widget,
        sync_manager: &Arc<Mutex<Option<Arc<SyncManager>>>>,
    ) -> HeaderBar {
        let header_bar = HeaderBar::builder().show_title_buttons(true).build();

        let title = self.config.title.as_deref().unwrap_or("Smearor Compositor");
        let title_label = Label::builder().label(title).build();
        header_bar.set_title_widget(Some(&title_label));

        let rotate_counter_clockwise_button = Button::builder()
            .icon_name("object-rotate-left-symbolic")
            .tooltip_text("Rotate Counter Clockwise")
            .css_classes(["large-button"])
            .build();

        let rotate_clockwise_button = Button::builder()
            .icon_name("object-rotate-right-symbolic")
            .tooltip_text("Rotate Clockwise")
            .css_classes(["large-button"])
            .build();

        let reset_rotation_button = Button::builder()
            .icon_name("view-refresh-symbolic")
            .tooltip_text("Reset Rotation")
            .css_classes(["large-button"])
            .build();

        let paste_button = Button::builder()
            .icon_name("edit-paste-symbolic")
            .tooltip_text("Paste from Host System to Compositor")
            .css_classes(["large-button"])
            .build();

        let copy_button = Button::builder()
            .icon_name("edit-copy-symbolic")
            .tooltip_text("Copy from Compositor to Host System")
            .css_classes(["large-button"])
            .build();

        let rotate_box = GtkBox::builder()
            .orientation(Orientation::Horizontal)
            .spacing(4)
            .build();
        rotate_box.append(&rotate_counter_clockwise_button);
        rotate_box.append(&rotate_clockwise_button);
        rotate_box.append(&reset_rotation_button);

        let separator = Separator::builder()
            .orientation(Orientation::Horizontal)
            .margin_start(8)
            .margin_end(8)
            .build();

        let clipboard_box = GtkBox::builder()
            .orientation(Orientation::Horizontal)
            .spacing(4)
            .build();
        clipboard_box.append(&paste_button);
        clipboard_box.append(&copy_button);

        let settings_separator = Separator::builder()
            .orientation(Orientation::Horizontal)
            .margin_start(8)
            .margin_end(8)
            .build();

        let settings_button = Button::builder()
            .icon_name("preferences-system-symbolic")
            .tooltip_text("Settings")
            .css_classes(["large-button"])
            .build();

        let screenshot_button = Button::builder()
            .icon_name("camera-photo-symbolic")
            .tooltip_text("Save screenshot")
            .css_classes(["large-button"])
            .build();

        header_bar.pack_start(&rotate_box);
        header_bar.pack_start(&separator);
        header_bar.pack_start(&clipboard_box);
        header_bar.pack_start(&settings_separator);
        header_bar.pack_start(&settings_button);
        header_bar.pack_start(&screenshot_button);

        if !self.config.disable_rotation {
            let initial_rotation = self.config.rotation;
            let rotation_widget_clone = rotation_widget.clone();
            let compositor_widget_clone = compositor_widget.clone();
            rotate_counter_clockwise_button.connect_clicked(move |_| {
                if let Some(rotation_widget) = rotation_widget_clone.downcast_ref::<RotationWidget>() {
                    let current_rotation = rotation_widget.rotation();
                    let rotation_deg = (current_rotation % 360.0).abs();

                    let new_rotation = if rotation_deg > 360.0 || (rotation_deg > 0.0 && rotation_deg <= 90.0) {
                        0.0
                    } else if rotation_deg > 90.0 && rotation_deg <= 180.0 {
                        90.0
                    } else if rotation_deg > 180.0 && rotation_deg <= 270.0 {
                        180.0
                    } else {
                        270.0
                    };

                    rotation_widget.set_rotation_with_animation(new_rotation);

                    let last_width = compositor_widget_clone.width();
                    let last_height = compositor_widget_clone.height();
                    let new_size = if (new_rotation - 90.0).abs() < 1.0 || (new_rotation - 270.0).abs() < 1.0 {
                        Size::new(last_height, last_width)
                    } else {
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

                    let new_rotation = if rotation_deg >= 0.0 && rotation_deg < 90.0 {
                        90.0
                    } else if rotation_deg >= 90.0 && rotation_deg < 180.0 {
                        180.0
                    } else if rotation_deg >= 180.0 && rotation_deg < 270.0 {
                        270.0
                    } else {
                        0.0
                    };

                    rotation_widget.set_rotation_with_animation(new_rotation);

                    let last_width = compositor_widget_clone.width();
                    let last_height = compositor_widget_clone.height();
                    let new_size = if (new_rotation - 90.0).abs() < 1.0 || (new_rotation - 270.0).abs() < 1.0 {
                        Size::new(last_height, last_width)
                    } else {
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

                    let last_width = compositor_widget_clone.width();
                    let last_height = compositor_widget_clone.height();
                    let rotation_deg = (initial_rotation % 360.0).abs();
                    let new_size = if (rotation_deg - 90.0).abs() < 1.0 || (rotation_deg - 270.0).abs() < 1.0 {
                        Size::new(last_height, last_width)
                    } else {
                        Size::new(last_width, last_height)
                    };
                    compositor_widget_clone.handle_resize(new_size);
                }
            });
        } else {
            rotate_counter_clockwise_button.set_sensitive(false);
            rotate_clockwise_button.set_sensitive(false);
            reset_rotation_button.set_sensitive(false);
        }

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

        let app_clone = app.clone();
        let compositor_widget_clone = compositor_widget.clone();
        screenshot_button.connect_clicked(move |_| {
            let screenshot_manager = ScreenshotManager::new(app_clone.clone(), compositor_widget_clone.clone());
            let _ = screenshot_manager.screenshot();
        });

        let compositor_widget_for_settings = compositor_widget.clone();
        let window_for_settings = window.clone();
        let rotation_widget_for_settings = rotation_widget.clone();
        let disable_dma_buf = self.config.disable_dma_buf;
        settings_button.connect_clicked(move |_| {
            show_settings_dialog(
                (&window_for_settings).as_ref(),
                &compositor_widget_for_settings,
                &rotation_widget_for_settings,
                disable_dma_buf,
            );
        });

        header_bar
    }

    fn setup_pie_menu_overlay(
        &self,
        rotation_widget: &Widget,
        pie_menu_sender: mpsc::Sender<PieMenuMessage>,
    ) -> PieMenuOverlayWidget {
        let pie_menu_widget = PieMenuOverlayWidget::new(Some(rotation_widget));
        pie_menu_widget.set_message_sender(pie_menu_sender);
        pie_menu_widget.set_rotation(self.config.rotation);
        pie_menu_widget
    }

    fn setup_rotation_sync(&self, rotation_widget: &Widget, pie_menu_widget: &PieMenuOverlayWidget) {
        let rotation_widget_clone = rotation_widget.clone();
        let pie_menu_widget_clone = pie_menu_widget.clone();
        let last_rotation = Rc::new(RefCell::new(self.config.rotation));

        let tick_callback = move || {
            if let Some(rotation_widget) = rotation_widget_clone.downcast_ref::<RotationWidget>() {
                let current_rotation = rotation_widget.rotation();
                let mut last_rotation_ref = last_rotation.borrow_mut();
                if (current_rotation - *last_rotation_ref).abs() > 0.1 {
                    *last_rotation_ref = current_rotation;
                    pie_menu_widget_clone.set_rotation(current_rotation);
                }
            }
            ControlFlow::Continue
        };

        glib::timeout_add_local(Duration::from_millis(16), tick_callback);
    }

    fn initialize_compositor_core(
        &self,
        compositor_widget: &CompositorWidget,
        compositor_message_sender: mpsc::Sender<CompositorMessage>,
        sync_manager: &Arc<Mutex<Option<Arc<SyncManager>>>>,
    ) {
        if let Ok(compositor) = compositor_widget.compositor() {
            if let Ok(guard) = compositor.lock() {
                guard.set_message_sender(compositor_message_sender);
                guard.set_double_buffer_enabled(self.config.double_buffer);
                guard.set_dma_buf_enabled(!self.config.disable_dma_buf);
                guard.set_client_decorations_enabled(!self.config.disable_client_decorations);

                let (margin_left, margin_right, margin_top, margin_bottom) = if let Some(margin) = self.config.margin {
                    (margin, margin, margin, margin)
                } else {
                    (
                        self.config.margin_left,
                        self.config.margin_right,
                        self.config.margin_top,
                        self.config.margin_bottom,
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
                guard.set_dialog_margin(self.config.dialog_margin);
                guard.set_opacity(self.config.opacity);
                guard.set_max_fps(self.config.max_fps);
                guard.set_color_mask_tolerance(self.config.color_mask_tolerance);

                if let Some(hex_color) = &self.config.background_color {
                    if let Ok(rgba_color) = RgbaColor::parse_hex_with_optional_alpha(hex_color) {
                        let _ = guard.set_background_color(rgba_color);
                    } else {
                        error!("Invalid hex color format: {}", hex_color);
                    }
                }

                if let Some(hex_color) = &self.config.subsurface_background_color {
                    if let Ok(rgba_color) = RgbaColor::parse_hex_with_optional_alpha(hex_color) {
                        let _ = guard.set_subsurface_background_color(rgba_color);
                    } else {
                        error!("Invalid hex color format for subsurface background color: {}", hex_color);
                    }
                }

                if let Some(hex_color) = &self.config.color_mask {
                    if let Ok(rgba_color) = RgbaColor::parse_hex_with_optional_alpha(hex_color) {
                        let _ = guard.set_color_mask(ColorMask::new(rgba_color.color, self.config.color_mask_tolerance));
                        debug!("Manual color mask set to {} with tolerance {}", hex_color, self.config.color_mask_tolerance);
                        guard.set_dma_buf_enabled(false);
                        debug!("DMA-BUF disabled because color mask is set");
                    } else {
                        error!("Invalid hex color format for color mask: {}", hex_color);
                    }
                }

                if self.config.auto_color_mask {
                    debug!("Auto color mask detection enabled - will detect dominant color from first frame");
                    guard.set_auto_color_mask(true);
                }

                if let Some(hex_color) = &self.config.subsurface_color_mask {
                    if let Ok(rgba_color) = RgbaColor::parse_hex_with_optional_alpha(hex_color) {
                        let _ = guard.set_subsurface_color_mask(ColorMask::new(rgba_color.color, self.config.color_mask_tolerance));
                        debug!("Manual subsurface color mask set to {} with tolerance {}", hex_color, self.config.color_mask_tolerance);
                    } else {
                        error!("Invalid hex color format for subsurface color mask: {}", hex_color);
                    }
                }

                if self.config.auto_subsurface_color_mask {
                    debug!("Auto subsurface color mask detection enabled - will detect dominant color from subsurfaces");
                    guard.set_auto_subsurface_color_mask(true);
                    guard.set_dma_buf_enabled(false);
                    debug!("DMA-BUF disabled because auto subsurface color mask is enabled");
                }
            }

            match SyncManager::new_with_widget(compositor_widget.clone()) {
                Ok(manager) => {
                    let manager = Arc::new(manager);
                    if let Err(e) = manager.start_polling() {
                        error!("Failed to start clipboard polling: {}", e);
                    } else if let Ok(mut guard) = sync_manager.lock() {
                        *guard = Some(manager);
                    }
                }
                Err(e) => {
                    error!("Failed to create sync manager: {}", e);
                }
            }
        }
    }

    fn setup_event_forwarding(&self, window: &ApplicationWindow, compositor_widget: &CompositorWidget) {
        let compositor_widget_clone_press = compositor_widget.clone();
        let compositor_widget_clone_release = compositor_widget.clone();
        let key_controller = EventControllerKey::new();
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
    }

    fn setup_title_sync(&self, window: &ApplicationWindow, compositor_widget: &CompositorWidget, header_bar: &HeaderBar) {
        if self.config.title.is_none() {
            let compositor_widget_clone = compositor_widget.clone();
            let window_clone = window.clone();
            let header_bar_clone = header_bar.clone();

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
                    if let Some(title_widget) = header_bar_clone.title_widget() {
                        if let Some(label) = title_widget.downcast_ref::<Label>() {
                            label.set_label(&client_title);
                        }
                    }
                }
                ControlFlow::Continue
            });
        }
    }

    fn setup_message_loops(
        &self,
        app: &gtk4::Application,
        window: &ApplicationWindow,
        compositor_widget: &CompositorWidget,
        rotation_widget: &Widget,
        pie_menu_widget: &PieMenuOverlayWidget,
        compositor_message_receiver: mpsc::Receiver<CompositorMessage>,
        pie_menu_receiver: mpsc::Receiver<PieMenuMessage>,
        screenshot_manager: &Arc<ScreenshotManager>,
        sync_manager: &Arc<Mutex<Option<Arc<SyncManager>>>>,
    ) {
        let app_clone = app.clone();
        let window_clone = window.clone();
        let compositor_widget_clone = compositor_widget.clone();
        let rotation_widget_clone = rotation_widget.clone();
        let sync_manager_clone = sync_manager.clone();
        let screenshot_manager_clone = screenshot_manager.clone();
        let initial_window_opacity = self.config.window_opacity as f64;
        let config_disable_dma_buf = self.config.disable_dma_buf;

        glib::timeout_add_local(Duration::from_millis(16), move || {
            if let Ok(message) = compositor_message_receiver.try_recv() {
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

            let mut last_rotation_message = None;

            loop {
                match pie_menu_receiver.try_recv() {
                    Ok(message) => {
                        debug!("Received pie menu message: {:?}", message);
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
                                last_rotation_message = Some(rotation);
                            }
                            PieMenuMessage::Settings => {
                                info!("Received Settings message from pie menu");
                                show_settings_dialog(
                                    window_clone.as_ref(),
                                    &compositor_widget_clone,
                                    &rotation_widget_clone,
                                    config_disable_dma_buf,
                                );
                            }
                            PieMenuMessage::Screenshot => {
                                info!("Received Screenshot message from pie menu");
                                if compositor_widget_clone.downcast_ref::<CompositorWidget>().is_some() {
                                    let _ = screenshot_manager_clone.screenshot();
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

            if let Some(rotation) = last_rotation_message {
                debug!("Applying last rotation message: {} degrees", rotation);
                if let Some(rotation_widget) = rotation_widget_clone.downcast_ref::<RotationWidget>() {
                    rotation_widget.set_rotation(SmearorRotation::Deg(rotation));
                }
            }

            let _ = compositor_widget_clone.check_and_request_shutdown();
            ControlFlow::Continue
        });
    }

    fn launch_child_process(&self, socket: &Socket) {
        if !self.config.command_arguments.is_empty() {
            let socket_clone = socket.clone();
            let command_arguments_clone = self.config.command_arguments.clone();
            let shell_clone = self.config.shell;
            let wayland_debug_clone = self.config.wayland_debug;
            let gsk_renderer_gl_clone = self.config.gsk_renderer_gl;

            thread::spawn(move || {
                debug!("Launching child application in background thread");
                debug!("Setting WAYLAND_DISPLAY environment variable in child process: {}", socket_clone);
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
                if let Err(e) = launch_application(
                    &socket_clone,
                    &command_arguments_clone,
                    shell_clone,
                    wayland_debug_clone,
                    gsk_renderer_gl_clone,
                ) {
                    error!("Failed to launch child application: {}", e);
                }
            });
        }
    }
}

fn launch_application(
    socket: &Socket,
    command_arguments: &[OsString],
    shell: bool,
    wayland_debug: bool,
    gsk_renderer_gl: bool,
) -> Result<(), Box<dyn Error>> {
    debug!("Launching application with arguments: {:?}", command_arguments);
    debug!("Setting WAYLAND_DISPLAY to: {}", socket);

    let mut command = if shell {
        let mut cmd = Command::new("sh");
        cmd.arg("-c");
        cmd.arg(command_arguments.join(OsStr::new(" ")).to_string_lossy().to_string());
        cmd
    } else {
        let executable_name = command_arguments[0].to_string_lossy().to_string();

        let resolved_path = match which(&executable_name) {
            Ok(path) => {
                debug!("Resolved executable '{}' to: {}", executable_name, path.display());
                path.to_string_lossy().to_string()
            }
            Err(_) => {
                return Err(format!("Executable '{}' not found in PATH", executable_name).into());
            }
        };

        let mut cmd = Command::new(&resolved_path);
        if command_arguments.len() > 1 {
            cmd.args(&command_arguments[1..]);
        }
        cmd
    };

    command.env("WAYLAND_DISPLAY", socket);
    debug!("WAYLAND_DISPLAY set to: {}", socket);

    if wayland_debug {
        command.env("WAYLAND_DEBUG", "1");
    }

    if gsk_renderer_gl {
        command.env("GSK_RENDERER", "gl");
    }

    command.env("GDK_BACKEND", "wayland");

    if let Ok(xdg_runtime_dir) = std::env::var("XDG_RUNTIME_DIR") {
        command.env("XDG_RUNTIME_DIR", xdg_runtime_dir);
    }

    if let Ok(session_bus) = std::env::var("DBUS_SESSION_BUS_ADDRESS") {
        command.env("DBUS_SESSION_BUS_ADDRESS", session_bus);
    }

    if wayland_debug {
        debug!("Child process environment variables:");
        debug!("  WAYLAND_DISPLAY: {}", socket);
        debug!("  GDK_BACKEND: wayland");
        if gsk_renderer_gl {
            debug!("  GSK_RENDERER: gl");
        }
        command.env("PRINT_ENV", "1");
    }

    for (key, value) in command.get_envs() {
        info!("{}={}", key.to_string_lossy().to_string(), value.map(|v| v.to_string_lossy().to_string()).unwrap_or_default());
    }

    command.stdin(Stdio::inherit());
    command.stdout(Stdio::piped());
    command.stderr(Stdio::piped());

    debug!("WAYLAND_DISPLAY and GDK_BACKEND=wayland set for child process");

    let mut child = command.spawn()?;
    let pid = child.id();
    debug!("Application launched with PID: {}", pid);
    debug!("Application should connect to Wayland socket: {}", socket);

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

    Ok(())
}

#[derive(TypedBuilder)]
pub struct SmearorWrotApplication {
    socket_manager: Arc<SocketManager>,
}

impl SmearorWrotApplication {
    pub fn socket_manager(&self) -> Arc<SocketManager> {
        self.socket_manager.clone()
    }
}
