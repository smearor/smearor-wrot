//! Wayland Compositor protocol handler

use crate::compositor::SmearorCompositor;
use crate::damage::surface::SurfaceDamage;
use crate::state::ClientState;
use smithay::backend::renderer::utils::on_commit_buffer_handler;
use smithay::reexports::wayland_server::Client;
use smithay::reexports::wayland_server::Resource;
use smithay::reexports::wayland_server::protocol::wl_surface::WlSurface;
use smithay::utils::Point;
use smithay::utils::Rectangle;
use smithay::utils::Size;
use smithay::wayland::compositor::BufferAssignment;
use smithay::wayland::compositor::CompositorClientState;
use smithay::wayland::compositor::CompositorHandler;
use smithay::wayland::compositor::CompositorState;
use smithay::wayland::compositor::Damage;
use smithay::wayland::compositor::SurfaceAttributes;
use smithay::wayland::compositor::get_parent;
use smithay::wayland::compositor::is_sync_subsurface;
use smithay::wayland::compositor::with_states;
use std::sync::LazyLock;
use std::sync::atomic::Ordering;

use crate::buffer::metadata::BufferMetadata;
use crate::commit::count::CommitCount;
use crate::commit::timing::CommitTiming;
use crate::margin::handler::MarginHandler;
use crate::message::compositor_message::CompositorMessage;
use crate::message::sender::CompositorMessageSender;
use crate::popup::handler::PopupHandler;
use crate::surface::SurfaceQuery;
use crate::surface::commit::TopLevelCommitHandler;
use crate::texture::cache::TextureCacheEntry;
use crate::texture::pixel_data::BGRA;
use crate::texture::pixel_data::PixelData;
use tracing::debug;
use tracing::error;

impl CompositorHandler for SmearorCompositor {
    fn compositor_state(&mut self) -> &mut CompositorState {
        &mut self.compositor_state
    }

