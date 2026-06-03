use gtk4::Window;
use gtk4::glib;
use gtk4::prelude::*;
use smearor_wrot_gtk::CompositorWidget;
use smearor_wrot_gtk::widget::color_mask::handler::ColorMaskHandler;
use smearor_wrot_gtk::widget::commit::CommitHandler;
use smearor_wrot_gtk::widget::config::handler::ConfigHandler;
use smearor_wrot_gtk::widget::dmabuf::handler::DmabufHandler;
use smearor_wrot_gtk::widget::shm::handler::ShmHandler;
use smearor_wrot_rotation::RotationWidget;
use tracing::debug;

pub fn show_settings_dialog(parent_window: &Window, compositor_widget: &CompositorWidget, rotation_widget: &gtk4::Widget, disable_dma_buf: bool) {
    debug!("Opening settings dialog");

    // Create dialog window
    let dialog = Window::builder().title("Settings").modal(true).default_width(400).default_height(300).build();

    // Set transient parent
    dialog.set_transient_for(Some(parent_window));

    // Get current debug settings from compositor widget
    let config = compositor_widget.config();
    let current_debug_pointer = config.debug_pointer;
    let current_debug_touch = config.debug_touch;
    let current_auto_color_mask = config.auto_color_mask;
    let current_auto_subsurface_color_mask = config.auto_subsurface_color_mask;
    let current_resizable = config.resizable;
    let current_decorated = config.show_decorations;
    let current_disable_client_decorations = config.disable_client_decorations;
    let current_disable_dma_buf = disable_dma_buf;
    let current_color_mask_shader = config.color_mask_shader;
    let current_animations = config.animations;

    // Create content box
    let content_box = gtk4::Box::builder()
        .orientation(gtk4::Orientation::Vertical)
        .spacing(12)
        .margin_start(12)
        .margin_end(12)
        .margin_top(12)
        .margin_bottom(12)
        .build();

    // Set 50% transparent background on content box
    content_box.add_css_class("semi-transparent-background");
    let css_provider = gtk4::CssProvider::new();
    css_provider.load_from_data(".semi-transparent-background { background-color: rgba(0, 0, 0, 0.5); }");
    content_box
        .style_context()
        .add_provider(&css_provider, gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION);

    // Create Debug Pointer toggle
    let debug_pointer_label = gtk4::Label::builder().label("Debug Pointer").build();
    let debug_pointer_toggle = gtk4::Switch::builder().active(current_debug_pointer).build();

    let debug_pointer_box = gtk4::Box::builder().orientation(gtk4::Orientation::Horizontal).spacing(12).build();
    debug_pointer_box.append(&debug_pointer_label);
    debug_pointer_box.append(&gtk4::Box::builder().hexpand(true).build());
    debug_pointer_box.append(&debug_pointer_toggle);

    // Create Debug Touch toggle
    let debug_touch_label = gtk4::Label::builder().label("Debug Touch").build();
    let debug_touch_toggle = gtk4::Switch::builder().active(current_debug_touch).build();

    let debug_touch_box = gtk4::Box::builder().orientation(gtk4::Orientation::Horizontal).spacing(12).build();
    debug_touch_box.append(&debug_touch_label);
    debug_touch_box.append(&gtk4::Box::builder().hexpand(true).build());
    debug_touch_box.append(&debug_touch_toggle);

    // Add toggles to content box
    content_box.append(&debug_pointer_box);
    content_box.append(&debug_touch_box);

    // Create Auto Color Mask toggle
    let auto_color_mask_label = gtk4::Label::builder().label("Auto Color Mask").build();
    let auto_color_mask_toggle = gtk4::Switch::builder().active(current_auto_color_mask).build();

    let auto_color_mask_box = gtk4::Box::builder().orientation(gtk4::Orientation::Horizontal).spacing(12).build();
    auto_color_mask_box.append(&auto_color_mask_label);
    auto_color_mask_box.append(&gtk4::Box::builder().hexpand(true).build());
    auto_color_mask_box.append(&auto_color_mask_toggle);

    // Create Auto Subsurface Color Mask toggle
    let auto_subsurface_color_mask_label = gtk4::Label::builder().label("Auto Subsurface Color Mask").build();
    let auto_subsurface_color_mask_toggle = gtk4::Switch::builder().active(current_auto_subsurface_color_mask).build();

    let auto_subsurface_color_mask_box = gtk4::Box::builder().orientation(gtk4::Orientation::Horizontal).spacing(12).build();
    auto_subsurface_color_mask_box.append(&auto_subsurface_color_mask_label);
    auto_subsurface_color_mask_box.append(&gtk4::Box::builder().hexpand(true).build());
    auto_subsurface_color_mask_box.append(&auto_subsurface_color_mask_toggle);

    // Create Resizable toggle
    let resizable_label = gtk4::Label::builder().label("Resizable").build();
    let resizable_toggle = gtk4::Switch::builder().active(current_resizable).build();

    let resizable_box = gtk4::Box::builder().orientation(gtk4::Orientation::Horizontal).spacing(12).build();
    resizable_box.append(&resizable_label);
    resizable_box.append(&gtk4::Box::builder().hexpand(true).build());
    resizable_box.append(&resizable_toggle);

    // Create Decorated toggle
    let decorated_label = gtk4::Label::builder().label("Decorated").build();
    let decorated_toggle = gtk4::Switch::builder().active(current_decorated).build();

    let decorated_box = gtk4::Box::builder().orientation(gtk4::Orientation::Horizontal).spacing(12).build();
    decorated_box.append(&decorated_label);
    decorated_box.append(&gtk4::Box::builder().hexpand(true).build());
    decorated_box.append(&decorated_toggle);

    // Create Disable Client Decorations toggle
    let disable_client_decorations_label = gtk4::Label::builder().label("Disable Client Decorations").build();
    let disable_client_decorations_toggle = gtk4::Switch::builder().active(current_disable_client_decorations).build();

    let disable_client_decorations_box = gtk4::Box::builder().orientation(gtk4::Orientation::Horizontal).spacing(12).build();
    disable_client_decorations_box.append(&disable_client_decorations_label);
    disable_client_decorations_box.append(&gtk4::Box::builder().hexpand(true).build());
    disable_client_decorations_box.append(&disable_client_decorations_toggle);

    // Create Color Mask Shader toggle
    let color_mask_shader_label = gtk4::Label::builder().label("Color Mask Shader (GPU-based)").build();
    let color_mask_shader_toggle = gtk4::Switch::builder().active(current_color_mask_shader).build();

    let color_mask_shader_box = gtk4::Box::builder().orientation(gtk4::Orientation::Horizontal).spacing(12).build();
    color_mask_shader_box.append(&color_mask_shader_label);
    color_mask_shader_box.append(&gtk4::Box::builder().hexpand(true).build());
    color_mask_shader_box.append(&color_mask_shader_toggle);

    // Create Animations toggle
    let animations_label = gtk4::Label::builder().label("Animations").build();
    let animations_toggle = gtk4::Switch::builder().active(current_animations).build();

    let animations_box = gtk4::Box::builder().orientation(gtk4::Orientation::Horizontal).spacing(12).build();
    animations_box.append(&animations_label);
    animations_box.append(&gtk4::Box::builder().hexpand(true).build());
    animations_box.append(&animations_toggle);

    // Add new toggles to content box
    content_box.append(&auto_color_mask_box);
    content_box.append(&auto_subsurface_color_mask_box);
    content_box.append(&resizable_box);
    content_box.append(&decorated_box);
    content_box.append(&disable_client_decorations_box);
    content_box.append(&color_mask_shader_box);
    content_box.append(&animations_box);

    // Add separator
    let separator = gtk4::Separator::builder().orientation(gtk4::Orientation::Horizontal).build();
    content_box.append(&separator);

    // Create read-only DMA-BUF disabled toggle
    let disable_dma_buf_label = gtk4::Label::builder().label("DMA-BUF Disabled (--disable-dma-buf)").build();
    let disable_dma_buf_toggle = gtk4::Switch::builder().active(current_disable_dma_buf).sensitive(false).build();

    let disable_dma_buf_box = gtk4::Box::builder().orientation(gtk4::Orientation::Horizontal).spacing(12).build();
    disable_dma_buf_box.append(&disable_dma_buf_label);
    disable_dma_buf_box.append(&gtk4::Box::builder().hexpand(true).build());
    disable_dma_buf_box.append(&disable_dma_buf_toggle);

    // Create read-only render path toggle
    let render_path_label = gtk4::Label::builder().label("Render Path (SHM/DMA-BUF)").build();
    let render_path_toggle = gtk4::Switch::builder().active(false).sensitive(false).build();

    let render_path_box = gtk4::Box::builder().orientation(gtk4::Orientation::Horizontal).spacing(12).build();
    render_path_box.append(&render_path_label);
    render_path_box.append(&gtk4::Box::builder().hexpand(true).build());
    render_path_box.append(&render_path_toggle);

    // Add read-only toggles to content box
    content_box.append(&disable_dma_buf_box);
    content_box.append(&render_path_box);

    // Get render path statistics from compositor
    let dma_buf_count = compositor_widget.get_dma_buf_render_count();
    let shm_count = compositor_widget.get_shm_render_count();

    // Determine current render path based on counts (DMA-BUF if count > 0, otherwise SHM)
    let current_render_path_is_dma_buf = dma_buf_count > 0;
    render_path_toggle.set_active(current_render_path_is_dma_buf);

    // Update render path label with counts
    render_path_label.set_label(&format!("Render Path (SHM: {} / DMA-BUF: {})", shm_count, dma_buf_count));

    // Get commit count for top-level window
    let commit_count = compositor_widget.get_first_toplevel_commit_count();

    // Create commit count label (read-only)
    let commit_count_label = gtk4::Label::builder()
        .label(&format!("Top-Level Window Commit Count: {}", commit_count))
        .build();

    content_box.append(&commit_count_label);

    // Add close button
    let close_button = gtk4::Button::builder().label("Close").build();

    let dialog_clone = dialog.clone();
    close_button.connect_clicked(move |_| {
        dialog_clone.close();
    });

    content_box.append(&close_button);

    // Set content as dialog child
    dialog.set_child(Some(&content_box));

    // Connect toggle changes to update compositor widget
    let compositor_widget_clone = compositor_widget.clone();
    debug_pointer_toggle.connect_state_set(move |_, is_active| {
        let mut config = compositor_widget_clone.config();
        config.debug_pointer = is_active;
        compositor_widget_clone.set_config(config);
        glib::Propagation::Proceed
    });

    let compositor_widget_clone = compositor_widget.clone();
    debug_touch_toggle.connect_state_set(move |_, is_active| {
        let mut config = compositor_widget_clone.config();
        config.debug_touch = is_active;
        compositor_widget_clone.set_config(config);
        glib::Propagation::Proceed
    });

    // Connect Auto Color Mask toggle
    let compositor_widget_clone = compositor_widget.clone();
    auto_color_mask_toggle.connect_state_set(move |_, is_active| {
        debug!("Auto Color Mask toggle changed to: {}", is_active);
        let mut config = compositor_widget_clone.config();
        config.auto_color_mask = is_active;
        compositor_widget_clone.set_config(config);
        let _ = compositor_widget_clone.apply_config_to_compositor();

        // Clear cached dominant color and update compositor when enabling auto color mask
        if is_active {
            let _ = compositor_widget_clone.clear_cached_dominant_color();
        }

        glib::Propagation::Proceed
    });

    // Connect Auto Subsurface Color Mask toggle
    let compositor_widget_clone = compositor_widget.clone();
    auto_subsurface_color_mask_toggle.connect_state_set(move |_, is_active| {
        debug!("Auto Subsurface Color Mask toggle changed to: {}", is_active);
        let mut config = compositor_widget_clone.config();
        config.auto_subsurface_color_mask = is_active;
        compositor_widget_clone.set_config(config);
        let _ = compositor_widget_clone.apply_config_to_compositor();

        // Clear cached dominant color and update compositor when enabling auto subsurface color mask
        if is_active {
            let _ = compositor_widget_clone.clear_cached_dominant_color_subsurface();
        }

        glib::Propagation::Proceed
    });

    // Connect Resizable toggle
    let compositor_widget_clone = compositor_widget.clone();
    let window_clone = parent_window.clone();
    resizable_toggle.connect_state_set(move |_, is_active| {
        let mut config = compositor_widget_clone.config();
        config.resizable = is_active;
        compositor_widget_clone.set_config(config);
        window_clone.set_resizable(is_active);
        glib::Propagation::Proceed
    });

    // Connect Decorated toggle
    let compositor_widget_clone = compositor_widget.clone();
    let window_clone = parent_window.clone();
    decorated_toggle.connect_state_set(move |_, is_active| {
        let mut config = compositor_widget_clone.config();
        config.show_decorations = is_active;
        compositor_widget_clone.set_config(config);
        window_clone.set_decorated(is_active);
        glib::Propagation::Proceed
    });

    // Connect Disable Client Decorations toggle
    let compositor_widget_clone = compositor_widget.clone();
    disable_client_decorations_toggle.connect_state_set(move |_, is_active| {
        let mut config = compositor_widget_clone.config();
        config.disable_client_decorations = is_active;
        compositor_widget_clone.set_config(config);
        let _ = compositor_widget_clone.apply_config_to_compositor();
        glib::Propagation::Proceed
    });

    // Connect Color Mask Shader toggle
    let compositor_widget_clone = compositor_widget.clone();
    color_mask_shader_toggle.connect_state_set(move |_, is_active| {
        debug!("Color Mask Shader toggle changed to: {}", is_active);
        let mut config = compositor_widget_clone.config();
        config.color_mask_shader = is_active;
        compositor_widget_clone.set_config(config);
        let _ = compositor_widget_clone.apply_config_to_compositor();
        glib::Propagation::Proceed
    });

    // Connect Animations toggle
    let compositor_widget_clone = compositor_widget.clone();
    let rotation_widget_clone = rotation_widget.clone();
    animations_toggle.connect_state_set(move |_, is_active| {
        debug!("Animations toggle changed to: {}", is_active);
        let mut config = compositor_widget_clone.config();
        config.animations = is_active;
        compositor_widget_clone.set_config(config);

        // Update rotation widget animations enabled state
        if let Some(rotation_widget) = rotation_widget_clone.downcast_ref::<RotationWidget>() {
            rotation_widget.set_animations_enabled(is_active);
        }

        glib::Propagation::Proceed
    });

    // Show dialog
    dialog.present();
}
