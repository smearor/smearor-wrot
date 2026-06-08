use crate::MarginConfig;
use smearor_wrot_geometry::Size;
use std::sync::Arc;
use std::sync::atomic::Ordering;

pub struct MarginManager {
    pub margin_config: Arc<MarginConfig>,
}

impl MarginManager {
    pub fn margin_left(&self) -> u32 {
        self.margin_config.margin_left.load(Ordering::Relaxed)
    }

    pub fn margin_right(&self) -> u32 {
        self.margin_config.margin_right.load(Ordering::Relaxed)
    }

    pub fn margin_top(&self) -> u32 {
        self.margin_config.margin_top.load(Ordering::Relaxed)
    }

    pub fn margin_bottom(&self) -> u32 {
        self.margin_config.margin_bottom.load(Ordering::Relaxed)
    }

    pub fn dialog_margin(&self) -> u32 {
        self.margin_config.dialog_margin.load(Ordering::Relaxed)
    }

    pub fn margin_horizontal(&self) -> u32 {
        self.margin_left() + self.margin_right()
    }

    pub fn margin_vertical(&self) -> u32 {
        self.margin_top() + self.margin_bottom()
    }

    pub fn margin_size(&self) -> Size<u32> {
        Size::new(self.margin_horizontal(), self.margin_vertical())
    }

    pub fn set_margin_left(&self, margin_left: u32) {
        self.margin_config.margin_left.store(margin_left, Ordering::Relaxed);
    }

    pub fn set_margin_right(&self, margin_right: u32) {
        self.margin_config.margin_right.store(margin_right, Ordering::Relaxed);
    }

    pub fn set_margin_top(&self, margin_top: u32) {
        self.margin_config.margin_top.store(margin_top, Ordering::Relaxed);
    }

    pub fn set_margin_bottom(&self, margin_bottom: u32) {
        self.margin_config.margin_bottom.store(margin_bottom, Ordering::Relaxed);
    }

    pub fn set_dialog_margin(&self, dialog_margin: u32) {
        self.margin_config.dialog_margin.store(dialog_margin, Ordering::Relaxed);
    }
}
