use atomic_float::AtomicF32;
use std::sync::atomic::AtomicBool;
use typed_builder::TypedBuilder;

#[derive(Debug, TypedBuilder)]
pub struct CompositorState {
    /// Whether double buffering is enabled
    pub double_buffer: AtomicBool,

    /// Whether DMA-BUF hardware acceleration is enabled
    pub dma_buf: AtomicBool,

    /// Whether client-side decorations are enabled
    pub client_decorations: AtomicBool,

    /// Opacity of the compositor (0.0 = fully transparent, 1.0 = fully opaque).
    pub opacity: AtomicF32,
}
