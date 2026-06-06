use crate::widget::buffer::error::SaveBufferError;
use crate::widget::buffer::handler::BufferHandler;
use crate::widget::compositor::error::CompositorError;
use crate::widget::compositor::handler::CompositorHandler;
use crate::widget::imp::holding_area::BufferHoldingArea;
use crate::widget::imp::widget::CompositorWidgetImpl;
use smearor_wrot_core::surface::SurfaceQuery;
use smearor_wrot_core::texture::pixel_data::BGRA;
use smearor_wrot_core::texture::pixel_data::PixelData;
use smearor_wrot_model::geometry::size::Size;
use std::path::PathBuf;

impl BufferHandler for CompositorWidgetImpl {
    fn save_buffer_to_png<P: Into<PathBuf>>(&self, path: P) -> Result<PathBuf, SaveBufferError> {
        let compositor = self.compositor().map_err(SaveBufferError::CompositorError)?;
        let guard = compositor.lock().map_err(|_| CompositorError::CompositorLockError)?;
        let surface_id = guard.get_first_toplevel_surface_id().ok_or(SaveBufferError::NoTopLevelSurface)?;

        match guard.texture_cache.get(&surface_id) {
            Some(texture_cache_entry) => {
                let cache_entry = texture_cache_entry.value();
                let pixel_data = PixelData::<BGRA>::from_slice(cache_entry.pixel_data.as_slice());
                let buffer_size = Size::new(cache_entry.buffer_metadata.width, cache_entry.buffer_metadata.height).into();
                BufferHoldingArea::save_buffer_to_png(self, &pixel_data, &buffer_size, path)
            }
            None => {
                let texture = self
                    .render_buffer_from_holding_area(&guard, &surface_id)
                    .ok_or(SaveBufferError::RenderBufferFromHoldingAreaError)?;
                let pixel_data = crate::extract_pixel_data_from_texture(&texture);
                let buffer_size = Size::from(&texture).into();
                BufferHoldingArea::save_buffer_to_png(self, &pixel_data, &buffer_size, path)
            }
        }
    }
}
