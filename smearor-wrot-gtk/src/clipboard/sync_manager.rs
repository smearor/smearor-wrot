use super::format_converter::FormatConverter;
use super::host_clipboard::HostClipboard;
use super::host_clipboard::HostClipboardError;
use super::wayland_selection::WaylandSelectionHandler;
use crate::CompositorWidget;
use crate::clipboard::error::ClipboardSyncError;
use crate::widget::compositor::error::CompositorError;
use crate::widget::compositor::handler::CompositorHandler;
use glib;
use smearor_wrot_core::clipboard::handler::ClipboardSelectionHandler;
use std::io::Read;
use std::os::unix::io::FromRawFd;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use tokio;
use tracing::debug;
use tracing::error;
use tracing::warn;

/// Synchronization manager for bidirectional clipboard sync
/// Manages synchronization between host clipboard and Wayland selection
pub struct SyncManager {
    host_clipboard: Arc<HostClipboard>,
    compositor_widget: Arc<CompositorWidget>,
    sync_enabled: Arc<AtomicBool>,
    signal_handler_id: Arc<Mutex<Option<glib::SignalHandlerId>>>,
}

impl SyncManager {
    /// Create a new SyncManager instance with CompositorWidget
    ///
    /// # Errors
    /// Returns an error if the host clipboard cannot be initialized
    pub fn new_with_widget(compositor_widget: CompositorWidget) -> Result<Self, ClipboardSyncError> {
        Ok(Self {
            host_clipboard: Arc::new(HostClipboard::new()?),
            compositor_widget: Arc::new(compositor_widget),
            sync_enabled: Arc::new(AtomicBool::new(true)),
            signal_handler_id: Arc::new(Mutex::new(None)),
        })
    }

    /// Create a new SyncManager instance (deprecated, use new_with_widget)
    ///
    /// # Errors
    /// Returns an error if the host clipboard cannot be initialized
    pub fn new(_wayland_selection: Arc<WaylandSelectionHandler>) -> Result<Self, ClipboardSyncError> {
        // TODO: Phase 1 - Deprecate this method once new_with_widget is fully integrated
        Ok(Self {
            host_clipboard: Arc::new(HostClipboard::new()?),
            compositor_widget: Arc::new(CompositorWidget::new()),
            sync_enabled: Arc::new(AtomicBool::new(true)),
            signal_handler_id: Arc::new(Mutex::new(None)),
        })
    }

    /// Enable or disable clipboard synchronization
    pub fn set_sync_enabled(&self, enabled: bool) {
        self.sync_enabled.store(enabled, Ordering::Relaxed);
    }

    /// Check if synchronization is enabled
    pub fn is_sync_enabled(&self) -> bool {
        self.sync_enabled.load(Ordering::Relaxed)
    }

    /// Sync from host clipboard to Wayland selection
    /// Called when the host clipboard changes
    ///
    /// # Errors
    /// Returns an error if the synchronization fails
    pub async fn sync_host_to_wayland(&self) -> Result<(), ClipboardSyncError> {
        if !self.is_sync_enabled() {
            return Ok(());
        }
        if let Some(content) = self.host_clipboard.read_text().await? {
            if self.host_clipboard.is_own_content(&content) {
                return Ok(());
            }
            let converted = FormatConverter::gtk_to_wayland_text(&content);
            self.compositor_widget.set_clipboard_content(Some(converted.clone()))?;
            // This tells Wayland clients that there's a selection available
            CompositorWidget::set_selection_from_host(&self.compositor_widget, vec!["text/plain;charset=utf-8".to_string()])?;
            debug!("Set selection on seat with mime types: text/plain;charset=utf-8");
        }
        Ok(())
    }

    /// Sync from Wayland selection to host clipboard
    /// Called when the Wayland selection changes
    ///
    /// # Errors
    /// Returns an error if the synchronization fails
    pub fn sync_wayland_to_host(&self) -> Result<(), ClipboardSyncError> {
        if !self.is_sync_enabled() {
            return Ok(());
        }
        // The SelectionSource uses lazy data transfer, so we need to request the data
        // For now, use the clipboard_content that was set by new_selection
        if let Some(content) = self
            .compositor_widget
            .get_clipboard_content()
            .map_err(ClipboardSyncError::CompositorClipboardError)?
        {
            self.host_clipboard.write_text(FormatConverter::wayland_to_gtk_text(&content))?;
            debug!("Synced Wayland content to host clipboard");
        }

        Ok(())
    }

