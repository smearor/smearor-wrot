use crate::CompositorWidget;
use glib::subclass::prelude::ObjectSubclassIsExt;

pub trait ShmHandler {
    fn get_shm_render_count(&self) -> u32;
}

impl ShmHandler for CompositorWidget {
    fn get_shm_render_count(&self) -> u32 {
        self.imp().get_shm_render_count()
    }
}
