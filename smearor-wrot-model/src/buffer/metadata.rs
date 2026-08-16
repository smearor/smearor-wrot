use std::fmt::Display;
use std::fmt::Formatter;

/// Metadata describing a Wayland buffer's dimensions and stride.
#[derive(Debug, Clone)]
pub struct BufferMetadata {
    /// Buffer width in pixels
    pub width: i32,
    /// Buffer height in pixels
    pub height: i32,
    /// Buffer stride (bytes per row)
    pub stride: i32,
}

impl BufferMetadata {
    /// Creates new buffer metadata
    pub fn new(width: i32, height: i32, stride: i32) -> Self {
        Self { width, height, stride }
    }
}

#[cfg(feature = "smithay")]
impl From<&smithay::wayland::shm::BufferData> for BufferMetadata {
    fn from(buffer_data: &smithay::wayland::shm::BufferData) -> Self {
        Self {
            width: buffer_data.width,
            height: buffer_data.height,
            stride: buffer_data.stride,
        }
    }
}

impl Display for BufferMetadata {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x{} with stride {}", self.width, self.height, self.stride)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer_metadata_new() {
        let metadata = BufferMetadata::new(1920, 1080, 7680);
        assert_eq!(metadata.width, 1920);
        assert_eq!(metadata.height, 1080);
        assert_eq!(metadata.stride, 7680);
    }

    #[test]
    fn test_buffer_metadata_display() {
        let metadata = BufferMetadata::new(800, 600, 3200);
        assert_eq!(format!("{}", metadata), "800x600 with stride 3200");
    }
}
