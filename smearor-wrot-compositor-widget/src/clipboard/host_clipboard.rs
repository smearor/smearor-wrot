use glib::prelude::ToValue;
use gtk4::gdk::ContentProvider;
use gtk4::gdk::Display;
use gtk4::prelude::DisplayExt;
use std::sync::Arc;
use std::sync::Mutex;
use thiserror::Error;

/// Host clipboard wrapper for accessing the host system clipboard via GTK4
/// This provides access to the clipboard of the host compositor (GNOME or Hyprland)
pub struct HostClipboard {
    clipboard: gtk4::gdk::Clipboard,
    last_pushed_content: Arc<Mutex<Option<String>>>,
}

#[derive(Debug, Clone, Error)]
pub enum HostClipboardError {
    #[error("Failed to get default display")]
    DisplayError,

    #[error("Failed to write text to clipboard: {0}")]
    WriteTextError(String),

    #[error("Failed to read text from clipboard: {0}")]
    ReadTextError(String),

    #[error("Failed to connect clipboard change handler: {0}")]
    ConnectError(String),
}

impl HostClipboard {
    /// Create a new HostClipboard instance
    ///
    /// # Errors
    /// Returns an error if the display cannot be obtained
    pub fn new() -> Result<Self, HostClipboardError> {
        Ok(Self {
            clipboard: Display::default().ok_or(HostClipboardError::DisplayError)?.clipboard(),
            last_pushed_content: Arc::new(Mutex::new(None)),
        })
    }

    /// Read text from the host clipboard asynchronously
    ///
    /// # Errors
    /// Returns an error if the clipboard cannot be read
    pub async fn read_text(&self) -> Result<Option<String>, HostClipboardError> {
        self.clipboard
            .read_text_future()
            .await
            .map(|s| s.map(|s| s.to_string()))
            .map_err(|e| HostClipboardError::ReadTextError(e.to_string()))
    }

    /// Write text to the host clipboard
    ///
    /// # Errors
    /// Returns an error if the clipboard cannot be written
    pub fn write_text(&self, text: String) -> Result<(), HostClipboardError> {
        {
            let Ok(mut last_pushed) = self.last_pushed_content.lock() else {
                return Err(HostClipboardError::WriteTextError("Failed to lock last_pushed_content".to_string()));
            };
            *last_pushed = Some(text.clone());
        }
        let content_provider = ContentProvider::for_value(&text.to_value());
        self.clipboard
            .set_content(Some(&content_provider))
            .map_err(|e| HostClipboardError::WriteTextError(e.to_string()))
    }

    /// Check if the current clipboard content was set by this instance
    /// This is used for loop protection to avoid infinite synchronization loops
    pub fn is_own_content(&self, current_content: &str) -> bool {
        let Ok(last_pushed) = self.last_pushed_content.lock() else {
            return false;
        };
        match last_pushed.as_ref() {
            Some(last_pushed) => {
                // Handle null-terminator differences by trimming before comparison
                last_pushed.trim_matches('\0') == current_content.trim_matches('\0')
            }
            None => false,
        }
    }

    /// Connect a callback for clipboard change events
    ///
    /// GTK4 uses property notifications instead of direct changed signals
    /// We use connect_formats_notify to detect clipboard content changes
    ///
    /// # Returns
    /// Returns the signal handler ID
    pub fn connect_changed<F>(&self, callback: F) -> glib::SignalHandlerId
    where
        F: Fn() + 'static,
    {
        // GTK4: Use connect_formats_notify instead of connect_changed
        // The "formats" property changes whenever clipboard content changes
        let callback = Arc::new(callback);

        self.clipboard.connect_formats_notify(move |_clipboard| {
            let callback_clone = callback.clone();
            callback_clone();
        })
    }
}
