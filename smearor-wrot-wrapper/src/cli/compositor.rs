use atomic_float::AtomicF32;
use clap::Parser;
use smearor_wrot_application::CompositorConfig;
use smearor_wrot_application::Margins;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicU32;

#[derive(Parser, Debug, Clone)]
pub struct CompositorArguments {
    /// Dialog margin in pixels for dialog positioning (default: 0).
    #[arg(long, default_value_t = 0)]
    pub(crate) dialog_margin: u32,

    /// Disable double buffering for wayland clients in the compositor (default: false).
    #[arg(long, action = clap::ArgAction::SetTrue, default_value_t = false)]
    pub(crate) disable_double_buffer: bool,

    /// Disable DMA-BUF hardware acceleration for wayland clients in the compositor (default: false).
    #[arg(long, action = clap::ArgAction::SetTrue, default_value_t = false)]
    pub(crate) disable_dma_buf: bool,

    /// Disable client-side decorations for wayland clients in the compositor.
    #[arg(long, action = clap::ArgAction::SetTrue, default_value_t = false)]
    pub(crate) disable_client_decorations: bool,

    /// Margin in pixels for window rendering (shortcut for all margins).
    #[arg(long)]
    pub(crate) margin: Option<u32>,

    /// Left margin in pixels for window rendering.
    #[arg(long, default_value_t = 0)]
    pub(crate) margin_left: u32,

    /// Right margin in pixels for window rendering.
    #[arg(long, default_value_t = 0)]
    pub(crate) margin_right: u32,

    /// Top margin in pixels for window rendering.
    #[arg(long, default_value_t = 0)]
    pub(crate) margin_top: u32,

    /// Bottom margin in pixels for window rendering.
    #[arg(long, default_value_t = 0)]
    pub(crate) margin_bottom: u32,

    /// Opacity of the compositor (0.0 = fully transparent, 1.0 = fully opaque).
    #[arg(long, default_value_t = 1.0, min = 0.0, max = 1.0)]
    pub(crate) opacity: f32,
}

impl From<CompositorArguments> for CompositorConfig {
    fn from(args: CompositorArguments) -> Self {
        let mut margins = if let Some(margin) = args.margin {
            Margins::all(margin)
        } else {
            Margins::default()
        };
        if let Some(margin_left) = args.margin_left {
            margins.left = margin_left;
        }
        if let Some(margin_right) = args.margin_right {
            margins.right = margin_right;
        }
        if let Some(margin_top) = args.margin_top {
            margins.top = margin_top;
        }
        if let Some(margin_bottom) = args.margin_bottom {
            margins.bottom = margin_bottom;
        }

        CompositorConfig {
            double_buffer: AtomicBool::new(!args.disable_double_buffer),
            dma_buf: AtomicBool::new(!args.disable_dma_buf),
            client_decorations: AtomicBool::new(!args.disable_client_decorations),
            opacity: AtomicF32::new(args.opacity.clamp(0.0, 1.0)),
            margins,
            dialog_margin: AtomicU32::new(args.dialog_margin),
        }
    }
}
