//! Damage tracking for compositor outputs
//!
//! This module provides traits and implementations for tracking damage regions
//! on compositor outputs. Damage tracking is essential for incremental rendering,
//! allowing the compositor to only redraw regions that have changed.

pub mod output;
pub mod surface;
