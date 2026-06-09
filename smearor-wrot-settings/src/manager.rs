use gtk4::ApplicationWindow;
use gtk4::Widget;
use gtk4::prelude::*;
use smearor_wrot_compositor_widget::CompositorWidget;
use smearor_wrot_compositor_widget::widget::color_mask::handler::ColorMaskHandler;
use smearor_wrot_compositor_widget::widget::commit::CommitHandler;
use smearor_wrot_compositor_widget::widget::config::handler::ConfigHandler;
use smearor_wrot_compositor_widget::widget::dmabuf::handler::DmabufHandler;
use smearor_wrot_compositor_widget::widget::shm::handler::ShmHandler;
use smearor_wrot_debug_overlay::DebugOverlayManager;
use smearor_wrot_keyboard::manager::KeyboardManager;
use smearor_wrot_rotation::RotationWidget;
use smearor_wrot_window::WindowManager;
use std::sync::Arc;
use thiserror::Error;
use tracing::debug;
use typed_builder::TypedBuilder;

#[derive(Debug, Error)]
pub enum SettingsError {
    #[error("Failed to get compositor: {0}")]
    CompositorAccessError(String),

    #[error("Failed to apply config: {0}")]
    ConfigApplyError(String),

    #[error("Failed to clear cache: {0}")]
    CacheClearError(String),

    #[error("Window operation failed: {0}")]
    WindowError(String),
}

#[derive(Debug, Clone, TypedBuilder)]
pub struct SettingsManager {
    compositor_widget: Arc<CompositorWidget>,
    debug_overlay: Arc<DebugOverlayManager>,
    window_manager: Arc<WindowManager>,
    keyboard_manager: Arc<KeyboardManager>,
    parent_window: ApplicationWindow,
    rotation_widget: Arc<RotationWidget>,
    // disable_dma_buf: bool,
}

impl SettingsManager {
    // pub fn new(compositor_widget: &CompositorWidget, parent_window: &ApplicationWindow, rotation_widget: &Widget, disable_dma_buf: bool) -> Self {
    //     Self {
    //         compositor_widget: compositor_widget.clone(),
    //         parent_window: parent_window.clone(),
    //         rotation_widget: rotation_widget.clone(),
    //         disable_dma_buf,
    //     }
    // }

    pub fn set_debug_pointer(&self, enabled: bool) {
        debug!("Setting debug pointer to: {}", enabled);
        let mut config = self.debug_overlay.debug_overlay_config();
        config.debug_pointer = enabled;
        self.debug_overlay.update_debug_overlay_config(config);
    }

    pub fn set_debug_touch(&self, enabled: bool) {
        debug!("Setting debug touch to: {}", enabled);
        let mut config = self.debug_overlay.debug_overlay_config();
        config.debug_touch = enabled;
        self.debug_overlay.update_debug_overlay_config(config);
    }

    pub fn set_auto_color_mask(&self, enabled: bool) {
        debug!("Setting auto color mask to: {}", enabled);
        let mut config = self.compositor_widget.config();
        config.auto_color_mask = enabled;
        self.compositor_widget.set_config(config);
        let _ = self.compositor_widget.apply_config_to_compositor();

        if enabled {
            let _ = self.compositor_widget.clear_cached_dominant_color();
        }
    }

    pub fn set_auto_subsurface_color_mask(&self, enabled: bool) {
        debug!("Setting auto subsurface color mask to: {}", enabled);
        let mut config = self.compositor_widget.config();
        config.auto_subsurface_color_mask = enabled;
        self.compositor_widget.set_config(config);
        let _ = self.compositor_widget.apply_config_to_compositor();

        if enabled {
            let _ = self.compositor_widget.clear_cached_dominant_color_subsurface();
        }
    }

    pub fn set_resizable(&self, enabled: bool) {
        debug!("Setting resizable to: {}", enabled);
        let mut config = self.compositor_widget.config();
        config.resizable = enabled;
        self.compositor_widget.set_config(config);
        self.parent_window.set_resizable(enabled);
    }

    pub fn set_decorated(&self, enabled: bool) {
        debug!("Setting decorated to: {}", enabled);
        let mut config = self.compositor_widget.config();
        config.show_decorations = enabled;
        self.compositor_widget.set_config(config);
        self.parent_window.set_decorated(enabled);
    }

    pub fn set_disable_client_decorations(&self, enabled: bool) {
        debug!("Setting disable client decorations to: {}", enabled);
        let mut config = self.compositor_widget.config();
        config.disable_client_decorations = enabled;
        self.compositor_widget.set_config(config);
        let _ = self.compositor_widget.apply_config_to_compositor();
    }

    pub fn set_color_mask_shader(&self, enabled: bool) {
        debug!("Setting color mask shader to: {}", enabled);
        let mut config = self.compositor_widget.config();
        config.color_mask_shader = enabled;
        self.compositor_widget.set_config(config);
        let _ = self.compositor_widget.apply_config_to_compositor();
    }

    pub fn set_animations(&self, enabled: bool) {
        debug!("Setting animations to: {}", enabled);
        let mut config = self.compositor_widget.config();
        config.animations = enabled;
        self.compositor_widget.set_config(config);

        if let Some(rotation_widget) = self.rotation_widget.downcast_ref::<RotationWidget>() {
            rotation_widget.set_animations_enabled(enabled);
        }
    }

    pub fn get_dma_buf_render_count(&self) -> u32 {
        self.compositor_widget.get_dma_buf_render_count()
    }

    pub fn get_shm_render_count(&self) -> u32 {
        self.compositor_widget.get_shm_render_count()
    }

    pub fn get_first_toplevel_commit_count(&self) -> u32 {
        self.compositor_widget.get_first_toplevel_commit_count()
    }

    pub fn get_disable_dma_buf(&self) -> bool {
        self.disable_dma_buf
    }

    pub fn get_debug_pointer(&self) -> bool {
        self.compositor_widget.debug_overlay_config().debug_pointer
    }

    pub fn get_debug_touch(&self) -> bool {
        self.compositor_widget.debug_overlay_config().debug_touch
    }

    pub fn get_auto_color_mask(&self) -> bool {
        self.compositor_widget.config().auto_color_mask
    }

    pub fn get_auto_subsurface_color_mask(&self) -> bool {
        self.compositor_widget.config().auto_subsurface_color_mask
    }

    pub fn get_resizable(&self) -> bool {
        self.compositor_widget.config().resizable
    }

    pub fn get_decorated(&self) -> bool {
        self.compositor_widget.config().show_decorations
    }

    pub fn get_disable_client_decorations(&self) -> bool {
        self.compositor_widget.config().disable_client_decorations
    }

    pub fn get_color_mask_shader(&self) -> bool {
        self.compositor_widget.config().color_mask_shader
    }

    pub fn get_animations(&self) -> bool {
        self.compositor_widget.config().animations
    }
}
