use crate::Margins;
use smearor_wrot_model_geometry::Size;
use std::sync::atomic::Ordering;

pub trait MarginStateAccessor {
    fn margin_left(&self) -> u32;
    fn margin_right(&self) -> u32;
    fn margin_top(&self) -> u32;
    fn margin_bottom(&self) -> u32;
    fn dialog_margin(&self) -> u32;
    fn margin_horizontal(&self) -> u32;
    fn margin_vertical(&self) -> u32;
    fn margins(&self) -> Margins;
    fn margin_size(&self) -> Size<u32>;
    fn set_margin_left(&self, margin_left: u32);
    fn set_margin_right(&self, margin_right: u32);
    fn set_margin_top(&self, margin_top: u32);
    fn set_margin_bottom(&self, margin_bottom: u32);
    fn set_dialog_margin(&self, dialog_margin: u32);
}
