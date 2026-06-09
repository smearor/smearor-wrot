use clap::Parser;
use smearor_wrot_application::DebugOverlayState;
use std::sync::atomic::AtomicBool;

#[derive(Parser, Debug, Clone)]
pub struct DebugOverlayArguments {
    /// Enable visual debugging of pointer.
    #[arg(long, action = clap::ArgAction::SetTrue, default_value_t = false)]
    pub(crate) debug_pointer: bool,

    /// Enable visual debugging of touch points.
    #[arg(long, action = clap::ArgAction::SetTrue, default_value_t = false)]
    pub(crate) debug_touch: bool,
}

impl From<DebugOverlayArguments> for DebugOverlayState {
    fn from(args: DebugOverlayArguments) -> Self {
        DebugOverlayState {
            debug_pointer: AtomicBool::new(args.debug_pointer),
            debug_touch: AtomicBool::new(args.debug_touch),
        }
    }
}
