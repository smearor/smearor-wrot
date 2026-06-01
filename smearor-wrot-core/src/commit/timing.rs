use crate::SmearorCompositor;
use smithay::reexports::wayland_server::backend::ObjectId;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Instant;

pub trait CommitTiming {
    /// Record the first commit time for a surface
    fn record_first_commit_time(&self, surface_id: ObjectId);

    /// Check if enough time has passed since the first commit (default: 500ms)
    fn has_enough_time_since_first_commit(&self, surface_id: ObjectId, delay_ms: u64) -> bool;
}

impl CommitTiming for SmearorCompositor {
    fn record_first_commit_time(&self, surface_id: ObjectId) {
        self.first_commit_times
            .entry(surface_id)
            .or_insert_with(|| Arc::new(Mutex::new(Instant::now())));
    }

    fn has_enough_time_since_first_commit(&self, surface_id: ObjectId, delay_ms: u64) -> bool {
        let Some(time) = self.first_commit_times.get(&surface_id) else {
            return false;
        };
        let Ok(instant) = time.lock() else {
            return false;
        };
        instant.elapsed().as_millis() >= delay_ms as u128
    }
}
