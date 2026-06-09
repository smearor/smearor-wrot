use crate::extract_pixel_data_from_texture;
use crate::widget::buffer::error::SaveBufferError;
use crate::widget::imp::dmabuf::render_node::DmaBufRenderNode;
use crate::widget::imp::shm::render_node::ShmRenderNode;
use crate::widget::imp::widget::CompositorWidgetImpl;
use gtk4::gdk::Texture;
use smearor_wrot_color_mask::ColorMask;
use smearor_wrot_compositor::SmearorCompositor;
use smearor_wrot_compositor::color_mask::toplevel::TopLevelColorMask;
use smearor_wrot_compositor::commit::count::CommitCount;
use smearor_wrot_compositor::commit::timing::CommitTiming;
use smearor_wrot_compositor::texture::pixel_data::BGRA;
use smearor_wrot_compositor::texture::pixel_data::PixelData;
use smearor_wrot_compositor::texture::pixel_data::RGBA;
use smearor_wrot_model_geometry::Size;
use smithay::backend::renderer::buffer_dimensions;
use smithay::reexports::wayland_server::backend::ObjectId;
use smithay::reexports::wayland_server::protocol::wl_buffer::WlBuffer;
use smithay::wayland::dmabuf::get_dmabuf;
use smithay::wayland::shm::with_buffer_contents;
use std::path::PathBuf;
use tracing::debug;
use tracing::error;
use tracing::info;

pub trait BufferHoldingArea {
    // Render buffer from Buffer-Holding-Area
    fn render_buffer_from_holding_area(&self, compositor: &SmearorCompositor, surface_id: &ObjectId) -> Option<Texture>;
    fn detect_color_mask_from_buffer(&self, compositor: &SmearorCompositor, surface_id: &ObjectId, buffer: &WlBuffer);
    fn apply_detected_color(&self, compositor: &SmearorCompositor, pixel_data: PixelData<BGRA>, surface_id: &ObjectId);
    fn save_buffer_to_png<P: Into<PathBuf>>(&self, pixel_data: &PixelData<BGRA>, buffer_size: &Size<u32>, path: P) -> Result<PathBuf, SaveBufferError>;
}

impl BufferHoldingArea for CompositorWidgetImpl {
    fn render_buffer_from_holding_area(&self, compositor: &SmearorCompositor, surface_id: &ObjectId) -> Option<Texture> {
        debug!("render_buffer_from_holding_area called for surface: {:?}", surface_id);
        let Ok(holding_area) = compositor.buffer_holding_area.lock() else {
            debug!("Failed to lock buffer_holding_area for surface: {:?}", surface_id);
            return None;
        };
        let Some(buffer) = holding_area.get(&surface_id) else {
            debug!("No buffer found in holding_area for surface: {:?}", surface_id);
            return None;
        };
        debug!("Using buffer from Buffer-Holding-Area for surface: {:?}", surface_id);

        // Auto-detect color mask if enabled and not yet detected
        self.detect_color_mask_from_buffer(compositor, surface_id, buffer);

        if let Some(texture) = self.dma_buf_render_window_to_texture(compositor, &surface_id, buffer) {
            debug!("DMA-BUF render successful for surface: {:?}", surface_id);
            return Some(texture);
        }
        debug!("DMA-BUF render failed or disabled, trying SHM for surface: {:?}", surface_id);
        if let Some(texture) = self.shm_render_window_to_texture(compositor, &surface_id, buffer) {
            debug!("SHM render successful for surface: {:?}", surface_id);
            return Some(texture);
        }
        debug!("Both DMA-BUF and SHM render failed for surface: {:?}", surface_id);

        // Fallback to texture_cache which contains pixel data from buffer commit
        debug!("Falling back to texture_cache for surface: {:?}", surface_id);
        if let Some(mut cache_entry) = compositor.texture_cache.get_mut(surface_id) {
            debug!("Using cached buffer data from texture_cache for surface: {:?}", surface_id);
            let texture_cache_entry = cache_entry.value_mut();

            // Create GDK texture from cached pixel data
            use crate::widget::imp::shm::texture::create_memory_texture_bgra;
            let gdk_texture = create_memory_texture_bgra(texture_cache_entry);
            debug!("Created GDK texture from texture_cache for surface: {:?}", surface_id);
            Some(Texture::from(gdk_texture))
        } else {
            debug!("No cached buffer data found in texture_cache for surface: {:?}", surface_id);
            None
        }
    }

