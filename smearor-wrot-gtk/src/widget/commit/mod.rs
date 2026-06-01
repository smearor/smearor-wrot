use crate::CompositorWidget;
use glib::subclass::prelude::ObjectSubclassIsExt;

pub trait CommitHandler {
    fn get_first_toplevel_commit_count(&self) -> u32;
}

impl CommitHandler for CompositorWidget {
    fn get_first_toplevel_commit_count(&self) -> u32 {
        self.imp().get_first_toplevel_commit_count()
    }
}
