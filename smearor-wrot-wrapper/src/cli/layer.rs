use clap::Parser;
use smearor_wrot_application::LayerShellState;
use smearor_wrot_application::SmearorLayer;

#[derive(Parser, Debug, Clone)]
pub struct LayerArguments {
    /// Specify the layer for the layer shell protocol (e.g., Background, Top).
    #[arg(long)]
    pub(crate) layer: Option<SmearorLayer>,

    /// Namespace for the layer shell, used by compositors for rules.
    #[arg(short = 'n', long)]
    pub(crate) namespace: Option<String>,
}

impl From<LayerArguments> for LayerShellState {
    fn from(args: LayerArguments) -> Self {
        Self {
            layer: args.layer,
            namespace: args.namespace,
        }
    }
}
