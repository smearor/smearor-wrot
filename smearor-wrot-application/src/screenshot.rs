use directories::UserDirs;
use gtk4::Application;
use gtk4::prelude::ApplicationExt;
use smearor_wrot_compositor_widget::CompositorWidget;
use smearor_wrot_compositor_widget::widget::buffer::error::SaveBufferError;
use smearor_wrot_compositor_widget::widget::buffer::handler::BufferHandler;
use std::path::PathBuf;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

pub struct ScreenshotManager {
    application: Application,
    compositor_widget: CompositorWidget,
}

impl ScreenshotManager {
    pub fn new(application: Application, compositor_widget: CompositorWidget) -> Self {
        Self {
            application,
            compositor_widget,
        }
    }

    pub fn screenshot(&self) -> Result<PathBuf, SaveBufferError> {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();
        let mut path = UserDirs::new()
            .and_then(|u| u.picture_dir().map(|p| p.to_owned()))
            .unwrap_or_else(|| PathBuf::from("/tmp"));
        let app_id = self
            .application
            .application_id()
            .map(|s| s.to_string())
            .unwrap_or_else(|| "smearor-wrot".to_string());
        path.push(format!("{app_id}_{timestamp}.png"));
        self.compositor_widget.save_buffer_to_png(path)
    }
}
