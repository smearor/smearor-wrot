use crate::dma::buffer::DmaBufBuffer;
use crate::dma::device::DEFAULT_DRM_DEVICE_PATHS;
use crate::dma::error::DmaBufError;
use gbm::BufferObjectFlags;
use smithay::backend::allocator::Fourcc;
use smithay::backend::allocator::gbm::GbmDevice;
use smithay::backend::drm::DrmNode;
use std::fs::File;
use std::fs::OpenOptions;
use std::os::fd::FromRawFd;
use std::os::fd::IntoRawFd;
use std::os::fd::OwnedFd;
use std::path::Path;
use std::path::PathBuf;
use tracing::debug;

pub trait DmaBufAllocator {
    /// Allocate a DMA-BUF buffer
    ///
    /// # Arguments
    ///
    /// * `width` - Buffer width in pixels
    /// * `height` - Buffer height in pixels
    /// * `format` - Buffer format (e.g., ARGB8888)
    ///
    /// # Returns
    ///
    /// * `Result<DmaBufBuffer>` - The allocated buffer or an error
    fn allocate_buffer(&self, width: u32, height: u32, format: u32) -> Result<DmaBufBuffer, DmaBufError>;

    /// Release a DMA-BUF buffer
    ///
    /// # Arguments
    ///
    /// * `buffer` - The buffer to release
    fn release_buffer(&self, buffer: DmaBufBuffer) -> Result<(), DmaBufError>;

    /// Check if DRM device is available and supports DMA-BUF
    ///
    /// # Returns
    ///
    /// * `bool` - true if DRM device with DMA-BUF support is available
    fn is_available(&self) -> bool;

    /// Get the DRM node for DMA-BUF operations
    ///
    /// # Returns
    ///
    /// * `Option<&DrmNode>` - The DRM node if available
    fn drm_node(&self) -> Option<&DrmNode>;

    fn device_path(&self) -> Option<&PathBuf>;

    /// Get the GBM device for OpenGL renderer initialization
    ///
    /// # Returns
    ///
    /// * `Option<GbmDevice<File>>` - The GBM device if initialized
    fn gbm_device(&self) -> Option<GbmDevice<File>>;
}

/// DMA-BUF buffer allocation and management
///
/// This structure provides methods for allocating and managing DMA-BUF buffers
/// for hardware-accelerated rendering using DRM/GEM.
#[derive(Default)]
pub struct DmaBufAllocatorImpl {
    /// DRM node for device management
    drm_node: Option<DrmNode>,

    /// GBM device for buffer allocation
    gbm_device: Option<GbmDevice<File>>,

    /// DRM device path (e.g., /dev/dri/renderD128)
    device_path: Option<PathBuf>,
}

impl DmaBufAllocatorImpl {
    /// Create a new DMA-BUF allocator
    pub fn new() -> Self {
        Self {
            drm_node: None,
            gbm_device: None,
            device_path: None,
        }
    }

    /// Initialize DMA-BUF allocator with DRM device if available
    ///
    /// This function tries to find a DRM device and initialize the DMA-BUF allocator.
    /// If no DRM device is found or initialization fails, it returns a basic allocator
    /// without DMA-BUF support.
    pub fn initialize_dma_buf_allocator() -> Self {
        // Try common DRM device paths
        for device_path in DEFAULT_DRM_DEVICE_PATHS {
            if Path::new(device_path).exists() {
                debug!("Found DRM device at: {}", device_path);
                match Self::with_device(device_path.to_string()) {
                    Ok(allocator) => {
                        if allocator.is_available() {
                            debug!("DMA-BUF allocator initialized successfully with device: {}", device_path);
                            return allocator;
                        } else {
                            debug!("DMA-BUF not available on device: {}", device_path);
                        }
                    }
                    Err(e) => {
                        debug!("Failed to initialize DMA-BUF allocator with device {}: {}", device_path, e);
                    }
                }
            }
        }

        debug!("No suitable DRM device found, using basic allocator without DMA-BUF support");
        Self::new()
    }

    /// Create a new DMA-BUF allocator with a specific DRM device
    ///
    /// # Arguments
    ///
    /// * `device_path` - Path to the DRM device (e.g., /dev/dri/renderD128)
    pub fn with_device<P: Into<PathBuf>>(device_path: P) -> Result<Self, DmaBufError> {
        let device_path = device_path.into();
        debug!("Initializing DMA-BUF allocator with device: {}", device_path.to_string_lossy());

        // Open DRM device
        let device_file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&device_path)
            .map_err(|e| DmaBufError::DrmNodeFailed(format!("Failed to open device: {}", e)))?;

        // Create DRM node
        let drm_node = DrmNode::from_file(
            device_file
                .try_clone()
                .map_err(|e| DmaBufError::DrmNodeFailed(format!("Failed to clone device file: {}", e)))?,
        )
        .map_err(|e| DmaBufError::DrmNodeFailed(format!("Failed to create DRM node: {}", e)))?;

        debug!("Created DRM node: {:?}", drm_node.dev_path());

