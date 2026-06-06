use gtk4::prelude::*;
use gtk4::Window;
use gtk4::Widget;
use smearor_wrot_compositor_widget::CompositorWidget;
use std::sync::Arc;
use tracing::debug;
use crate::settings::dialog::SettingsDialog;
use crate::settings::manager::SettingsManager;

pub struct SettingsHandler {
    parent_window: gtk4::ApplicationWindow,
    compositor_widget: CompositorWidget,
    rotation_widget: Widget,
    disable_dma_buf: bool,
    dialog: Option<Window>,
}

impl SettingsHandler {
    pub fn new(
        parent_window: &gtk4::ApplicationWindow,
        compositor_widget: &CompositorWidget,
        rotation_widget: &Widget,
        disable_dma_buf: bool,
    ) -> Self {
        Self {
            parent_window: parent_window.clone(),
            compositor_widget: compositor_widget.clone(),
            rotation_widget: rotation_widget.clone(),
            disable_dma_buf,
            dialog: None,
        }
    }

    pub fn open_settings_dialog(&mut self) {
        debug!("Opening settings dialog");

        if self.dialog.is_some() {
            debug!("Settings dialog already open, returning");
            return;
        }

        let settings_manager = Arc::new(SettingsManager::new(
            &self.compositor_widget,
            &self.parent_window,
            &self.rotation_widget,
            self.disable_dma_buf,
        ));

        let settings_dialog = SettingsDialog::new(&self.parent_window, settings_manager);
        let dialog = settings_dialog.build();

        dialog.present();
        self.dialog = Some(dialog);
    }

    pub fn close_settings_dialog(&mut self) {
        debug!("Closing settings dialog");

        if let Some(dialog) = self.dialog.take() {
            dialog.close();
        }
    }
}
