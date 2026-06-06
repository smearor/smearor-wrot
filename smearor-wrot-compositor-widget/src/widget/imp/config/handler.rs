use crate::CompositorWidgetConfig;
use crate::widget::compositor::error::CompositorError;
use crate::widget::compositor::handler::CompositorHandler;
use crate::widget::config::error::ConfigError;
use crate::widget::config::handler::ConfigHandler;
use crate::widget::imp::widget::CompositorWidgetImpl;
use gtk4::Orientation;
use smearor_wrot_compositor::color_mask::subsurface::SubSurfaceColorMask;
use smearor_wrot_compositor::color_mask::toplevel::TopLevelColorMask;
use smearor_wrot_compositor::frame::limit::FrameLimiter;
use smearor_wrot_compositor::windows::decoration::ClientDecorationAware;
use smearor_wrot_model::geometry::size::Size;

impl ConfigHandler for CompositorWidgetImpl {
    fn set_config(&self, config: CompositorWidgetConfig) {
        if let Ok(mut guard) = self.config.lock() {
            *guard = config;
        }
    }

    fn config(&self) -> CompositorWidgetConfig {
        match self.config.lock() {
            Ok(guard) => guard.clone(),
            Err(_) => CompositorWidgetConfig::default(),
        }
    }

    fn apply_config_to_compositor(&self) -> Result<(), ConfigError> {
        let config = self.config();
        let compositor = self.compositor()?;
        let guard = compositor.lock().map_err(|_| CompositorError::CompositorLockError)?;
        guard.set_auto_color_mask(config.auto_color_mask);
        guard.set_auto_subsurface_color_mask(config.auto_subsurface_color_mask);
        guard.set_color_mask_tolerance(config.color_mask_tolerance);
        guard.set_color_mask_shader(config.color_mask_shader);
        guard.set_client_decorations_enabled(config.disable_client_decorations);
        guard.set_max_fps(config.max_fps);
        Ok(())
    }

    fn min_size_by_orientation(&self, orientation: Orientation) -> i32 {
        if orientation == Orientation::Horizontal {
            self.config().min_width
        } else {
            self.config().min_height
        }
    }

    fn min_size(&self) -> Size<i32> {
        Size::new(self.config().min_width, self.config().min_height)
    }
}
