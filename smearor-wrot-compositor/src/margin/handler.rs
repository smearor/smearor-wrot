use crate::SmearorCompositor;
use smearor_wrot_geometry::Size;
use smearor_wrot_margin::Margins;
use std::sync::atomic::Ordering;
use tracing::debug;

pub trait MarginHandler {
    /// Set margins for window rendering
    fn set_margins(&self, margins: Margins);

    /// Get left margin
    fn get_margin_left(&self) -> u32;

    /// Get right margin
    fn get_margin_right(&self) -> u32;

    /// Get horizontal margin
    fn get_margin_horizontal(&self) -> u32;

    /// Get top margin
    fn get_margin_top(&self) -> u32;

    /// Get bottom margin
    fn get_margin_bottom(&self) -> u32;

    /// Get vertical margin
    fn get_margin_vertical(&self) -> u32;

    /// Get margin size
    fn get_margin_size(&self) -> Size<u32>;
}

impl MarginHandler for SmearorCompositor {
    /// Set margins for window rendering
    fn set_margins(&self, margins: Margins) {
        self.margin_left.store(margins.left, Ordering::Relaxed);
        self.margin_right.store(margins.right, Ordering::Relaxed);
        self.margin_top.store(margins.top, Ordering::Relaxed);
        self.margin_bottom.store(margins.bottom, Ordering::Relaxed);
        debug!("Margins set to {margins}");
    }

    fn get_margin_left(&self) -> u32 {
        self.margin_left.load(Ordering::Relaxed)
    }

    fn get_margin_right(&self) -> u32 {
        self.margin_right.load(Ordering::Relaxed)
    }

    fn get_margin_horizontal(&self) -> u32 {
        self.get_margin_left() + self.get_margin_right()
    }

    fn get_margin_top(&self) -> u32 {
        self.margin_top.load(Ordering::Relaxed)
    }

    fn get_margin_bottom(&self) -> u32 {
        self.margin_bottom.load(Ordering::Relaxed)
    }

    fn get_margin_vertical(&self) -> u32 {
        self.get_margin_top() + self.get_margin_bottom()
    }

    fn get_margin_size(&self) -> Size<u32> {
        Size::new(self.get_margin_horizontal(), self.get_margin_vertical())
    }
}
