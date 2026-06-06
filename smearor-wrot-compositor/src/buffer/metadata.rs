use smithay::wayland::shm::BufferData;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct BufferMetadata {
    pub width: i32,
    pub height: i32,
    pub stride: i32,
}

impl BufferMetadata {
    pub fn new(width: i32, height: i32, stride: i32) -> Self {
        Self { width, height, stride }
    }
}

impl From<&BufferData> for BufferMetadata {
    fn from(buffer_data: &BufferData) -> Self {
        Self {
            width: buffer_data.width,
            height: buffer_data.height,
            stride: buffer_data.stride,
        }
    }
}

impl Display for BufferMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x{} with stride {}", self.width, self.height, self.stride)
    }
}
