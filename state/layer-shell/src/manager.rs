use crate::LayerShellState;
use std::sync::Arc;
use typed_builder::TypedBuilder;

/// Manages the (outer) application window and layer shell.
#[derive(Debug, TypedBuilder)]
pub struct LayerShellStateManager {
    /// The layer shell state
    state: Arc<LayerShellState>,
}

impl LayerShellStateManager {}
