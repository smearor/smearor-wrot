//! Wayland Output protocol handler

use smithay::wayland::output::OutputHandler;

use crate::compositor::SmearorCompositor;

impl OutputHandler for SmearorCompositor {}

smithay::delegate_output!(SmearorCompositor);