    /// Extract text from SelectionSource and sync to host clipboard
    /// This is called when a Wayland client sets the selection
    /// It extracts the text from the SelectionSource asynchronously using GIO and stores it in the host clipboard
    pub fn extract_and_sync_wayland_selection(&self) -> Result<(), ClipboardSyncError> {
        debug!("Extracting and syncing Wayland selection to host clipboard");
        let compositor_widget = self.compositor_widget.clone();
        let host_clipboard = self.host_clipboard.clone();

        let compositor = self.compositor_widget.compositor()?;
        let compositor = compositor.lock().map_err(|_| CompositorError::CompositorLockError)?;

        // Get the pipe read end from the compositor
        let raw_fd = {
            let mut pipe_read_end = compositor.clipboard_pipe_read_end.lock().map_err(|_| ClipboardSyncError::PipeReadLockError)?;
            pipe_read_end.take()
        };

        if let Some(fd) = raw_fd {
            debug!("Got pipe read end: {}", fd);
            // Create a File from the raw fd and read asynchronously using tokio
            let mut file = unsafe { std::fs::File::from_raw_fd(fd) };

            let host_clipboard_clone = host_clipboard.clone();

            let context = glib::MainContext::default();
            context.spawn_local(async move {
                debug!("Starting async read from pipe using tokio");
                let result = tokio::task::spawn_blocking(move || {
                    let mut buffer = String::new();
                    match file.read_to_string(&mut buffer) {
                        Ok(_) => Ok(buffer),
                        Err(e) => Err(e),
                    }
                })
                .await;

                match result {
                    Ok(Ok(text)) => {
                        debug!("Successfully read from pipe: {}", text);
                        // Sync to host clipboard directly
                        if let Err(e) = host_clipboard_clone.write_text(FormatConverter::wayland_to_gtk_text(&text)) {
                            error!("Failed to write Wayland selection to host clipboard: {}", e);
                        } else {
                            debug!("Successfully synced Wayland selection to host clipboard");
                        }
                    }
                    Ok(Err(e)) => {
                        error!("Failed to read from pipe: {}", e);
                    }
                    Err(e) => {
                        error!("Task failed: {}", e);
                    }
                }
            });
        } else {
            debug!("No pipe read end available");
            // Fallback to old method for compatibility
            compositor
                .extract_selection_source_text()
                .map_err(ClipboardSyncError::CompositorSelectionError)?;
            let context = glib::MainContext::default();
            context.spawn_local(async move {
                glib::timeout_future(std::time::Duration::from_millis(100)).await;
                match compositor_widget.get_clipboard_content() {
                    Ok(Some(content)) => {
                        if let Err(e) = host_clipboard.write_text(FormatConverter::wayland_to_gtk_text(&content)) {
                            error!("Failed to write Wayland selection to host clipboard: {}", e);
                        } else {
                            debug!("Successfully synced Wayland selection to host clipboard");
                        }
                    }
                    Ok(None) => {
                        warn!("No clipboard content found");
                    }
                    Err(e) => {
                        error!("Failed to get clipboard content: {e}");
                    }
                }
            });
        }
        Ok(())
    }

    /// Manual paste from host to compositor
    /// Called when the user clicks the Paste button
    ///
    /// # Errors
    /// Returns an error if the paste operation fails
    pub async fn manual_paste(&self) -> Result<(), ClipboardSyncError> {
        debug!("Manual paste triggered");
        self.sync_host_to_wayland().await
    }

    /// Manual copy from compositor to host
    /// Called when the user clicks the Copy button
    ///
    /// # Errors
    /// Returns an error if the copy operation fails
    pub fn manual_copy(&self) -> Result<(), ClipboardSyncError> {
        debug!("Manual copy triggered");
        self.sync_wayland_to_host()
    }

    /// Start automatic synchronization
    /// GTK4 uses connect_formats_notify for event-based change detection
    pub fn start_polling(&self) -> Result<(), ClipboardSyncError> {
        debug!("Starting clipboard polling with connect_formats_notify");
        let signal_handler_id = self.host_clipboard.connect_changed({
            let compositor_widget = self.compositor_widget.clone();
            let host_clipboard = self.host_clipboard.clone();
            move || {
                debug!("Clipboard change detected");
                let context = glib::MainContext::default();
                let compositor_widget_clone = compositor_widget.clone();
                let host_clipboard_clone = host_clipboard.clone();

                context.spawn_local(async move {
                    debug!("Reading clipboard content");
                    match host_clipboard_clone.read_text().await {
                        Ok(Some(content)) => {
                            debug!("Clipboard content read: {}", content);
                            if !host_clipboard_clone.is_own_content(&content) {
                                debug!("Content is not own, syncing to Wayland");
                                let converted = FormatConverter::gtk_to_wayland_text(&content);
                                debug!("Converted content: {}", converted);
                                match compositor_widget_clone.set_clipboard_content(Some(converted.clone())) {
                                    Ok(_) => match compositor_widget_clone.set_selection_from_host(vec!["text/plain;charset=utf-8".to_string()]) {
                                        Ok(_) => {
                                            debug!("Set selection on seat from automatic sync");
                                            debug!("Successfully set Wayland selection");
                                        }
                                        Err(e) => {
                                            debug!("Failed to set selection on seat from automatic sync: {e}");
                                        }
                                    },
                                    Err(e) => {
                                        debug!("Failed to set clipboard content: {e}");
                                    }
                                }
                            } else {
                                debug!("Content is own, skipping sync");
                            }
                        }
                        Ok(None) => debug!("Clipboard is empty"),
                        Err(e) => debug!("Failed to read clipboard: {}", e),
                    }
                });
            }
        });

        let mut guard = self
            .signal_handler_id
            .lock()
            .map_err(|_| ClipboardSyncError::HostClipboardError(HostClipboardError::DisplayError))?;
        *guard = Some(signal_handler_id);

        debug!("Clipboard polling started successfully");
        Ok(())
    }

    /// Stop automatic synchronization
    pub fn stop_polling(&self) {
        let Ok(mut guard) = self.signal_handler_id.lock() else {
            return;
        };
        if let Some(_signal_handler_id) = guard.take() {
            // SignalHandlerId disconnects automatically when dropped
            // The handler will be removed when the ID is dropped
        }
    }
}
