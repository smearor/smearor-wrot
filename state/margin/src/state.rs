use crate::Margins;
use crate::accessor::MarginStateAccessor;
use smearor_wrot_model_geometry::Size;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering;
use typed_builder::TypedBuilder;

#[derive(Debug, TypedBuilder)]
pub struct MarginState {
    #[builder(default)]
    pub margin_left: AtomicU32,

    #[builder(default)]
    pub margin_right: AtomicU32,

    #[builder(default)]
    pub margin_top: AtomicU32,

    #[builder(default)]
    pub margin_bottom: AtomicU32,

    #[builder(default)]
    pub dialog_margin: AtomicU32,
}

impl MarginStateAccessor for MarginState {
    fn margin_left(&self) -> u32 {
        self.margin_left.load(Ordering::Relaxed)
    }

    fn margin_right(&self) -> u32 {
        self.margin_right.load(Ordering::Relaxed)
    }

    fn margin_top(&self) -> u32 {
        self.margin_top.load(Ordering::Relaxed)
    }

    fn margin_bottom(&self) -> u32 {
        self.margin_bottom.load(Ordering::Relaxed)
    }

    fn dialog_margin(&self) -> u32 {
        self.dialog_margin.load(Ordering::Relaxed)
    }

    fn margin_horizontal(&self) -> u32 {
        self.margin_left() + self.margin_right()
    }

    fn margin_vertical(&self) -> u32 {
        self.margin_top() + self.margin_bottom()
    }

    fn margins(&self) -> Margins {
        Margins::new(self.margin_left(), self.margin_right(), self.margin_top(), self.margin_bottom())
    }

    fn margin_size(&self) -> Size<u32> {
        Size::new(self.margin_horizontal(), self.margin_vertical())
    }

    fn set_margin_left(&self, margin_left: u32) {
        self.margin_left.store(margin_left, Ordering::Relaxed);
    }

    fn set_margin_right(&self, margin_right: u32) {
        self.margin_right.store(margin_right, Ordering::Relaxed);
    }

    fn set_margin_top(&self, margin_top: u32) {
        self.margin_top.store(margin_top, Ordering::Relaxed);
    }

    fn set_margin_bottom(&self, margin_bottom: u32) {
        self.margin_bottom.store(margin_bottom, Ordering::Relaxed);
    }

    fn set_dialog_margin(&self, dialog_margin: u32) {
        self.dialog_margin.store(dialog_margin, Ordering::Relaxed);
    }
}
