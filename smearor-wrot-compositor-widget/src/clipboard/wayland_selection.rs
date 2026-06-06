use std::sync::Arc;
use std::sync::Mutex;
use thiserror::Error;

/// Wayland Selection Handler for clipboard synchronization
/// This implements the Smithay SelectionHandler trait to handle clipboard operations
/// between the compositor and child applications
pub struct WaylandSelectionHandler {
    current_selection: Arc<Mutex<Option<String>>>,
}

#[derive(Debug, Clone, Error)]
pub enum WaylandSelectionError {
    #[error("Failed to acquire lock")]
    LockError,
}

impl WaylandSelectionHandler {
    /// Create a new WaylandSelectionHandler instance
    pub fn new() -> Self {
        Self {
            current_selection: Arc::new(Mutex::new(None)),
        }
    }

    /// Get the current selection content
    ///
    /// # Errors
    /// Returns an error if the lock cannot be acquired
    pub fn get_selection(&self) -> Result<Option<String>, WaylandSelectionError> {
        self.current_selection
            .lock()
            .map(|guard| guard.clone())
            .map_err(|_| WaylandSelectionError::LockError)
    }

    /// Set the current selection content
    ///
    /// # Errors
    /// Returns an error if the lock cannot be acquired
    pub fn set_selection(&self, content: String) -> Result<(), WaylandSelectionError> {
        let mut selection = self.current_selection.lock().map_err(|_| WaylandSelectionError::LockError)?;
        *selection = Some(content);
        Ok(())
    }
}
