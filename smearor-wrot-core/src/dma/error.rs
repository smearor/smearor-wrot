/// DMA-BUF error types
#[derive(Debug)]
pub enum DmaBufError {
    /// DMA-BUF allocation failed
    AllocationFailed(String),
    /// DMA-BUF format not supported
    UnsupportedFormat,
    /// DMA-BUF feature not implemented yet
    NotImplemented,
    /// DMA-BUF initialization failed
    InitializationFailed(String),
    /// DRM node creation failed
    DrmNodeFailed(String),
    /// GBM device creation failed
    GbmDeviceFailed(String),
}

impl std::fmt::Display for DmaBufError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DmaBufError::AllocationFailed(msg) => write!(f, "DMA-BUF allocation failed: {}", msg),
            DmaBufError::UnsupportedFormat => write!(f, "DMA-BUF format not supported"),
            DmaBufError::NotImplemented => write!(f, "DMA-BUF feature not implemented"),
            DmaBufError::InitializationFailed(msg) => write!(f, "DMA-BUF initialization failed: {}", msg),
            DmaBufError::DrmNodeFailed(msg) => write!(f, "DRM node creation failed: {}", msg),
            DmaBufError::GbmDeviceFailed(msg) => write!(f, "GBM device creation failed: {}", msg),
        }
    }
}

impl std::error::Error for DmaBufError {}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dma_buf_error_display() {
        let err = DmaBufError::AllocationFailed("test".to_string());
        assert!(err.to_string().contains("allocation failed"));
    }
}
