use gtk4::glib;
use gtk4::prelude::*;
use std::sync::Arc;
use tracing::debug;
use crate::settings::manager::SettingsManager;

pub struct SettingsDialog {
    dialog: gtk4::Window,
    settings_manager: Arc<SettingsManager>,
}

impl SettingsDialog {
    pub fn new(parent_window: &gtk4::ApplicationWindow, settings_manager: Arc<SettingsManager>) -> Self {
        debug!("Creating settings dialog");

        let dialog = gtk4::Window::builder()
            .title("Settings")
            .modal(true)
            .default_width(400)
            .default_height(300)
            .build();

        dialog.set_transient_for(Some(parent_window));

        Self {
            dialog,
            settings_manager,
        }
    }

    pub fn build(&self) -> gtk4::Window {
        let content_box = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Vertical)
            .spacing(12)
            .margin_start(12)
            .margin_end(12)
            .margin_top(12)
            .margin_bottom(12)
            .build();

        content_box.add_css_class("semi-transparent-background");
        let css_provider = gtk4::CssProvider::new();
        css_provider.load_from_data(".semi-transparent-background { background-color: rgba(0, 0, 0, 0.5); }");
        content_box
            .style_context()
            .add_provider(&css_provider, gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION);

        let debug_overlay_box = self.create_debug_overlay_toggles();
        content_box.append(&debug_overlay_box);

        let compositor_config_box = self.create_compositor_config_toggles();
        content_box.append(&compositor_config_box);

        let separator = gtk4::Separator::builder()
            .orientation(gtk4::Orientation::Horizontal)
            .build();
        content_box.append(&separator);

        let read_only_box = self.create_read_only_toggles();
        content_box.append(&read_only_box);

        let statistics_box = self.create_statistics_labels();
        content_box.append(&statistics_box);

        let close_button = gtk4::Button::builder()
            .label("Close")
            .build();

        let dialog_clone = self.dialog.clone();
        close_button.connect_clicked(move |_| {
            dialog_clone.close();
        });

        content_box.append(&close_button);

        self.dialog.set_child(Some(&content_box));

