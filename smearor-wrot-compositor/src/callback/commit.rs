use crate::SmearorCompositor;
use smithay::reexports::wayland_server::backend::ObjectId;
use std::sync::Arc;

pub type CommitCallback = Arc<dyn Fn(ObjectId)>;

pub trait CommitCallbackAware {
    fn set_commit_callback(&self, callback: CommitCallback);
}

impl CommitCallbackAware for SmearorCompositor {
    fn set_commit_callback(&self, callback: CommitCallback) {
        if let Ok(mut cb) = self.commit_callback.lock() {
            *cb = Some(callback);
        }
    }
}
