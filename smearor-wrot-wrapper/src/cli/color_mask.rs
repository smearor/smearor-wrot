use atomic_float::AtomicF32;
use clap::Parser;
use smearor_wrot_application::ColorMaskConfig;
use smearor_wrot_application::DEFAULT_COLOR_MASK_TOLERANCE;
use std::sync::atomic::AtomicBool;

#[derive(Parser, Debug, Clone)]
pub struct ColorMaskArguments {
    /// Background color in hex format (e.g., #FF0000 for red).
    #[arg(long)]
    pub(crate) background_color: Option<String>,

    /// Color mask in hex format for chroma-keying (e.g., #808080 to make gray transparent).
    #[arg(long)]
    pub(crate) color_mask: Option<String>,

    /// Enable automatic background color detection for color mask.
    #[arg(long, action = clap::ArgAction::SetTrue, default_value_t = false)]
    pub(crate) auto_color_mask: bool,

    /// Subsurface background color in hex format (e.g., #FF0000 for red on subsurfaces).
    #[arg(long)]
    pub(crate) subsurface_background_color: Option<String>,

    /// Subsurface color mask in hex format for chroma-keying (e.g., #FFFFFF to make white transparent on subsurfaces).
    #[arg(long)]
    pub(crate) subsurface_color_mask: Option<String>,

    /// Enable automatic background color detection for subsurface color mask.
    #[arg(long, action = clap::ArgAction::SetTrue, default_value_t = false)]
    pub(crate) auto_subsurface_color_mask: bool,

    /// Tolerance for color mask (0.0-1.0, default: 0.1).
    #[arg(long, default_value_t = DEFAULT_COLOR_MASK_TOLERANCE)]
    pub(crate) color_mask_tolerance: f32,

    /// Enable shader-based color masking for better performance (default: false).
    #[arg(long, action = clap::ArgAction::SetTrue, default_value_t = false)]
    pub(crate) color_mask_shader: bool,
}

impl From<ColorMaskArguments> for ColorMaskConfig {
    fn from(args: ColorMaskArguments) -> Self {
        ColorMaskConfig {
            background_color: args.background_color,
            color_mask: args.color_mask,
            auto_color_mask: AtomicBool::new(args.auto_color_mask),
            subsurface_background_color: args.subsurface_background_color,
            subsurface_color_mask: args.subsurface_color_mask,
            auto_subsurface_color_mask: AtomicBool::new(args.auto_subsurface_color_mask),
            color_mask_tolerance: AtomicF32::new(args.color_mask_tolerance),
            color_mask_shader: AtomicBool::new(args.color_mask_shader),
        }
    }
}