        self.dialog.clone()
    }

    fn create_debug_overlay_toggles(&self) -> gtk4::Box {
        let container = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Vertical)
            .spacing(12)
            .build();

        let debug_pointer_label = gtk4::Label::builder()
            .label("Debug Pointer")
            .build();
        let debug_pointer_toggle = gtk4::Switch::builder()
            .active(self.settings_manager.get_debug_pointer())
            .build();

        let debug_pointer_row = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Horizontal)
            .spacing(12)
            .build();
        debug_pointer_row.append(&debug_pointer_label);
        debug_pointer_row.append(&gtk4::Box::builder().hexpand(true).build());
        debug_pointer_row.append(&debug_pointer_toggle);

        let settings_manager_clone = self.settings_manager.clone();
        debug_pointer_toggle.connect_state_set(move |_, is_active| {
            settings_manager_clone.set_debug_pointer(is_active);
            glib::Propagation::Proceed
        });

        let debug_touch_label = gtk4::Label::builder()
            .label("Debug Touch")
            .build();
        let debug_touch_toggle = gtk4::Switch::builder()
            .active(self.settings_manager.get_debug_touch())
            .build();

        let debug_touch_row = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Horizontal)
            .spacing(12)
            .build();
        debug_touch_row.append(&debug_touch_label);
        debug_touch_row.append(&gtk4::Box::builder().hexpand(true).build());
        debug_touch_row.append(&debug_touch_toggle);

        let settings_manager_clone = self.settings_manager.clone();
        debug_touch_toggle.connect_state_set(move |_, is_active| {
            settings_manager_clone.set_debug_touch(is_active);
            glib::Propagation::Proceed
        });

        container.append(&debug_pointer_row);
        container.append(&debug_touch_row);

        container
    }

    fn create_compositor_config_toggles(&self) -> gtk4::Box {
        let container = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Vertical)
            .spacing(12)
            .build();

        let auto_color_mask_label = gtk4::Label::builder()
            .label("Auto Color Mask")
            .build();
        let auto_color_mask_toggle = gtk4::Switch::builder()
            .active(self.settings_manager.get_auto_color_mask())
            .build();

        let auto_color_mask_row = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Horizontal)
            .spacing(12)
            .build();
        auto_color_mask_row.append(&auto_color_mask_label);
        auto_color_mask_row.append(&gtk4::Box::builder().hexpand(true).build());
        auto_color_mask_row.append(&auto_color_mask_toggle);

        let settings_manager_clone = self.settings_manager.clone();
        auto_color_mask_toggle.connect_state_set(move |_, is_active| {
            settings_manager_clone.set_auto_color_mask(is_active);
            glib::Propagation::Proceed
        });

        let auto_subsurface_color_mask_label = gtk4::Label::builder()
            .label("Auto Subsurface Color Mask")
            .build();
        let auto_subsurface_color_mask_toggle = gtk4::Switch::builder()
            .active(self.settings_manager.get_auto_subsurface_color_mask())
            .build();

        let auto_subsurface_color_mask_row = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Horizontal)
            .spacing(12)
            .build();
        auto_subsurface_color_mask_row.append(&auto_subsurface_color_mask_label);
        auto_subsurface_color_mask_row.append(&gtk4::Box::builder().hexpand(true).build());
        auto_subsurface_color_mask_row.append(&auto_subsurface_color_mask_toggle);

        let settings_manager_clone = self.settings_manager.clone();
        auto_subsurface_color_mask_toggle.connect_state_set(move |_, is_active| {
            settings_manager_clone.set_auto_subsurface_color_mask(is_active);
            glib::Propagation::Proceed
        });

        let resizable_label = gtk4::Label::builder()
            .label("Resizable")
            .build();
        let resizable_toggle = gtk4::Switch::builder()
            .active(self.settings_manager.get_resizable())
            .build();

        let resizable_row = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Horizontal)
            .spacing(12)
            .build();
        resizable_row.append(&resizable_label);
        resizable_row.append(&gtk4::Box::builder().hexpand(true).build());
        resizable_row.append(&resizable_toggle);

        let settings_manager_clone = self.settings_manager.clone();
        resizable_toggle.connect_state_set(move |_, is_active| {
            settings_manager_clone.set_resizable(is_active);
            glib::Propagation::Proceed
        });

        let decorated_label = gtk4::Label::builder()
            .label("Decorated")
            .build();
        let decorated_toggle = gtk4::Switch::builder()
            .active(self.settings_manager.get_decorated())
            .build();

        let decorated_row = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Horizontal)
            .spacing(12)
            .build();
        decorated_row.append(&decorated_label);
        decorated_row.append(&gtk4::Box::builder().hexpand(true).build());
        decorated_row.append(&decorated_toggle);

        let settings_manager_clone = self.settings_manager.clone();
        decorated_toggle.connect_state_set(move |_, is_active| {
            settings_manager_clone.set_decorated(is_active);
            glib::Propagation::Proceed
        });

        let disable_client_decorations_label = gtk4::Label::builder()
            .label("Disable Client Decorations")
            .build();
        let disable_client_decorations_toggle = gtk4::Switch::builder()
            .active(self.settings_manager.get_disable_client_decorations())
            .build();

        let disable_client_decorations_row = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Horizontal)
            .spacing(12)
            .build();
        disable_client_decorations_row.append(&disable_client_decorations_label);
        disable_client_decorations_row.append(&gtk4::Box::builder().hexpand(true).build());
        disable_client_decorations_row.append(&disable_client_decorations_toggle);

        let settings_manager_clone = self.settings_manager.clone();
        disable_client_decorations_toggle.connect_state_set(move |_, is_active| {
            settings_manager_clone.set_disable_client_decorations(is_active);
            glib::Propagation::Proceed
        });

        let color_mask_shader_label = gtk4::Label::builder()
            .label("Color Mask Shader (GPU-based)")
            .build();
        let color_mask_shader_toggle = gtk4::Switch::builder()
            .active(self.settings_manager.get_color_mask_shader())
            .build();

        let color_mask_shader_row = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Horizontal)
            .spacing(12)
            .build();
        color_mask_shader_row.append(&color_mask_shader_label);
        color_mask_shader_row.append(&gtk4::Box::builder().hexpand(true).build());
        color_mask_shader_row.append(&color_mask_shader_toggle);

        let settings_manager_clone = self.settings_manager.clone();
        color_mask_shader_toggle.connect_state_set(move |_, is_active| {
            settings_manager_clone.set_color_mask_shader(is_active);
            glib::Propagation::Proceed
        });

        let animations_label = gtk4::Label::builder()
            .label("Animations")
            .build();
        let animations_toggle = gtk4::Switch::builder()
            .active(self.settings_manager.get_animations())
            .build();

        let animations_row = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Horizontal)
            .spacing(12)
            .build();
        animations_row.append(&animations_label);
        animations_row.append(&gtk4::Box::builder().hexpand(true).build());
        animations_row.append(&animations_toggle);

        let settings_manager_clone = self.settings_manager.clone();
        animations_toggle.connect_state_set(move |_, is_active| {
            settings_manager_clone.set_animations(is_active);
            glib::Propagation::Proceed
        });

        container.append(&auto_color_mask_row);
        container.append(&auto_subsurface_color_mask_row);
        container.append(&resizable_row);
        container.append(&decorated_row);
        container.append(&disable_client_decorations_row);
        container.append(&color_mask_shader_row);
        container.append(&animations_row);

        container
    }

    fn create_read_only_toggles(&self) -> gtk4::Box {
        let container = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Vertical)
            .spacing(12)
            .build();

        let disable_dma_buf_label = gtk4::Label::builder()
            .label("DMA-BUF Disabled (--disable-dma-buf)")
            .build();
        let disable_dma_buf_toggle = gtk4::Switch::builder()
            .active(self.settings_manager.get_disable_dma_buf())
            .sensitive(false)
            .build();

        let disable_dma_buf_row = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Horizontal)
            .spacing(12)
            .build();
        disable_dma_buf_row.append(&disable_dma_buf_label);
        disable_dma_buf_row.append(&gtk4::Box::builder().hexpand(true).build());
        disable_dma_buf_row.append(&disable_dma_buf_toggle);

        let render_path_label = gtk4::Label::builder()
            .label("Render Path (SHM/DMA-BUF)")
            .build();
        let render_path_toggle = gtk4::Switch::builder()
            .active(false)
            .sensitive(false)
            .build();

        let render_path_row = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Horizontal)
            .spacing(12)
            .build();
        render_path_row.append(&render_path_label);
        render_path_row.append(&gtk4::Box::builder().hexpand(true).build());
        render_path_row.append(&render_path_toggle);

        container.append(&disable_dma_buf_row);
        container.append(&render_path_row);

        container
    }

    fn create_statistics_labels(&self) -> gtk4::Box {
        let container = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Vertical)
            .spacing(12)
            .build();

        let dma_buf_count = self.settings_manager.get_dma_buf_render_count();
        let shm_count = self.settings_manager.get_shm_render_count();

        let current_render_path_is_dma_buf = dma_buf_count > 0;
        let render_path_label_text = format!("Render Path (SHM: {} / DMA-BUF: {})", shm_count, dma_buf_count);

        let commit_count = self.settings_manager.get_first_toplevel_commit_count();
        let commit_count_label = gtk4::Label::builder()
            .label(&format!("Top-Level Window Commit Count: {}", commit_count))
            .build();

        container.append(&commit_count_label);

        container
    }
}
