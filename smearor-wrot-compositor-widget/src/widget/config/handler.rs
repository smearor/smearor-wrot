use crate::CompositorWidget;
use crate::CompositorWidgetConfig;
use crate::widget::config::error::ConfigError;
use glib::subclass::prelude::ObjectSubclassIsExt;
use gtk4::Orientation;
use smearor_wrot_model::geometry::size::Size;

pub trait ConfigHandler {
    fn set_config(&self, config: CompositorWidgetConfig);
    fn config(&self) -> CompositorWidgetConfig;

    fn apply_config_to_compositor(&self) -> Result<(), ConfigError>;

    fn min_size_by_orientation(&self, orientation: gtk4::Orientation) -> i32;

    fn min_size(&self) -> Size<i32>;
}

impl ConfigHandler for CompositorWidget {
    fn set_config(&self, config: CompositorWidgetConfig) {
        self.imp().set_config(config)
    }

    fn config(&self) -> CompositorWidgetConfig {
        self.imp().config()
    }

    fn apply_config_to_compositor(&self) -> Result<(), ConfigError> {
        self.imp().apply_config_to_compositor()
    }

    fn min_size_by_orientation(&self, orientation: Orientation) -> i32 {
        self.imp().min_size_by_orientation(orientation)
    }

    fn min_size(&self) -> Size<i32> {
        self.imp().min_size()
    }
}
