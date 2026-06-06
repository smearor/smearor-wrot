use crate::CompositorWidget;
use crate::widget::config::handler::ConfigHandler;
use crate::widget::imp::widget::CompositorWidgetImpl;
use gtk4::HeaderBar;
use gtk4::Label;
use gtk4::prelude::WidgetExt;

impl CompositorWidgetImpl {
    pub(crate) fn setup_header_bar(&self, obj: &CompositorWidget) {
        let config = self.config();
        if config.show_decorations {
            let header_bar = HeaderBar::builder().show_title_buttons(true).build();
            let title = config.title.unwrap_or_else(|| "Smearor Compositor".to_string());
            let label = Label::builder().label(&title).build();
            header_bar.set_title_widget(Some(&label));
            header_bar.set_parent(obj);
            *self.header_bar.borrow_mut() = Some(header_bar);
            *self.header_bar_title_label.borrow_mut() = Some(label);
        }
    }

    pub(crate) fn update_header_bar_title(&self, title: &str) {
        let label_ref = self.header_bar_title_label.borrow();
        if let Some(label) = label_ref.as_ref() {
            if label.label() != title {
                label.set_label(title);
            }
        }
    }
}
