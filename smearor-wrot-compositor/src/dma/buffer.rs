use crate::SmearorCompositor;
use smithay::backend::allocator::Fourcc;
use smithay::backend::allocator::dmabuf::Dmabuf;
use std::os::fd::RawFd;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

/// Trait for DMA-BUF hardware acceleration control
pub trait DmaBuffer {
    /// Enable or disable DMA-BUF hardware acceleration
    ///
    /// # Arguments
    ///
    /// * `enabled` - Whether to enable DMA-BUF hardware acceleration
    fn set_dma_buf_enabled(&self, enabled: bool);

    /// Check if DMA-BUF hardware acceleration is enabled
    ///
    /// # Returns
    ///
    /// * `bool` - true if DMA-BUF is enabled, false otherwise
    fn is_dma_buf_enabled(&self) -> bool;

    /// Check if DMA-BUF is available and can be used
    fn is_dma_buf_available(&self) -> bool;
}

/// Default implementation for types with an AtomicBool field
impl<T> DmaBuffer for T
where
    T: AsRef<AtomicBool>,
{
    fn set_dma_buf_enabled(&self, enabled: bool) {
        self.as_ref().store(enabled, Ordering::SeqCst);
    }

    fn is_dma_buf_enabled(&self) -> bool {
        self.as_ref().load(Ordering::SeqCst)
    }

    fn is_dma_buf_available(&self) -> bool {
        self.is_dma_buf_enabled()
    }
}

impl DmaBuffer for SmearorCompositor {
    fn set_dma_buf_enabled(&self, enabled: bool) {
        self.dma_buf_enabled.store(enabled, Ordering::SeqCst);

        // TODO: Phase 7 - DMA-BUF support for hardware acceleration - Implement conditional DMA-BUF global creation/removal
        // Currently, the DMA-BUF global is always created if a DRM device is available.
        // To properly disable DMA-BUF at the Smithay level, we need to either:
        // 1. Create the global conditionally during initialization (requires flag to be available earlier)
        // 2. Remove the global after creation (not directly supported by Smithay)
        // 3. Reject DMA-BUF requests when disabled (requires handler modification)
        // For now, DMA-BUF is disabled at the import level in render_node.rs
    }

    fn is_dma_buf_enabled(&self) -> bool {
        self.dma_buf_enabled.load(Ordering::SeqCst)
    }

    fn is_dma_buf_available(&self) -> bool {
        self.is_dma_buf_enabled() && self.dma_buf_state.is_some()
    }
}

/// DMA-BUF buffer handle
pub struct DmaBufBuffer {
    /// File descriptor for the DMA-BUF
    pub fd: RawFd,
    /// Buffer width in pixels
    pub width: u32,
    /// Buffer height in pixels
    pub height: u32,
    /// Buffer format (e.g., ARGB8888)
    pub format: u32,
    /// Buffer stride in bytes
    pub stride: u32,
    /// Buffer size in bytes
    pub size: usize,
}

impl DmaBufBuffer {
    /// Create a new DMA-BUF buffer handle
    ///
    /// # Arguments
    ///
    /// * `fd` - File descriptor for the DMA-BUF
    /// * `width` - Buffer width in pixels
    /// * `height` - Buffer height in pixels
    /// * `format` - Buffer format
    /// * `stride` - Buffer stride in bytes
    /// * `size` - Buffer size in bytes
    pub fn new(fd: RawFd, width: u32, height: u32, format: u32, stride: u32, size: usize) -> Self {
        Self {
            fd,
            width,
            height,
            format,
            stride,
            size,
        }
    }

    /// Create a DMA-BUF buffer from Smithay Dmabuf
    ///
    /// # Arguments
    ///
    /// * `dmabuf` - Smithay Dmabuf to convert
    /// * `width` - Buffer width in pixels
    /// * `height` - Buffer height in pixels
    /// * `format` - Buffer format (Fourcc)
    ///
    /// # Returns
    ///
    /// * `Result<Self, String>` - DMA-BUF buffer or error
    pub fn from_smithay_dmabuf(dmabuf: Dmabuf, width: u32, height: u32, format: Fourcc) -> Result<Self, String> {
        use smithay::backend::allocator::Buffer;
        use std::os::unix::io::AsRawFd;

        // Get the first plane's file descriptor
        let fd = dmabuf.handles().next().ok_or("Failed to get DMA-BUF file descriptor")?.as_raw_fd();

        // Get the first plane's stride
        let stride = dmabuf.strides().next().ok_or("Failed to get DMA-BUF stride")?;

        // Get the buffer size
        let size = dmabuf.size();
        let buffer_size = (size.w as usize) * (size.h as usize) * 4; // Assuming 4 bytes per pixel (ARGB)

        Ok(Self {
            fd,
            width,
            height,
            format: format as u32,
            stride,
            size: buffer_size,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn test_dma_buffer_enabled() {
        let dma_buf_enabled = Arc::new(AtomicBool::new(false));
        assert!(!dma_buf_enabled.is_dma_buf_enabled());
    }

    #[test]
    fn test_set_dma_buffer_enabled() {
        let dma_buf_enabled = Arc::new(AtomicBool::new(false));
        dma_buf_enabled.set_dma_buf_enabled(true);
        assert!(dma_buf_enabled.is_dma_buf_enabled());
    }

    #[test]
    fn test_dma_buffer_toggle() {
        let dma_buf_enabled = Arc::new(AtomicBool::new(true));
        assert!(dma_buf_enabled.is_dma_buf_enabled());
        dma_buf_enabled.set_dma_buf_enabled(false);
        assert!(!dma_buf_enabled.is_dma_buf_enabled());
        dma_buf_enabled.set_dma_buf_enabled(true);
        assert!(dma_buf_enabled.is_dma_buf_enabled());
    }
}