        // Create GBM device
        let gbm_device = GbmDevice::new(device_file).map_err(|e| DmaBufError::GbmDeviceFailed(format!("Failed to create GBM device: {}", e)))?;

        debug!("Created GBM device successfully");

        Ok(Self {
            drm_node: Some(drm_node),
            gbm_device: Some(gbm_device),
            device_path: Some(device_path),
        })
    }

    /// Calculate buffer size for given dimensions and format
    ///
    /// # Arguments
    ///
    /// * `width` - Buffer width in pixels
    /// * `height` - Buffer height in pixels
    /// * `format` - Buffer format (e.g., ARGB8888 = 4 bytes per pixel)
    ///
    /// # Returns
    ///
    /// * `usize` - Buffer size in bytes
    pub fn calculate_buffer_size(width: u32, height: u32, _format: u32) -> usize {
        // TODO: Phase 7 - DMA-BUF support for hardware acceleration - Implement format-specific size calculation
        // For now, assume 4 bytes per pixel (ARGB8888)
        let bytes_per_pixel = 4;
        (width * height * bytes_per_pixel) as usize
    }

    /// Calculate buffer stride for given width and format
    ///
    /// # Arguments
    ///
    /// * `width` - Buffer width in pixels
    /// * `format` - Buffer format
    ///
    /// # Returns
    ///
    /// * `u32` - Buffer stride in bytes
    pub fn calculate_stride(width: u32, _format: u32) -> u32 {
        // TODO: Phase 7 - DMA-BUF support for hardware acceleration - Implement format-specific stride calculation
        // For now, assume 4 bytes per pixel (ARGB8888)
        width * 4
    }
}

impl DmaBufAllocator for DmaBufAllocatorImpl {
    fn allocate_buffer(&self, width: u32, height: u32, format: u32) -> Result<DmaBufBuffer, DmaBufError> {
        debug!("Allocating DMA-BUF buffer: {}x{}, format: {}", width, height, format);

        let gbm_device = self
            .gbm_device
            .as_ref()
            .ok_or_else(|| DmaBufError::InitializationFailed("GBM device not initialized".to_string()))?;

        let fourcc = Fourcc::try_from(format).map_err(|_| DmaBufError::UnsupportedFormat)?;
        let stride = Self::calculate_stride(width, format);
        let size = Self::calculate_buffer_size(width, height, format);

        let buffer_object: gbm::BufferObject<File> = gbm_device
            .create_buffer_object(width, height, fourcc, BufferObjectFlags::empty())
            .map_err(|e| DmaBufError::AllocationFailed(format!("Failed to create GBM buffer object: {}", e)))?;

        let owned_fd = buffer_object
            .fd()
            .map_err(|e| DmaBufError::AllocationFailed(format!("Failed to get DMA-BUF fd: {:?}", e)))?;
        let fd = owned_fd.into_raw_fd();

        let dma_buf_buffer = DmaBufBuffer::new(fd, width, height, format, stride, size);

        debug!("Successfully allocated DMA-BUF buffer with fd: {}", fd);

        Ok(dma_buf_buffer)
    }

    fn release_buffer(&self, buffer: DmaBufBuffer) -> Result<(), DmaBufError> {
        debug!("Releasing DMA-BUF buffer with fd: {}", buffer.fd);

        let owned_fd = unsafe { OwnedFd::from_raw_fd(buffer.fd) };

        drop(owned_fd);

        debug!("Successfully released DMA-BUF buffer with fd: {}", buffer.fd);

        Ok(())
    }

    fn is_available(&self) -> bool {
        self.drm_node.is_some() && self.gbm_device.is_some()
    }

    fn drm_node(&self) -> Option<&DrmNode> {
        self.drm_node.as_ref()
    }

    fn device_path(&self) -> Option<&PathBuf> {
        self.device_path.as_ref()
    }

    fn gbm_device(&self) -> Option<GbmDevice<File>> {
        self.gbm_device
            .as_ref()
            .map(|_device| {
                // Clone the GBM device by reopening the DRM device
                if let Some(device_path) = &self.device_path {
                    OpenOptions::new()
                        .read(true)
                        .write(true)
                        .open(device_path)
                        .ok()
                        .and_then(|file| GbmDevice::new(file).ok())
                } else {
                    None
                }
            })
            .flatten()
    }
}

impl Drop for DmaBufAllocatorImpl {
    fn drop(&mut self) {
        // TODO: Phase 7 - DMA-BUF support for hardware acceleration - Implement cleanup
        // Close DRM device if open
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dma_buf_allocator_creation() {
        let allocator = DmaBufAllocatorImpl::new();
        assert!(!allocator.is_available());
    }

    #[test]
    fn test_calculate_buffer_size() {
        let size = DmaBufAllocatorImpl::calculate_buffer_size(1920, 1080, 0);
        assert_eq!(size, 1920 * 1080 * 4);
    }

    #[test]
    fn test_calculate_stride() {
        let stride = DmaBufAllocatorImpl::calculate_stride(1920, 0);
        assert_eq!(stride, 1920 * 4);
    }
}
