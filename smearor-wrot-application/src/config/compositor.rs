use atomic_float::AtomicF32;
use smearor_wrot_geometry::Margins;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicU32;
use typed_builder::TypedBuilder;

#[derive(Debug, TypedBuilder)]
pub struct CompositorConfig {
    /// Whether double buffering is enabled
    pub double_buffer: AtomicBool,

    /// Whether DMA-BUF hardware acceleration is enabled
    pub dma_buf: AtomicBool,

    /// Whether client-side decorations are enabled
    pub client_decorations: AtomicBool,

    /// Opacity of the compositor (0.0 = fully transparent, 1.0 = fully opaque).
    pub opacity: AtomicF32,

    /// Margins between the outer GTK application window and the compositor toplevel windows
    pub margins: Margins,

    /// Margin between the outer GTK application window and the compositor toplevel dialogs
    pub dialog_margin: AtomicU32,
}