    fn client_compositor_state<'a>(&self, client: &'a Client) -> &'a CompositorClientState {
        static DEFAULT_STATE: LazyLock<CompositorClientState> = LazyLock::new(CompositorClientState::default);
        client.get_data::<ClientState>().map(|data| &data.compositor_state).unwrap_or_else(|| {
            error!("Failed to get client compositor state, using default");
            // Return a reference to a dummy state - this is a recovery mechanism
            // In production, we might want to return an error or handle this differently
            &DEFAULT_STATE
        })
    }

    fn destroyed(&mut self, surface: &WlSurface) {
        let surface_id = surface.id();
        debug!("Surface destroyed: {:?}", surface_id);

        // TODO: Phase 5 - Subsurface Rendering - Implement buffer cleanup strategy
        // For subsurfaces, buffers are needed for rendering even after surface destruction.
        // We don't remove buffers from Buffer-Holding-Area immediately to allow rendering.
        // Buffers will be replaced when new buffers are committed or through periodic cleanup.
        // For now, we only cleanup other surface-related data.

        // Cleanup other surface-related data
        self.texture_cache.remove(&surface_id);
        self.damage_regions.remove(&surface_id);
        self.surface_buffers.remove(&surface_id);
        self.render_cache.remove(&surface_id);

        debug!("Cleaned up surface data for: {:?}", surface_id);
    }

    fn new_subsurface(&mut self, surface: &WlSurface, parent: &WlSurface) {
        debug!("New subsurface created: {:?} with parent: {:?}", surface.id(), parent.id());
        let Ok(mut subsurfaces) = self.subsurfaces.lock() else {
            error!("Failed to lock subsurfaces registry");
            return;
        };
        subsurfaces.push(surface.clone());
        debug!("Subsurface registry now has {} subsurfaces", subsurfaces.len());
    }

    fn commit(&mut self, surface: &WlSurface) {
        debug!("=== COMMIT START === Commit called for surface: {:?}", surface.id());

        let surface_id = surface.id();

        // Increment commit count for this surface
        self.increment_commit_count(surface_id.clone());
        debug!("Commit count for surface {:?}: {}", surface_id, self.get_commit_count(surface_id.clone()));

        // Record first commit time if this is the first commit
        if self.get_commit_count(surface_id.clone()) == 1 {
            self.record_first_commit_time(surface_id.clone());
            debug!("Recorded first commit time for surface: {:?}", surface_id);
            // Only send FirstCommit message once globally, not per surface
            if !self.first_commit_received.load(Ordering::Relaxed) {
                self.first_commit_received.store(true, Ordering::Relaxed);
                self.send_message(CompositorMessage::FirstCommit);
            }
        }

        // Buffer thief: We intercept the buffer before Smithay processes it
        with_states(surface, |surface_data| {
            let mut attrs = surface_data.cached_state.get::<SurfaceAttributes>();

            // We check the current state (pending is already None when we are here)
            let current_buffer = attrs.current().buffer.as_ref();

            debug!("Checking current buffer state {:?}: {:?}", surface_id, current_buffer);

            if let Some(BufferAssignment::NewBuffer(buffer)) = current_buffer {
                debug!("CAPTURED: Buffer of type  {:?} intercepted and secured in Holding-Area: {:?}", buffer.id(), surface_id);

                // Store buffer in Holding-Area (hard reference)
                // This increases the reference count and prevents Smithay from releasing the buffer
                let mut holding_area = self.buffer_holding_area.lock().unwrap();
                holding_area.insert(surface_id.clone(), buffer.clone());
                drop(holding_area);

                // Check if buffer is DMA-BUF
                use smithay::wayland::dmabuf::get_dmabuf;
                if let Ok(_dmabuf) = get_dmabuf(&buffer) {
                    debug!("Buffer is DMA-BUF for surface: {:?}", surface_id);
                } else {
                    debug!("Buffer is not DMA-BUF for surface: {:?}", surface_id);

                    // Cache SHM buffer data in texture_cache for both toplevels and subsurfaces
                    // This ensures subsurfaces can be rendered even after the surface is destroyed
                    use smithay::wayland::shm::with_buffer_contents;
                    if let Ok(()) = with_buffer_contents(&buffer, |memory_pointer, data_length, buffer_metadata| {
                        if data_length > 0 {
                            let pixel_data_slice = unsafe { std::slice::from_raw_parts(memory_pointer, data_length) };
                            let texture_cache_entry =
                                TextureCacheEntry::new(BufferMetadata::from(&buffer_metadata), PixelData::<BGRA>::new(pixel_data_slice.to_vec()));

                            debug!("Caching SHM buffer data in texture_cache for surface: {:?} ({texture_cache_entry})", surface_id);

                            self.texture_cache.insert(surface_id.clone(), texture_cache_entry);
                        }
                    }) {
                        debug!("Successfully cached SHM buffer data for surface: {:?}", surface_id);
                    } else {
                        debug!("Failed to cache SHM buffer data for surface: {:?}", surface_id);
                    }
                }

                // Store buffer in holding area for both DMA-BUF and SHM
                // This ensures render_buffer_from_holding_area can access the buffer
                let mut holding_area = self.buffer_holding_area.lock().unwrap();
                holding_area.insert(surface_id.clone(), buffer.clone());
                drop(holding_area);
            } else {
                debug!("No new buffer in current state (state: {:?})", current_buffer);
            }
        });

        // IMPORTANT: Now run Smithay's standard logic
        on_commit_buffer_handler::<Self>(surface);

        // Now the buffer in Smithay might be 'Removed',
        // but we have our own clone in 'buffer_holding_area'!

        // Extract damage information from surface attributes
        let has_damage_regions = with_states(surface, |surface_data| !surface_data.cached_state.get::<SurfaceAttributes>().current().damage.is_empty());

        if has_damage_regions {
            // Extract and convert damage regions
            let damage_regions = with_states(surface, |surface_data| {
                let mut regions = Vec::new();
                for damage in &surface_data.cached_state.get::<SurfaceAttributes>().current().damage {
                    let rect = match damage {
                        Damage::Surface(rect) => {
                            debug!("Surface damage rect({}, {}, {}, {})", rect.loc.x, rect.loc.y, rect.size.w, rect.size.h);
                            Rectangle::new(Point::new(rect.loc.x, rect.loc.y), Size::new(rect.size.w, rect.size.h))
                        }
                        Damage::Buffer(rect) => {
                            debug!("Buffer damage rect({}, {}, {}, {})", rect.loc.x, rect.loc.y, rect.size.w, rect.size.h);
                            Rectangle::new(Point::new(rect.loc.x, rect.loc.y), Size::new(rect.size.w, rect.size.h))
                        }
                    };
                    regions.push(rect);
                }
                regions
            });

            if damage_regions.is_empty() {
                // All damage regions were filtered out (all were full damage)
                // Mark entire surface as damaged
                debug!("All damage regions were full damage, marking entire surface as damaged");
                self.mark_surface_damage(surface, None);
            } else {
                for region in damage_regions {
                    self.mark_surface_damage(surface, Some(region));
                }
            }
        } else {
            // No damage regions provided by client - mark entire surface as damaged
            // This is correct Wayland behavior: a commit without damage implies the entire surface should be rendered
            debug!("No damage regions from client, marking entire surface as damaged");
            self.mark_surface_damage(surface, None);
        }
        debug!("Marked surface {:?} as damaged", surface.id());

        // Store frame callbacks to send after GTK renders frame
        // This synchronizes Firefox with GTK's rendering cycle
        if let Some(output) = &self.virtual_output {
            let mut root = surface.clone();
            while let Some(parent) = get_parent(&root) {
                root = parent;
            }
            if let Some(window) = self.window_for_surface(&root) {
                let has_frame_callbacks = with_states(surface, |surface_data| {
                    !surface_data.cached_state.get::<SurfaceAttributes>().current().frame_callbacks.is_empty()
                });
                if has_frame_callbacks {
                    let elapsed = self.start_time.elapsed();
                    if let Ok(mut pending) = self.pending_frame_callbacks.lock() {
                        pending.push((surface.clone(), elapsed));
                        debug!("Stored pending frame callback for surface {:?}", surface.id());
                    }
                }
            }
        }

        if !is_sync_subsurface(surface) {
            let mut root = surface.clone();
            while let Some(parent) = get_parent(&root) {
                root = parent;
            }
            if let Some(window) = self.window_for_surface(&root) {
                window.on_commit();
                debug!("Window geometry after window.on_commit(): {:?}", window.geometry());

                // TODO: Phase 6 - Dialog Management - Check dialog size and activate
                // Check if this window is a dialog
                if let Ok(dialogs) = self.dialogs.lock() {
                    if dialogs.iter().any(|d| d.wl_surface() == &root) {
                        debug!("This window is a dialog, checking size and activating");

                        // Check if the dialog size exceeds 80% of the widget size
                        let output_size = if let Some(output) = &self.virtual_output {
                            output.current_mode().map(|mode| (mode.size.w, mode.size.h)).unwrap_or((1920, 1080))
                        } else {
                            (1920, 1080)
                        };

                        let margin_left = self.get_margin_left() as i32;
                        let margin_right = self.get_margin_right() as i32;
                        let margin_top = self.get_margin_top() as i32;
                        let margin_bottom = self.get_margin_bottom() as i32;

                        let dialog_margin = self.get_dialog_margin() as i32;

                        let adjusted_width = output_size.0 - margin_left - margin_right - 2 * dialog_margin;
                        let adjusted_height = output_size.1 - margin_top - margin_bottom - 2 * dialog_margin;

                        // Ensure adjusted size is positive
                        let adjusted_width = adjusted_width.max(100);
                        let adjusted_height = adjusted_height.max(100);

                        let window_geometry = window.geometry();
                        let current_width = window_geometry.size.w as i32;
                        let current_height = window_geometry.size.h as i32;

                        debug!("Dialog size: {}x{}, adjusted size: {}x{}", current_width, current_height, adjusted_width, adjusted_height);

                        if current_width > adjusted_width || current_height > adjusted_height {
                            debug!("Dialog size exceeds adjusted size limit, sending configure event");

                            // Send configure event with 80% limit
                            if let Some(toplevel) = window.toplevel() {
                                toplevel.send_configure();
                                debug!("Sent configure event to dialog to enforce size limit");
                            }
                        }
                    }
                } else {
                    debug!("Failed to lock dialogs registry for size check");
                }
            }
        };

        self.handle_toplevel_commit(surface);

        // If this is a subsurface with a buffer, mark damage on parent toplevel to trigger rendering
        // Firefox uses subsurface architecture where content is in subsurface but toplevel needs rendering
        let is_subsurface = if let Ok(subsurfaces) = self.subsurfaces.lock() {
            subsurfaces.contains(surface)
        } else {
            false
        };

        let has_buffer = if let Ok(holding_area) = self.buffer_holding_area.lock() {
            holding_area.contains_key(&surface_id)
        } else {
            false
        };

        if is_subsurface && has_buffer {
            debug!("Subsurface with buffer detected, finding parent toplevel to mark damage");
            // Find parent toplevel surface using get_parent function
            if let Some(parent) = get_parent(surface) {
                debug!("Found parent toplevel for subsurface: {:?}", parent.id());
                // Mark entire parent surface as damaged to trigger rendering
                self.mark_surface_damage(&parent, None);
                debug!("Marked parent toplevel as damaged for subsurface commit");
            }
            // Send message to force GTK to redraw immediately
            if let Ok(sender_option) = self.message_sender.lock() {
                if let Some(sender) = sender_option.as_ref() {
                    let _ = sender.send(CompositorMessage::ForceRedraw);
                }
            }
        }

        // TODO: Phase 5 - Implement buffer caching at commit time
        // Render buffer when committed and store in texture_cache for snapshot()
        // This fixes the timing issue where buffer is marked as "Removed" before snapshot() renders

        self.handle_popup_commits(surface);
    }
}

smithay::delegate_compositor!(SmearorCompositor);
