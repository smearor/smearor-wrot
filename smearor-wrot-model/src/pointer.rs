use crate::RgbaColor;
use crate::geometry::position::Position;
use gtk4::Snapshot;
#[cfg(feature = "gtk4")]
use gtk4::gdk::RGBA;
#[cfg(feature = "gtk4")]
use gtk4::graphene::Rect;
use gtk4::prelude::SnapshotExt;
use std::fmt::Debug;

pub const DEFAULT_POINTER_SIZE: f32 = 40.0;
pub const DEFAULT_POINTER_BORDER_WIDTH: f32 = 2.0;

pub const DEFAULT_GTK_POINTER_COLOR: RgbaColor = RgbaColor::with_rgb(0.0, 0.0, 1.0, 1.0);
pub const DEFAULT_APP_POINTER_COLOR: RgbaColor = RgbaColor::with_rgb(0.0, 1.0, 1.0, 1.0);

pub const DEFAULT_GTK_TOUCH_COLOR: RgbaColor = RgbaColor::with_rgb(1.0, 0.0, 0.0, 1.0);
pub const DEFAULT_APP_TOUCH_COLOR: RgbaColor = RgbaColor::with_rgb(0.0, 1.0, 0.0, 1.0);

#[derive(Debug, Clone, Copy)]
pub struct PointerPosition<T: Debug + Clone + Copy> {
    pub gtk_pos: Position<T>,
    pub app_pos: Position<T>,
    pub size: T,
    pub gtk_color: RgbaColor,
    pub app_color: RgbaColor,
    pub border_width: T,
}

impl<T: Debug + Clone + Copy> PointerPosition<T> {
    pub fn new(gtk_pos: Position<T>, app_pos: Position<T>, size: T, gtk_color: RgbaColor, app_color: RgbaColor, border_width: T) -> Self {
        Self {
            gtk_pos,
            app_pos,
            size,
            gtk_color,
            app_color,
            border_width,
        }
    }
}

#[cfg(feature = "gtk4")]
impl PointerPosition<f32> {
    pub fn new_pointer(gtk_pos: Position<f32>, app_pos: Position<f32>) -> Self {
        Self::new(
            gtk_pos,
            app_pos,
            DEFAULT_POINTER_SIZE,
            DEFAULT_GTK_POINTER_COLOR,
            DEFAULT_APP_POINTER_COLOR,
            DEFAULT_POINTER_BORDER_WIDTH,
        )
    }

    pub fn new_touch(gtk_pos: Position<f32>, app_pos: Position<f32>) -> Self {
        Self::new(
            gtk_pos,
            app_pos,
            DEFAULT_POINTER_SIZE,
            DEFAULT_GTK_TOUCH_COLOR,
            DEFAULT_APP_TOUCH_COLOR,
            DEFAULT_POINTER_BORDER_WIDTH,
        )
    }

    pub fn gtk_rect(&self) -> Rect {
        Rect::new(self.gtk_pos.x - self.size / 2.0, self.gtk_pos.y - self.size / 2.0, self.size, self.size)
    }

    pub fn app_rect(&self) -> Rect {
        Rect::new(self.app_pos.x - self.size / 2.0, self.app_pos.y - self.size / 2.0, self.size, self.size)
    }

    pub fn app_top(&self) -> Rect {
        let app_bounds = self.app_rect();
        Rect::new(app_bounds.x(), app_bounds.y(), app_bounds.width(), self.border_width)
    }

    pub fn app_bottom(&self) -> Rect {
        let app_bounds = self.app_rect();
        Rect::new(
            app_bounds.x(),
            app_bounds.y() + app_bounds.height() - self.border_width,
            app_bounds.width(),
            self.border_width,
        )
    }

    pub fn app_left(&self) -> Rect {
        let app_bounds = self.app_rect();
        Rect::new(app_bounds.x(), app_bounds.y(), self.border_width, app_bounds.height())
    }

    pub fn app_right(&self) -> Rect {
        let app_bounds = self.app_rect();
        Rect::new(
            app_bounds.x() + app_bounds.width() - self.border_width,
            app_bounds.y(),
            self.border_width,
            app_bounds.height(),
        )
    }

    pub fn gtk_color(&self) -> RGBA {
        self.gtk_color.into()
    }

    pub fn app_color(&self) -> RGBA {
        self.app_color.into()
    }

    pub fn render_snapshot(&self, snapshot: &Snapshot) {
        snapshot.append_color(&self.gtk_color(), &self.gtk_rect());

        let app_color = self.app_color();
        snapshot.append_color(&app_color, &self.app_top());
        snapshot.append_color(&app_color, &self.app_bottom());
        snapshot.append_color(&app_color, &self.app_left());
        snapshot.append_color(&app_color, &self.app_right());
    }
}
