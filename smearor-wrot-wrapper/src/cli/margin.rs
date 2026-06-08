use clap::Parser;
use smearor_wrot_application::MarginConfig;
use smearor_wrot_application::Margins;
use std::sync::atomic::AtomicU32;

#[derive(Parser, Debug, Clone)]
pub struct MarginArguments {
    /// Dialog margin in pixels for dialog positioning (default: 0).
    #[arg(long, default_value_t = 0)]
    pub(crate) dialog_margin: u32,

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
}

impl From<MarginArguments> for MarginConfig {
    fn from(args: MarginArguments) -> Self {
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

        MarginConfig {
            margin_left: AtomicU32::new(margins.left),
            margin_right: AtomicU32::new(margins.right),
            margin_top: AtomicU32::new(margins.top),
            margin_bottom: AtomicU32::new(margins.bottom),
            dialog_margin: AtomicU32::new(args.dialog_margin),
        }
    }
}