    fn detect_color_mask_from_buffer(&self, compositor: &SmearorCompositor, surface_id: &ObjectId, buffer: &WlBuffer) {
        debug!("detect_color_mask_from_buffer CALLED for surface: {:?}", surface_id);

        if !compositor.get_auto_color_mask() || compositor.is_color_mask_detected() {
            debug!(
                "detect_color_mask_from_buffer: auto_color_mask={} or color_mask_detected={}, skipping",
                compositor.get_auto_color_mask(),
                compositor.is_color_mask_detected()
            );
            return;
        }

        let commit_count = compositor.get_commit_count(surface_id.clone());
        debug!(
            "Checking auto color mask detection (holding area): enabled={}, detected={}, commit_count={}",
            compositor.get_auto_color_mask(),
            compositor.is_color_mask_detected(),
            commit_count
        );

        // Wait for 2 commits to ensure the application is fully rendered
        if commit_count < 2 {
            debug!("Commit count < 2, skipping auto color detection");
            return;
        }

        // Wait for 1000ms after first commit to ensure the application is fully rendered
        if !compositor.has_enough_time_since_first_commit(surface_id.clone(), 100) {
            debug!("Not enough time since first commit, skipping auto color detection");
            return;
        }

        debug!(
            "Commit count >= 2 and 1000ms elapsed, attempting auto color detection from holding area for surface: {:?}",
            surface_id
        );

        // Check if buffer has valid dimensions
        let Some(buffer_size) = buffer_dimensions(buffer) else {
            debug!("Could not determine buffer dimensions, skipping auto color detection");
            return;
        };

        if buffer_size.w <= 0 || buffer_size.h <= 0 {
            debug!("Buffer has invalid dimensions: {}x{}, skipping auto color detection", buffer_size.w, buffer_size.h);
            return;
        }
        debug!("Buffer has valid dimensions: {}x{}", buffer_size.w, buffer_size.h);

        // Update window size and invalidate color mask if size changed
        compositor.update_window_size(surface_id.clone(), buffer_size.w, buffer_size.h);

        // Try DMA-BUF first
        if let Ok(_dmabuf) = get_dmabuf(buffer) {
            debug!("Buffer is DMA-BUF, extracting pixel data for color detection");
            // Import DMA-BUF to extract pixel data
            if let Some(texture) = self.dma_buf_render_window_to_texture(compositor, surface_id, buffer) {
                let pixel_data = extract_pixel_data_from_texture(&texture);
                self.apply_detected_color(compositor, pixel_data, surface_id);
            }
            return;
        }

        // Try SHM
        if let Ok(Some(_)) = with_buffer_contents(buffer, |_, _, _| Some(())) {
            debug!("Buffer is SHM, extracting pixel data for color detection");
            if let Some(texture) = self.shm_render_window_to_texture(compositor, surface_id, buffer) {
                let pixel_data = extract_pixel_data_from_texture(&texture);
                self.apply_detected_color(compositor, pixel_data, surface_id);
            }
            return;
        }

        debug!("Failed to determine buffer type for color detection");
    }

    fn apply_detected_color(&self, compositor: &SmearorCompositor, pixel_data: PixelData<BGRA>, surface_id: &ObjectId) {
        let Some(dominant_color) = pixel_data.get_dominant_color(8) else {
            debug!("Failed to detect dominant color from pixel data");
            return;
        };

        let tolerance = compositor.get_color_mask_tolerance();
        let Ok(_) = compositor.set_color_mask(ColorMask::new(dominant_color, tolerance)) else {
            error!("Failed to set color mask from holding area data");
            return;
        };

        compositor.set_color_mask_detected(true);
        info!(
            "Auto-detected dominant color from holding area: {} with tolerance {:.2} (surface: {:?})",
            dominant_color, tolerance, surface_id
        );
    }

    fn save_buffer_to_png<P: Into<PathBuf>>(&self, pixel_data: &PixelData<BGRA>, buffer_size: &Size<u32>, path: P) -> Result<PathBuf, SaveBufferError> {
        let path = path.into();
        let rgba_pixel_data = PixelData::<RGBA>::from(pixel_data);
        rgba_pixel_data.save_png(&path, buffer_size).map_err(SaveBufferError::PixelDataSaveError)?;
        info!("Saved buffer as image/png: {}", path.to_string_lossy());
        Ok(path.to_owned())
    }
}
