use crate::widget::imp::ApplicationError;
use crate::widget::imp::CompositorWidgetImpl;
use glib::subclass::prelude::ObjectSubclassExt;
use gtk4::Snapshot;
use gtk4::gdk::RGBA;
use gtk4::graphene::Point;
use gtk4::pango::FontDescription;
use gtk4::pango::Layout;
use gtk4::prelude::SnapshotExt;

pub trait ErrorRenderer {
    fn render_error_feedback(&self, snapshot: &Snapshot, error: &ApplicationError);

    fn render_fallback_warning_icon(&self, snapshot: &Snapshot, widget_width: f32, widget_height: f32);

    fn render_error_text(&self, snapshot: &Snapshot, program_name: &str, error_message: &str, widget_width: f32, widget_height: f32);
}

impl ErrorRenderer for CompositorWidgetImpl {
    fn render_error_feedback(&self, snapshot: &Snapshot, error: &ApplicationError) {
        use gtk4::prelude::WidgetExt;

        let widget = self.obj();
        let widget_width = widget.width() as f32;
        let widget_height = widget.height() as f32;

        // Get program name and error message
        let (program_name, error_message): (String, &str) = match error {
            ApplicationError::NotFound(name) => (name.clone(), "not found"),
            ApplicationError::NotSpecified => ("No program".to_string(), "not specified"),
        };

        // Render fallback warning icon
        self.render_fallback_warning_icon(snapshot, widget_width, widget_height);

        // Render text below icon
        self.render_error_text(snapshot, &program_name, error_message, widget_width, widget_height);
    }

    fn render_fallback_warning_icon(&self, snapshot: &Snapshot, widget_width: f32, widget_height: f32) {
        // Draw a simple warning triangle with exclamation mark
        let icon_size = 64.0;
        let center_x = widget_width / 2.0;
        let center_y = widget_height / 2.0 - 20.0;

        // Triangle points
        let top_x = center_x;
        let top_y = center_y - icon_size / 2.0;
        let bottom_left_x = center_x - icon_size / 2.0;
        let bottom_left_y = center_y + icon_size / 2.0;
        let bottom_right_x = center_x + icon_size / 2.0;
        let bottom_right_y = center_y + icon_size / 2.0;

        // Draw triangle outline (yellow/orange)
        let triangle_color = gtk4::gdk::RGBA::new(1.0, 0.8, 0.0, 1.0);

        // Draw triangle using three lines
        let line_width = 3.0;

        // Top to bottom-left
        let tl_rect = gtk4::graphene::Rect::new(
            bottom_left_x.min(top_x),
            bottom_left_y.min(top_y),
            (bottom_left_x - top_x).abs().max(line_width),
            (bottom_left_y - top_y).abs().max(line_width),
        );
        snapshot.append_color(&triangle_color, &tl_rect);

        // Top to bottom-right
        let tr_rect = gtk4::graphene::Rect::new(
            bottom_right_x.min(top_x),
            bottom_right_y.min(top_y),
            (bottom_right_x - top_x).abs().max(line_width),
            (bottom_right_y - top_y).abs().max(line_width),
        );
        snapshot.append_color(&triangle_color, &tr_rect);

        // Bottom-left to bottom-right
        let bl_rect = gtk4::graphene::Rect::new(
            bottom_left_x.min(bottom_right_x),
            bottom_left_y.min(bottom_right_y),
            (bottom_right_x - bottom_left_x).abs().max(line_width),
            line_width,
        );
        snapshot.append_color(&triangle_color, &bl_rect);

        // Draw exclamation mark in center
        let exclamation_color = gtk4::gdk::RGBA::new(0.0, 0.0, 0.0, 1.0);
        let exclamation_width = 6.0;
        let exclamation_height = 20.0;
        let exclamation_x = center_x - exclamation_width / 2.0;
        let exclamation_y = center_y - exclamation_height / 2.0;

        let exclamation_rect = gtk4::graphene::Rect::new(exclamation_x, exclamation_y, exclamation_width, exclamation_height);
        snapshot.append_color(&exclamation_color, &exclamation_rect);

        // Draw dot below exclamation
        let dot_size = 6.0;
        let dot_x = center_x - dot_size / 2.0;
        let dot_y = center_y + exclamation_height / 2.0 + 5.0;
        let dot_rect = gtk4::graphene::Rect::new(dot_x, dot_y, dot_size, dot_size);
        snapshot.append_color(&exclamation_color, &dot_rect);
    }

    fn render_error_text(&self, snapshot: &Snapshot, program_name: &str, error_message: &str, widget_width: f32, widget_height: f32) {
        use gtk4::prelude::WidgetExt;

        let widget = self.obj();
        let context = widget.create_pango_context();
        let layout = Layout::new(&context);

        // Configure font
        let font_description = FontDescription::from_string("Sans 14");
        layout.set_font_description(Some(&font_description));

        // Calculate text positions
        let icon_size = 64.0;
        let center_x = widget_width / 2.0;
        let center_y = widget_height / 2.0;

        // Line 1: program name
        layout.set_text(program_name);
        let (line1_width, line1_height) = layout.pixel_size();
        let line1_x = center_x - (line1_width as f32) / 2.0;
        let line1_y = center_y + icon_size / 2.0 + 20.0;

        // Render program name
        let text_color = RGBA::new(1.0, 1.0, 1.0, 1.0);
        snapshot.translate(&Point::new(line1_x, line1_y));
        snapshot.append_layout(&layout, &text_color);
        snapshot.translate(&Point::new(-line1_x, -line1_y));

        // Line 2: error message
        layout.set_text(error_message);
        let (line2_width, line2_height) = layout.pixel_size();
        let line2_x = center_x - (line2_width as f32) / 2.0;
        let line2_y = line1_y + line1_height as f32 + 5.0;

        // Render error message
        snapshot.translate(&Point::new(line2_x, line2_y));
        snapshot.append_layout(&layout, &text_color);
        snapshot.translate(&Point::new(-line2_x, -line2_y));
    }
}
