use crate::CompositorWidget;
use crate::widget::config::handler::ConfigHandler;
use crate::widget::imp::widget::CompositorWidgetImpl;
use gtk4::prelude::WidgetExt;

impl CompositorWidgetImpl {
    pub(crate) fn setup_widget_config(&self, obj: &CompositorWidget) {
        let config = self.config();
        obj.set_margin_top(config.margin_top);
        obj.set_margin_bottom(config.margin_bottom);
        obj.set_margin_start(config.margin_start);
        obj.set_margin_end(config.margin_end);
    }
}
