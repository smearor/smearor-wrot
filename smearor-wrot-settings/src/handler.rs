use crate::SettingsManager;
use gtk4::ApplicationWindow;
use gtk4::Widget;
use gtk4::Window;
use gtk4::prelude::*;
// use smearor_wrot_compositor_widget::CompositorWidget;
use std::sync::Arc;
use tracing::debug;
use tracing::info;

pub struct SettingsHandler {
    settings_manager: Arc<SettingsManager>,
    // parent_window: ApplicationWindow,
    // compositor_widget: CompositorWidget,
    // rotation_widget: Widget,
    // disable_dma_buf: bool,
    dialog: Option<Window>,
}

impl SettingsHandler {
    pub fn new(parent_window: &ApplicationWindow, compositor_widget: &CompositorWidget, rotation_widget: &Widget, disable_dma_buf: bool) -> Self {
        Self {
            parent_window: parent_window.clone(),
            compositor_widget: compositor_widget.clone(),
            rotation_widget: rotation_widget.clone(),
            disable_dma_buf,
            dialog: None,
        }
    }

    pub fn open_settings_dialog(&mut self) {
        info!("Opening settings dialog");

        if self.dialog.is_some() {
            info!("Settings dialog already open, returning");
            return;
        }

        let s = SettingsManager::builder()
            .compositor_widget(self.compositor_widget)
            .debug_overlay(debug_overlay)
            .build();
        let settings_manager = Arc::new(s);
        let settings_manager = Arc::new(SettingsManager::new(
            &self.compositor_widget,
            &self.parent_window,
            &self.rotation_widget,
            self.disable_dma_buf,
        ));

        let settings_dialog = SettingsDialog::new(&self.parent_window, settings_manager);
        let dialog = settings_dialog.build();
        // dialog.connect_destroy(|a| {
        //     self.close_settings_dialog();
        // });

        dialog.present();
        self.dialog = Some(dialog);
    }

    pub fn close_settings_dialog(&mut self) {
        info!("Closing settings dialog");

        if let Some(dialog) = self.dialog.take() {
            dialog.close();
        }
    }
}
