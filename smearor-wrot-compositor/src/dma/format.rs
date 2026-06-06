use gbm::Modifier;
use smithay::backend::allocator::Format;
use smithay::backend::allocator::Fourcc;

pub struct DmabufFormatProvider;

impl DmabufFormatProvider {
    // Returns common DMA-BUF formats
    pub fn get_dma_buf_formats() -> Vec<Format> {
        vec![
            Format {
                code: Fourcc::Argb8888,
                modifier: Modifier::Linear,
            },
            Format {
                code: Fourcc::Xrgb8888,
                modifier: Modifier::Linear,
            },
            Format {
                code: Fourcc::Abgr8888,
                modifier: Modifier::Linear,
            },
            Format {
                code: Fourcc::Xbgr8888,
                modifier: Modifier::Linear,
            },
        ]
    }
}
