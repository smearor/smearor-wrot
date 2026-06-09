use crate::MarginState;
use crate::Margins;
use crate::accessor::MarginStateAccessor;
use smearor_wrot_model_geometry::Size;
use std::sync::Arc;

pub struct MarginManager {
    /// The state of the margin manager
    pub state: Arc<MarginState>,
}

impl MarginStateAccessor for MarginManager {
    fn margin_left(&self) -> u32 {
        self.state.margin_left()
    }

    fn margin_right(&self) -> u32 {
        self.state.margin_right()
    }

    fn margin_top(&self) -> u32 {
        self.state.margin_top()
    }

    fn margin_bottom(&self) -> u32 {
        self.state.margin_bottom()
    }

    fn dialog_margin(&self) -> u32 {
        self.state.dialog_margin()
    }

    fn margin_horizontal(&self) -> u32 {
        self.state.margin_horizontal()
    }

    fn margin_vertical(&self) -> u32 {
        self.state.margin_vertical()
    }

    fn margins(&self) -> Margins {
        self.state.margins()
    }

    fn margin_size(&self) -> Size<u32> {
        self.state.margin_size()
    }

    fn set_margin_left(&self, margin_left: u32) {
        self.state.set_margin_left(margin_left);
    }

    fn set_margin_right(&self, margin_right: u32) {
        self.state.set_margin_right(margin_right);
    }

    fn set_margin_top(&self, margin_top: u32) {
        self.state.set_margin_top(margin_top);
    }

    fn set_margin_bottom(&self, margin_bottom: u32) {
        self.state.set_margin_bottom(margin_bottom);
    }

    fn set_dialog_margin(&self, dialog_margin: u32) {
        self.state.set_dialog_margin(dialog_margin);
    }
}
