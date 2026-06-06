use crate::widget::imp::widget::CompositorWidgetImpl;
use gtk4::Snapshot;
use gtk4::prelude::SnapshotExt;
use smearor_wrot_compositor::SmearorCompositor;
use smearor_wrot_compositor::background::toplevel::ToplevelBackground;
use smearor_wrot_model::geometry::size::Size;
use std::sync::MutexGuard;
use tracing::debug;

pub trait SnapshotBackgroundColor {
    /// Apply background color if set
    fn apply_background_color(&self, snapshot: &Snapshot, compositor: &MutexGuard<SmearorCompositor>, widget_size: Size<i32>);
}

impl SnapshotBackgroundColor for CompositorWidgetImpl {
    fn apply_background_color(&self, snapshot: &Snapshot, compositor: &MutexGuard<SmearorCompositor>, widget_size: Size<i32>) {
        let Some(background_color) = compositor.get_background_color() else {
            return;
        };
        let bg_color = background_color.into();
        let bg_rect = Size::<f32>::from(widget_size).rect_from_coordinates(0.0, 0.0);
        snapshot.append_color(&bg_color, &bg_rect);
        debug!("Snapshot: applied background color {}", background_color);
    }
}
