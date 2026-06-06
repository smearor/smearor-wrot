//! Buffer management
//!
//! This module provides buffer management for the compositor including
//! buffer import/export, damage tracking, and lifecycle management.

pub mod import_export;
pub mod lifecycle;
pub mod metadata;
pub mod tracking;

pub use import_export::BufferImportExport;
pub use lifecycle::BufferLifecycle;
pub use tracking::BufferTracking;
