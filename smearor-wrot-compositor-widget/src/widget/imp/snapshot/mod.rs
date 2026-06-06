use crate::color_mask::color_mask_applier::ColorMaskApplier;
use crate::color_mask::color_mask_applier::dma_buf::DmaBufColorMaskApplier;
use crate::widget::imp::snapshot::background_color::SnapshotBackgroundColor;
use crate::widget::imp::snapshot::error_renderer::ErrorRenderer;
use crate::widget::imp::widget::CompositorWidgetImpl;
use glib::subclass::prelude::ObjectSubclassExt;
use gtk4::prelude::SnapshotExt;
use gtk4::prelude::TextureExt;
use gtk4::prelude::WidgetExt;
use smearor_wrot_compositor::color_mask::toplevel::TopLevelColorMask;
use smearor_wrot_compositor::damage::surface::SurfaceDamage;
use smearor_wrot_compositor::dialog::handler::DialogHandler;
use smearor_wrot_compositor::popup::handler::PopupHandler;
use smearor_wrot_compositor::subsurface::handler::SubsurfaceHandler;
use smearor_wrot_compositor::surface::SurfaceQuery;
use smearor_wrot_model::geometry::size::Size;
use std::sync::atomic::Ordering;
use tracing::debug;

pub mod background_color;
pub mod error_renderer;

