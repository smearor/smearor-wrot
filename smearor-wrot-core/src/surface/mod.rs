//! Surface management
//!
//! This module provides traits and implementations for surface management
//! including query, mapping, stacking, and caching operations.

pub mod cache;
pub mod cleanup;
pub mod commit;
pub mod dialog;
pub mod mapping;
pub mod query;
pub mod stacking;

pub use cache::SurfaceCache;
pub use mapping::SurfaceMapping;
pub use query::SurfaceQuery;
pub use stacking::SurfaceStacking;
