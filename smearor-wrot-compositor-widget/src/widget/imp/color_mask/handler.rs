use crate::widget::color_mask::error::ColorMaskError;
use crate::widget::color_mask::handler::ColorMaskHandler;
use crate::widget::compositor::error::CompositorError;
use crate::widget::compositor::handler::CompositorHandler;
use crate::widget::imp::widget::CompositorWidgetImpl;
use glib::subclass::prelude::ObjectSubclassExt;
use gtk4::prelude::WidgetExt;
use smearor_wrot_compositor::color_mask::subsurface::SubSurfaceColorMask;
use smearor_wrot_compositor::color_mask::toplevel::TopLevelColorMask;
use tracing::debug;

impl ColorMaskHandler for CompositorWidgetImpl {
    fn clear_cached_dominant_color(&self) -> Result<(), ColorMaskError> {
        debug!("Auto Color Mask enabled: clearing cache and forcing buffer update");
        let compositor = self.compositor()?;
        let compositor = compositor.lock().map_err(|_| CompositorError::CompositorLockError)?;
        compositor.set_color_mask_detected(false);
        compositor.clear_color_mask()?;
        debug!("Color mask cache cleared, detection flag reset");
        compositor.force_buffer_update();
        debug!("Buffer update forced via configure events");
        // Trigger re-render to force color detection
        self.obj().queue_draw();
        debug!("Queue draw called to trigger re-render");
        Ok(())
    }

    fn clear_cached_dominant_color_subsurface(&self) -> Result<(), ColorMaskError> {
        debug!("Auto Subsurface Color Mask enabled: clearing cache and forcing buffer update");
        let compositor = self.compositor()?;
        let compositor = compositor.lock().map_err(|_| CompositorError::CompositorLockError)?;
        compositor.set_subsurface_color_mask_detected(false);
        compositor.clear_subsurface_color_mask()?;
        debug!("Subsurface color mask cache cleared, detection flag reset");
        compositor.force_buffer_update();
        debug!("Buffer update forced via configure events");
        // Trigger re-render to force color detection
        self.obj().queue_draw();
        debug!("Queue draw called to trigger re-render");
        Ok(())
    }
}
