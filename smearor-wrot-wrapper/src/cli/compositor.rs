use atomic_float::AtomicF32;
use clap::Parser;
use smearor_wrot_application::CompositorState;
use smearor_wrot_application::Margins;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicU32;

#[derive(Parser, Debug, Clone)]
pub struct CompositorArguments {
    /// Disable double buffering for wayland clients in the compositor (default: false).
    #[arg(long, action = clap::ArgAction::SetTrue, default_value_t = false)]
    pub(crate) disable_double_buffer: bool,

    /// Disable DMA-BUF hardware acceleration for wayland clients in the compositor (default: false).
    #[arg(long, action = clap::ArgAction::SetTrue, default_value_t = false)]
    pub(crate) disable_dma_buf: bool,

    /// Disable client-side decorations for wayland clients in the compositor.
    #[arg(long, action = clap::ArgAction::SetTrue, default_value_t = false)]
    pub(crate) disable_client_decorations: bool,

    /// Opacity of the compositor (0.0 = fully transparent, 1.0 = fully opaque).
    #[arg(long, default_value_t = 1.0, min = 0.0, max = 1.0)]
    pub(crate) opacity: f32,
}

impl From<CompositorArguments> for CompositorState {
    fn from(args: CompositorArguments) -> Self {
        CompositorState {
            double_buffer: AtomicBool::new(!args.disable_double_buffer),
            dma_buf: AtomicBool::new(!args.disable_dma_buf),
            client_decorations: AtomicBool::new(!args.disable_client_decorations),
            opacity: AtomicF32::new(args.opacity.clamp(0.0, 1.0)),
        }
    }
}
