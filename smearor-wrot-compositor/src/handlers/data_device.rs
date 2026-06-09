//! Wayland Data Device protocol handler

use nix::unistd;
use smithay::input::Seat;
use smithay::wayland::selection::SelectionHandler;
use smithay::wayland::selection::SelectionSource;
use smithay::wayland::selection::SelectionTarget;
use smithay::wayland::selection::data_device::ClientDndGrabHandler;
use smithay::wayland::selection::data_device::DataDeviceHandler;
use smithay::wayland::selection::data_device::DataDeviceState;
use smithay::wayland::selection::data_device::ServerDndGrabHandler;
use smithay::wayland::selection::data_device::request_data_device_client_selection;
use std::os::unix::io::AsRawFd;
use std::os::unix::io::FromRawFd;
use std::os::unix::io::IntoRawFd;
use std::os::unix::io::OwnedFd;
use tracing::debug;
use tracing::error;

use crate::compositor::SmearorCompositor;
use crate::message::compositor_message::CompositorMessage;
use crate::message::sender::CompositorMessageSender;

impl SelectionHandler for SmearorCompositor {
    type SelectionUserData = ();

    // TODO: Phase 1 - SelectionHandler new_selection - Handle Wayland→Host clipboard sync
    // Called when a client sets the selection
    fn new_selection(&mut self, _ty: SelectionTarget, _source: Option<SelectionSource>, _seat: Seat<Self>) {
        debug!("new_selection called for selection type: {:?}", _ty);
        // TODO: Phase 1 - Extract text from SelectionSource and store in clipboard_content
        // For now, store the source and extract text later
        if let Some(source) = _source {
            if let Ok(mut clipboard_source) = self.clipboard_source.lock() {
                *clipboard_source = Some(source.clone());
                debug!("Stored SelectionSource for clipboard sync");
            }
            // TODO: Phase 1 - Extract text from SelectionSource
            // This requires async handling and MIME-type negotiation
            // For text/plain, we need to read the data from the source
            // Create a pipe and request the data from the source
            // Clear clipboard_content when receiving a new SelectionSource from a Wayland client
            // This ensures we don't mistakenly skip extraction due to stale content
            if source.mime_types().contains(&"text/plain;charset=utf-8".to_string()) {
                // Check if this is a client-side selection (not server-side)
                // Server-side selections cannot be requested from the client
                let is_client_selection = if let Ok(clipboard_content) = self.clipboard_content.lock() {
                    clipboard_content.is_none()
                } else {
                    false
                };

                if !is_client_selection {
                    debug!("Selection is server-side, skipping client data request");
                    return;
                }

                if let Ok(mut clipboard_content) = self.clipboard_content.lock() {
                    debug!("Clearing clipboard_content for new Wayland client selection");
                    *clipboard_content = None;
                }
                debug!("SelectionSource contains text/plain, extracting data");
                match unistd::pipe() {
                    Ok((read_end, write_end)) => {
                        debug!("Created pipe for data extraction");
                        // Store the read end for async reading
                        // We need to store the raw fd, not the OwnedFd
                        let read_fd = read_end.as_raw_fd();
                        if let Ok(mut pipe_read_end) = self.clipboard_pipe_read_end.lock() {
                            *pipe_read_end = Some(read_fd);
                            debug!("Stored pipe read end: {}", read_fd);
                        }
                        // Convert write_end to OwnedFd for request_data_device_client_selection
                        // We need to leak the read_end to prevent it from being closed
                        std::mem::forget(read_end);
                        match request_data_device_client_selection(&_seat, "text/plain;charset=utf-8".to_string(), write_end) {
                            Ok(_) => {
                                debug!("Successfully requested data from SelectionSource");
                                // Send message to GTK wrapper to extract and sync the data
                                self.send_message(CompositorMessage::WaylandSelectionChanged);
                                debug!("Sent WaylandSelectionChanged message to GTK wrapper");
                            }
                            Err(e) => {
                                error!("Failed to request data from SelectionSource: {}", e);
                                // Clear the pipe read end since the request failed
                                if let Ok(mut pipe_read_end) = self.clipboard_pipe_read_end.lock() {
                                    *pipe_read_end = None;
                                }
                                // Close the read fd since the request failed
                                let _ = unistd::close(read_fd);
                            }
                        }
                    }
                    Err(e) => {
                        error!("Failed to create pipe: {}", e);
                    }
                }
            } else {
                debug!("SelectionSource does not contain text/plain, skipping extraction");
                debug!("Available mime types: {:?}", source.mime_types());
            }
        } else {
            debug!("Selection cleared");
            if let Ok(mut clipboard_source) = self.clipboard_source.lock() {
                *clipboard_source = None;
            }
            if let Ok(mut clipboard_content) = self.clipboard_content.lock() {
                *clipboard_content = None;
            }
            if let Ok(mut pipe_read_end) = self.clipboard_pipe_read_end.lock() {
                *pipe_read_end = None;
            }
        }
    }

    // TODO: Phase 1 - SelectionHandler send_selection - Handle Host→Wayland clipboard sync
    // Called when a client requests the selection
    fn send_selection(&mut self, _ty: SelectionTarget, _mime_type: String, _fd: OwnedFd, _seat: Seat<Self>, _user_data: &Self::SelectionUserData) {
        debug!("send_selection called for selection type: {:?}, mime_type: {}", _ty, _mime_type);
        // TODO: Phase 1 - Read from clipboard_content and write to fd
        if let Ok(clipboard_content) = self.clipboard_content.lock() {
            if let Some(content) = clipboard_content.as_ref() {
                debug!("Sending clipboard content: {}", content);
                // TODO: Phase 1 - Write content to fd using std::io::Write
                use std::io::Write;
                let mut file = unsafe { std::fs::File::from_raw_fd(_fd.into_raw_fd()) };
                if let Err(e) = file.write_all(content.as_bytes()) {
                    error!("Failed to write clipboard content to fd: {}", e);
                }
                if let Err(e) = file.flush() {
                    error!("Failed to flush fd: {}", e);
                }
            } else {
                debug!("Clipboard content is empty");
            }
        } else {
            error!("Failed to lock clipboard_content");
        }

        // TODO: Phase 1 - Clipboard Integration - Store content for Wayland→Host sync
        // When a Wayland client requests the selection, we also store the content
        // so it can be synced to the host clipboard
        if _ty == SelectionTarget::Clipboard && _mime_type == "text/plain;charset=utf-8" {
            if let Ok(clipboard_content) = self.clipboard_content.lock() {
                if let Some(content) = clipboard_content.as_ref() {
                    debug!("Storing content for Wayland→Host sync: {}", content);
                    // The content is already stored in clipboard_content
                    // The SyncManager will read it and sync to host clipboard
                }
            }
        }
    }
}

impl DataDeviceHandler for SmearorCompositor {
    fn data_device_state(&self) -> &DataDeviceState {
        &self.states.data_device_state
    }
}

impl ClientDndGrabHandler for SmearorCompositor {}
impl ServerDndGrabHandler for SmearorCompositor {}

smithay::delegate_data_device!(SmearorCompositor);
