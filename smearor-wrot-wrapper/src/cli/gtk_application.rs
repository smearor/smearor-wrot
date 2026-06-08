use clap::Parser;
use smearor_wrot_application::GtkApplicationConfig;
use std::sync::atomic::AtomicI64;

#[derive(Parser, Debug, Clone)]
pub struct GtkApplicationArguments {
    /// The application id.
    #[arg(short = 'i', long)]
    pub(crate) id: Option<String>,

    /// Maximum frames per second (default: 60).
    #[arg(long, default_value_t = 60)]
    pub(crate) max_fps: i64,
}

impl From<GtkApplicationArguments> for GtkApplicationConfig {
    fn from(args: GtkApplicationArguments) -> Self {
        Self {
            id: args.id,
            max_fps: AtomicI64::new(args.max_fps),
        }
    }
}
