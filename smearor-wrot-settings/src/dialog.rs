use crate::SettingsManager;
use gtk4::Label;
use gtk4::Orientation;
use gtk4::Switch;
use gtk4::glib;
use gtk4::glib::Propagation;
use gtk4::prelude::*;
use std::sync::Arc;
use tracing::debug;

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

        Self { dialog, settings_manager }
    }

    pub fn show(&self) {
        self.dialog.present();
    }

    pub fn build(&self) -> gtk4::Window {
        let content_box = gtk4::Box::builder()
            .orientation(Orientation::Vertical)
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

        let separator = gtk4::Separator::builder().orientation(Orientation::Horizontal).build();
        content_box.append(&separator);

        let read_only_box = self.create_read_only_toggles();
        content_box.append(&read_only_box);

        // let statistics_box = self.create_statistics_labels();
        // content_box.append(&statistics_box);

        let close_button = gtk4::Button::builder().label("Close").build();

        let dialog_clone = self.dialog.clone();
        close_button.connect_clicked(move |_| {
            dialog_clone.close();
        });

        content_box.append(&close_button);

        self.dialog.set_child(Some(&content_box));

        self.dialog.clone()
    }

    fn create_debug_overlay_toggles(&self) -> gtk4::Box {
        let container = gtk4::Box::builder().orientation(Orientation::Vertical).spacing(12).build();

        let debug_pointer_label = Label::builder().label("Debug Pointer").build();
        let debug_pointer_toggle = Switch::builder().active(self.settings_manager.get_debug_pointer()).build();

        let debug_pointer_row = gtk4::Box::builder().orientation(Orientation::Horizontal).spacing(12).build();
        debug_pointer_row.append(&debug_pointer_label);
        debug_pointer_row.append(&gtk4::Box::builder().hexpand(true).build());
        debug_pointer_row.append(&debug_pointer_toggle);

        let settings_manager_clone = self.settings_manager.clone();
        debug_pointer_toggle.connect_state_set(move |_, is_active| {
            settings_manager_clone.set_debug_pointer(is_active);
            Propagation::Proceed
        });

        let debug_touch_label = Label::builder().label("Debug Touch").build();
        let debug_touch_toggle = Switch::builder().active(self.settings_manager.get_debug_touch()).build();

        let debug_touch_row = gtk4::Box::builder().orientation(Orientation::Horizontal).spacing(12).build();
        debug_touch_row.append(&debug_touch_label);
        debug_touch_row.append(&gtk4::Box::builder().hexpand(true).build());
        debug_touch_row.append(&debug_touch_toggle);

        let settings_manager_clone = self.settings_manager.clone();
        debug_touch_toggle.connect_state_set(move |_, is_active| {
            settings_manager_clone.set_debug_touch(is_active);
            Propagation::Proceed
        });

        container.append(&debug_pointer_row);
        container.append(&debug_touch_row);

        container
    }

    fn create_compositor_config_toggles(&self) -> gtk4::Box {
        let container = gtk4::Box::builder().orientation(Orientation::Vertical).spacing(12).build();

        let auto_color_mask_label = Label::builder().label("Auto Color Mask").build();
        let auto_color_mask_toggle = Switch::builder().active(self.settings_manager.get_auto_color_mask()).build();

        let auto_color_mask_row = gtk4::Box::builder().orientation(Orientation::Horizontal).spacing(12).build();
        auto_color_mask_row.append(&auto_color_mask_label);
        auto_color_mask_row.append(&gtk4::Box::builder().hexpand(true).build());
        auto_color_mask_row.append(&auto_color_mask_toggle);

        let settings_manager_clone = self.settings_manager.clone();
        auto_color_mask_toggle.connect_state_set(move |_, is_active| {
            settings_manager_clone.set_auto_color_mask(is_active);
            Propagation::Proceed
        });

        let auto_subsurface_color_mask_label = Label::builder().label("Auto Subsurface Color Mask").build();
        let auto_subsurface_color_mask_toggle = Switch::builder().active(self.settings_manager.get_auto_subsurface_color_mask()).build();

        let auto_subsurface_color_mask_row = gtk4::Box::builder().orientation(Orientation::Horizontal).spacing(12).build();
        auto_subsurface_color_mask_row.append(&auto_subsurface_color_mask_label);
        auto_subsurface_color_mask_row.append(&gtk4::Box::builder().hexpand(true).build());
        auto_subsurface_color_mask_row.append(&auto_subsurface_color_mask_toggle);

        let settings_manager_clone = self.settings_manager.clone();
        auto_subsurface_color_mask_toggle.connect_state_set(move |_, is_active| {
            settings_manager_clone.set_auto_subsurface_color_mask(is_active);
            Propagation::Proceed
        });

        let resizable_label = Label::builder().label("Resizable").build();
        let resizable_toggle = Switch::builder().active(self.settings_manager.get_resizable()).build();

        let resizable_row = gtk4::Box::builder().orientation(Orientation::Horizontal).spacing(12).build();
        resizable_row.append(&resizable_label);
        resizable_row.append(&gtk4::Box::builder().hexpand(true).build());
        resizable_row.append(&resizable_toggle);

        let settings_manager_clone = self.settings_manager.clone();
        resizable_toggle.connect_state_set(move |_, is_active| {
            settings_manager_clone.set_resizable(is_active);
            Propagation::Proceed
        });

        let decorated_label = Label::builder().label("Decorated").build();
        let decorated_toggle = Switch::builder().active(self.settings_manager.get_decorated()).build();

        let decorated_row = gtk4::Box::builder().orientation(Orientation::Horizontal).spacing(12).build();
        decorated_row.append(&decorated_label);
        decorated_row.append(&gtk4::Box::builder().hexpand(true).build());
        decorated_row.append(&decorated_toggle);

        let settings_manager_clone = self.settings_manager.clone();
        decorated_toggle.connect_state_set(move |_, is_active| {
            settings_manager_clone.set_decorated(is_active);
            Propagation::Proceed
        });

        let disable_client_decorations_label = Label::builder().label("Disable Client Decorations").build();
        let disable_client_decorations_toggle = Switch::builder().active(self.settings_manager.get_disable_client_decorations()).build();

        let disable_client_decorations_row = gtk4::Box::builder().orientation(Orientation::Horizontal).spacing(12).build();
        disable_client_decorations_row.append(&disable_client_decorations_label);
        disable_client_decorations_row.append(&gtk4::Box::builder().hexpand(true).build());
        disable_client_decorations_row.append(&disable_client_decorations_toggle);

        let settings_manager_clone = self.settings_manager.clone();
        disable_client_decorations_toggle.connect_state_set(move |_, is_active| {
            settings_manager_clone.set_disable_client_decorations(is_active);
            Propagation::Proceed
        });

        let color_mask_shader_label = Label::builder().label("Color Mask Shader (GPU-based)").build();
        let color_mask_shader_toggle = Switch::builder().active(self.settings_manager.get_color_mask_shader()).build();

        let color_mask_shader_row = gtk4::Box::builder().orientation(Orientation::Horizontal).spacing(12).build();
        color_mask_shader_row.append(&color_mask_shader_label);
        color_mask_shader_row.append(&gtk4::Box::builder().hexpand(true).build());
        color_mask_shader_row.append(&color_mask_shader_toggle);

        let settings_manager_clone = self.settings_manager.clone();
        color_mask_shader_toggle.connect_state_set(move |_, is_active| {
            settings_manager_clone.set_color_mask_shader(is_active);
            Propagation::Proceed
        });

        let animations_label = Label::builder().label("Animations").build();
        let animations_toggle = Switch::builder().active(self.settings_manager.get_animations()).build();

        let animations_row = gtk4::Box::builder().orientation(Orientation::Horizontal).spacing(12).build();
        animations_row.append(&animations_label);
        animations_row.append(&gtk4::Box::builder().hexpand(true).build());
        animations_row.append(&animations_toggle);

        let settings_manager_clone = self.settings_manager.clone();
        animations_toggle.connect_state_set(move |_, is_active| {
            settings_manager_clone.set_animations(is_active);
            Propagation::Proceed
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
        let container = gtk4::Box::builder().orientation(Orientation::Vertical).spacing(12).build();

        // let disable_dma_buf_label = Label::builder().label("DMA-BUF Disabled (--disable-dma-buf)").build();
        // let disable_dma_buf_toggle = Switch::builder().active(self.settings_manager.get_disable_dma_buf()).sensitive(false).build();

        // let disable_dma_buf_row = gtk4::Box::builder().orientation(Orientation::Horizontal).spacing(12).build();
        // disable_dma_buf_row.append(&disable_dma_buf_label);
        // disable_dma_buf_row.append(&gtk4::Box::builder().hexpand(true).build());
        // disable_dma_buf_row.append(&disable_dma_buf_toggle);

        let shm_count = self.settings_manager.get_shm_render_count();
        let shm_label = Label::builder().label(format!("SHM: {shm_count}")).build();

        let dma_buf_count = self.settings_manager.get_dma_buf_render_count();
        let dma_buf_label = Label::builder().label(format!("DMA-BUF {dma_buf_count}")).build();

        let render_path_toggle = Switch::builder().active(dma_buf_count > 0).build();
        let settings_manager_clone = self.settings_manager.clone();
        render_path_toggle.connect_state_set(move |_, is_active| {
            // settings_manager_clone.set_dma_buf_disabled(!is_active);
            Propagation::Proceed
        });

        let render_path_row = gtk4::Box::builder().orientation(Orientation::Horizontal).spacing(12).build();
        render_path_row.append(&gtk4::Box::builder().hexpand(true).build());
        render_path_row.append(&shm_label);
        render_path_row.append(&render_path_toggle);
        render_path_row.append(&dma_buf_label);
        render_path_row.append(&gtk4::Box::builder().hexpand(true).build());

        // container.append(&disable_dma_buf_row);
        container.append(&render_path_row);

        container
    }
}
