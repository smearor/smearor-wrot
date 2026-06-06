use crate::SmearorCompositor;
use crate::surface::SurfaceQuery;
use smithay::reexports::wayland_server::backend::ObjectId;
use std::sync::Arc;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering;

pub trait CommitCount {
    /// Get commit count for a specific surface
    fn get_commit_count(&self, surface_id: ObjectId) -> u32;

    /// Get the commit count of the first toplevel surface
    fn get_first_toplevel_commit_count(&self) -> u32;

    /// Increment commit count for a specific surface
    fn increment_commit_count(&self, surface_id: ObjectId);

    /// Reset commit count for a specific surface
    fn reset_commit_count(&self, surface_id: ObjectId);
}

impl CommitCount for SmearorCompositor {
    fn get_commit_count(&self, surface_id: ObjectId) -> u32 {
        self.commit_counts.get(&surface_id).map(|count| count.load(Ordering::Relaxed)).unwrap_or(0)
    }

    fn get_first_toplevel_commit_count(&self) -> u32 {
        self.get_first_toplevel_surface_id()
            .map(|surface_id| self.get_commit_count(surface_id))
            .unwrap_or_default()
    }

    fn increment_commit_count(&self, surface_id: ObjectId) {
        let count = self.commit_counts.entry(surface_id).or_insert_with(|| Arc::new(AtomicU32::new(0)));
        count.fetch_add(1, Ordering::Relaxed);
    }

    fn reset_commit_count(&self, surface_id: ObjectId) {
        if let Some(count) = self.commit_counts.get(&surface_id) {
            count.store(0, Ordering::Relaxed);
        }
    }
}
