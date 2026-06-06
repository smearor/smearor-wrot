use crate::CompositorWidget;
use crate::widget::color_mask::error::ColorMaskError;
use glib::subclass::prelude::ObjectSubclassIsExt;

pub trait ColorMaskHandler {
    fn clear_cached_dominant_color(&self) -> Result<(), ColorMaskError>;
    fn clear_cached_dominant_color_subsurface(&self) -> Result<(), ColorMaskError>;
}

impl ColorMaskHandler for CompositorWidget {
    fn clear_cached_dominant_color(&self) -> Result<(), ColorMaskError> {
        self.imp().clear_cached_dominant_color()
    }

    fn clear_cached_dominant_color_subsurface(&self) -> Result<(), ColorMaskError> {
        self.imp().clear_cached_dominant_color_subsurface()
    }
}
