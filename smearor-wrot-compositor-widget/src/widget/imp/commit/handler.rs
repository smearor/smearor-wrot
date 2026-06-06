use crate::widget::commit::CommitHandler;
use crate::widget::compositor::handler::CompositorHandler;
use crate::widget::imp::widget::CompositorWidgetImpl;
use smearor_wrot_compositor::commit::count::CommitCount;

impl CommitHandler for CompositorWidgetImpl {
    fn get_first_toplevel_commit_count(&self) -> u32 {
        let Ok(compositor) = self.compositor() else {
            return 0;
        };
        let Ok(guard) = compositor.lock() else {
            return 0;
        };
        guard.get_first_toplevel_commit_count()
    }
}