impl CompositorWidgetImpl {
    pub(crate) fn render_snapshot(&self, snapshot: &gtk4::Snapshot) {
        // Cleanup destroyed DMA-BUF textures from registry
        self.cleanup_dmabuf_registry();

        // Check for application error and show visual feedback
        let application_error = self.application_error.borrow();
        if let Some(error) = application_error.as_ref() {
            self.render_error_feedback(snapshot, error);
            return;
        }

        let compositor = self.compositor.borrow();
        let Some(compositor) = compositor.as_ref() else {
            return;
        };

        {
            let Ok(compositor) = compositor.lock() else {
                return;
            };
            let element_count = compositor.count_elements();
            if element_count == 0 {
                return;
            }
            debug!("Snapshot: rendering {} surfaces", element_count);
        }
        {
            let Ok(compositor) = compositor.lock() else {
                return;
            };
            let widget = self.obj();
            let widget_width = widget.width();
            let widget_height = widget.height();
            let size = Size::new(widget_width, widget_height);

            // Render toplevel windows
            for window in compositor.space.elements() {
                if window.toplevel().is_none() {
                    continue;
                }
                debug!("Snapshot: rendering window with toplevel");

                // Get window geometry and position in space
                let window_geometry = window.geometry();
                let window_location = compositor.space.element_location(window);
                debug!("Snapshot: window geometry: {:?}", window_geometry);
                debug!("Snapshot: window location in space: {:?}", window_location);

                // Use window position from space, negate geometry offset
                let (render_x, render_y) = if let Some(loc) = window_location {
                    let x = (loc.x as f32) - (window_geometry.loc.x as f32);
                    let y = (loc.y as f32) - (window_geometry.loc.y as f32);
                    (x, y)
                } else {
                    // Fallback to negating geometry offset if position not available
                    (-(window_geometry.loc.x as f32), -(window_geometry.loc.y as f32))
                };

                debug!("Snapshot: using render offset ({}, {})", render_x, render_y);

                // Get opacity and background color from compositor
                let opacity = compositor.get_opacity();

                // Check if there are active modal dialogs
                let has_active_dialogs = compositor.has_active_dialogs();

                // Apply opacity to entire compositor widget if not fully opaque
                if opacity < 1.0 {
                    snapshot.push_opacity(opacity as f64);
                    debug!("Snapshot: applied opacity {} to entire widget", opacity);
                }

                // Apply background color if set
                self.apply_background_color(&snapshot, &compositor, size);

                // Always render toplevel window (dimmed if dialogs are active)
                if let Some(texture) = self.render_window_to_texture(window, &*compositor, widget_width, widget_height) {
                    let texture_width = texture.width();
                    let texture_height = texture.height();
                    debug!("Snapshot: successfully rendered window to texture, size: {}x{}", texture_width, texture_height);

                    // // Check if texture size has changed compared to last frame
                    // let last_size = *self.last_texture_size.borrow();
                    // let size_changed = match last_size {
                    //     Some((last_w, last_h)) => (last_w - texture_width).abs() > 2 || (last_h - texture_height).abs() > 2,
                    //     None => true,
                    // };
                    //
                    // if size_changed {
                    //     debug!("Snapshot: texture size changed from {:?} to {}x{}", last_size, texture_width, texture_height);
                    //     *self.last_texture_size.borrow_mut() = Some((texture_width, texture_height));
                    //
                    //     // Don't trigger queue_resize() to avoid blocking resize handle
                    //     // The hysteresis of 2 pixels should prevent bounce effect
                    //     self.obj().queue_resize();
                    // }

                    let texture_width_as_float = texture_width as f32;
                    let texture_height_as_float = texture_height as f32;

                    // Check if color masking is enabled
                    let color_mask = compositor.get_color_mask();
                    let auto_color_mask = compositor.get_auto_color_mask();
                    let color_mask_detected = compositor.is_color_mask_detected();

                    if compositor.color_mask_shader.load(Ordering::Relaxed) {
                        // Apply color mask using DmaBufColorMaskApplier if enabled
                        if let Some(mask_color) = color_mask {
                            debug!("Snapshot: applying color mask using DmaBufColorMaskApplier");

                            // Get or create the DMA-BUF color mask applier
                            let mut applier = self.dma_buf_color_mask_applier.borrow_mut();
                            if applier.is_none() {
                                *applier = Some(DmaBufColorMaskApplier::new());
                            }

                            if let Some(applier) = applier.as_mut() {
                                let rendering_bounds = gtk4::graphene::Rect::new(render_x, render_y, texture_width_as_float, texture_height_as_float);
                                if let Err(e) = applier.apply_color_mask(&texture, mask_color, snapshot, &rendering_bounds) {
                                    debug!("Snapshot: failed to apply color mask: {}, falling back to normal rendering", e);
                                    // Fall through to normal rendering
                                } else {
                                    debug!("Snapshot: successfully applied color mask shader");
                                    // Skip normal rendering since shader was applied
                                    continue;
                                }
                            }
                        } else if auto_color_mask && color_mask_detected {
                            debug!("Snapshot: auto color mask is enabled and detected, but no mask color set");
                        }
                    } else {
                        // Apply color mask using trait-based applier if enabled - DISABLED for debugging
                        // CPU-based color masking is done in dmabuf/render_node.rs SHM fallback path
                        if color_mask.is_some() || (auto_color_mask && color_mask_detected) {
                            debug!("Snapshot: color mask is enabled, but DISABLED - using CPU-based masking in SHM fallback");
                            // Fall through to normal rendering below
                        }
                    }

                    // Fallback to normal rendering if shader not available or masking disabled
                    // Apply dimming overlay if dialogs are active
                    if has_active_dialogs {
                        snapshot.push_opacity(0.5);
                        debug!("Snapshot: applied dimming (opacity 0.5) to application window");
                    }

                    let rendering_bounds = gtk4::graphene::Rect::new(render_x, render_y, texture_width_as_float, texture_height_as_float);

                    snapshot.append_texture(&texture, &rendering_bounds);

                    debug!("Snapshot: appended texture to snapshot with bounds: {:?}", rendering_bounds);

                    // Pop opacity after rendering (both dimming and global opacity)
                    if has_active_dialogs {
                        snapshot.pop();
                    }

                    // Pop global opacity after all rendering is complete
                    if opacity < 1.0 {
                        snapshot.pop();
                    }

                    break;
                } else {
                    debug!("Snapshot: failed to render window to texture");
                }
            }

            // TODO: Phase 5 - Popup Surface Rendering - Render popups
            // Render popup surfaces like menus and tooltips
            let popups = compositor.get_all_popups();
            if !popups.is_empty() {
                debug!("Snapshot: rendering {} popups", popups.len());
                for (popup, position) in popups {
                    debug!("Snapshot: rendering popup at position {:?}", position);
                    if let Some(texture) = self.render_popup_to_texture(&popup, &*compositor) {
                        let texture_width = texture.width();
                        let texture_height = texture.height();
                        debug!("Snapshot: successfully rendered popup to texture, size: {}x{}", texture_width, texture_height);

                        // Calculate popup position relative to parent window
                        // The position from PopupManager is relative to the toplevel surface
                        // We need to add the window position in space to get the final position
                        let mut popup_x = position.x as f32;
                        let mut popup_y = position.y as f32;

                        // Find the parent window and add its position in space
                        for window in compositor.space.elements() {
                            if let Some(toplevel) = window.toplevel() {
                                let toplevel_surface = toplevel.wl_surface();
                                // Check if this window is the parent of the popup
                                if let Ok(popup_root_surface) = smithay::desktop::find_popup_root_surface(&popup) {
                                    if toplevel_surface == &popup_root_surface {
                                        if let Some(window_location) = compositor.space.element_location(window) {
                                            let window_geometry = window.geometry();
                                            // Add window position and subtract geometry offset
                                            popup_x += (window_location.x - window_geometry.loc.x) as f32;
                                            popup_y += (window_location.y - window_geometry.loc.y) as f32;
                                            debug!("Snapshot: adjusted popup position to ({}, {})", popup_x, popup_y);
                                        }
                                    }
                                }
                            }
                        }

                        let texture_width_as_float = texture_width as f32;
                        let texture_height_as_float = texture_height as f32;

                        let popup_bounds = gtk4::graphene::Rect::new(popup_x, popup_y, texture_width_as_float, texture_height_as_float);

                        snapshot.append_texture(&texture, &popup_bounds);

                        debug!("Snapshot: appended popup texture to snapshot with bounds: {:?}", popup_bounds);
                    } else {
                        debug!("Snapshot: failed to render popup to texture");
                    }
                }
            }

            // TODO: Phase 5 - Subsurface Rendering - Render subsurfaces
            // Render subsurface-based popups (e.g., GTK4 native popups)
            let subsurfaces = compositor.get_all_subsurfaces();
            if !subsurfaces.is_empty() {
                debug!("Snapshot: rendering {} subsurfaces", subsurfaces.len());
                for subsurface_position_data in subsurfaces {
                    debug!("Snapshot: rendering subsurface at position {:?}", subsurface_position_data.position);
                    if let Some(texture) = self.render_subsurface_to_texture(&subsurface_position_data.subsurface, &*compositor) {
                        let texture_size = Size::from(&texture);
                        debug!("Snapshot: successfully rendered subsurface to texture, size: {texture_size}");

                        // Calculate absolute subsurface position on screen by recursively tracing
                        // the parent chain up to the root window/popup mapped in space.
                        // If the parent is not mapped or visible, skip rendering this subsurface entirely.
                        let Some(parent_pos) = compositor.find_surface_absolute_position(&subsurface_position_data.parent) else {
                            use smithay::reexports::wayland_server::Resource;
                            debug!(
                                "Snapshot: parent of subsurface {:?} is not mapped/visible, skipping rendering",
                                subsurface_position_data.subsurface.id()
                            );
                            continue;
                        };

                        let subsurface_position = parent_pos + subsurface_position_data.position;
                        debug!("Snapshot: recursively calculated subsurface absolute position: {}", subsurface_position);

                        let subsurface_bounds = subsurface_position.rect(texture_size);

                        snapshot.append_texture(&texture, &subsurface_bounds);

                        debug!("Snapshot: appended subsurface texture to snapshot with bounds: {:?}", subsurface_bounds);
                    } else {
                        debug!("Snapshot: subsurface has no buffer, skipping subsurface rendering");
                    }
                }
            }
        }

        // TODO: Phase 6 - Dialog Management - Render modal dialogs
        // Render modal dialogs when active (only dialog, not application window)
        let dialogs = {
            let Ok(compositor) = compositor.lock() else {
                debug!("Failed to lock compositor for dialog rendering");
                return;
            };
            compositor.get_all_dialogs()
        };

        if !dialogs.is_empty() {
            debug!("Snapshot: rendering {} modal dialogs", dialogs.len());

            for dialog in dialogs {
                debug!("Snapshot: rendering dialog for surface");
                let Ok(compositor) = compositor.lock() else {
                    debug!("Failed to lock compositor for dialog rendering");
                    return;
                };
                if let Some(texture) = self.render_dialog_to_texture(&dialog, &*compositor) {
                    let texture_width = texture.width();
                    let texture_height = texture.height();
                    debug!("Snapshot: successfully rendered dialog to texture, size: {}x{}", texture_width, texture_height);

                    // let window_geometry = window.geometry();
                    // let window_location = compositor.space.element_location(window);
                    // debug!("Snapshot: window geometry: {:?}", window_geometry);
                    // debug!("Snapshot: window location in space: {:?}", window_location);
                    //
                    // // Use window position from space, negate geometry offset
                    // let (render_x, render_y) = if let Some(loc) = window_location {
                    //     let x = (loc.x as f32) - (window_geometry.loc.x as f32);
                    //     let y = (loc.y as f32) - (window_geometry.loc.y as f32);
                    //     (x, y)
                    // } else {
                    //     // Fallback to negating geometry offset if position not available
                    //     (-(window_geometry.loc.x as f32), -(window_geometry.loc.y as f32))
                    // };
                    //
                    // debug!("Snapshot: using render offset ({}, {})", render_x, render_y);

                    // Get dialog geometry offset (similar to toplevel rendering)
                    let (dialog_offset_x, dialog_offset_y) = {
                        let offset_x = 0;
                        let offset_y = 0;

                        if let Some(window) = compositor.space.elements().next() {
                            let window_geometry = window.geometry();
                            let window_location = compositor.space.element_location(window);

                            let (offset_x, offset_y) = if let Some(loc) = window_location {
                                let x = (loc.x as f32) - (window_geometry.loc.x as f32);
                                let y = (loc.y as f32) - (window_geometry.loc.y as f32);
                                (x, y)
                            } else {
                                // Fallback to negating geometry offset if position not available
                                (-(window_geometry.loc.x as f32), -(window_geometry.loc.y as f32))
                            };
                        }
                        (offset_x, offset_y)
                    };
                    debug!("Snapshot: using dialog render offset ({}, {})", dialog_offset_x, dialog_offset_y);

                    // Use original dialog size (client was configured with 80% limit via configure events)
                    let dialog_width = texture_width as i32;
                    let dialog_height = texture_height as i32;
                    debug!("Snapshot: dialog texture size: {}x{}", dialog_width, dialog_height);

                    // Calculate dialog position (centered)
                    let widget_width = self.obj().width() as i32;
                    let widget_height = self.obj().height() as i32;
                    debug!("Snapshot: widget size: {}x{}", widget_width, widget_height);
                    let (dialog_x, dialog_y) = compositor.calculate_dialog_position(widget_width, widget_height, dialog_width, dialog_height);
                    debug!("Snapshot: calculated dialog position: ({}, {})", dialog_x, dialog_y);

                    // Apply geometry offset correction
                    let dialog_x = dialog_x - dialog_offset_x;
                    let dialog_y = dialog_y - dialog_offset_y;
                    debug!("Snapshot: dialog position after offset correction: ({}, {})", dialog_x, dialog_y);

                    let dialog_x_float = dialog_x as f32;
                    let dialog_y_float = dialog_y as f32;
                    let dialog_width_float = dialog_width as f32;
                    let dialog_height_float = dialog_height as f32;

                    let dialog_bounds = gtk4::graphene::Rect::new(dialog_x_float, dialog_y_float, dialog_width_float, dialog_height_float);

                    snapshot.append_texture(&texture, &dialog_bounds);

                    debug!("Snapshot: appended dialog texture to snapshot with bounds: {:?}", dialog_bounds);
                } else {
                    debug!("Snapshot: failed to render dialog to texture");
                }
            }
        }
        {
            let Ok(mut compositor) = compositor.lock() else {
                return;
            };
            compositor.resolve_surface_and_clear_surface_damage();
        }
    }
}
