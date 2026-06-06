use std::hash::Hash;
use std::hash::Hasher;
use typed_builder::TypedBuilder;

pub const DEFAULT_MENU_ITEM_RADIUS: f32 = 40.0;

#[derive(Debug, Clone, TypedBuilder)]
pub struct MenuItem {
    /// The id
    #[builder(setter(into))]
    pub id: String,
    /// The label of the menu item
    #[builder(setter(into))]
    pub label: String,
    /// The color of the label
    #[builder(default = String::from("#ffffffff"), setter(into))]
    pub label_color: String,
    /// The icon of the menu item
    #[builder(setter(into))]
    pub icon_name: String,
    /// The color of the icon
    #[builder(default = String::from("#77777777"), setter(into))]
    pub color: String,
    /// The angle of the menu item in degrees
    pub angle: f32,
    /// The angle of the menu item in degrees
    #[builder(default, setter(into, strip_option))]
    pub radius: Option<f32>,
    /// The name of the event to be triggered when the menu item is selected.
    #[builder(setter(into))]
    pub event: String,
}

impl MenuItem {
    pub fn radius(&self) -> f32 {
        self.radius.unwrap_or(DEFAULT_MENU_ITEM_RADIUS)
    }
}

impl Hash for MenuItem {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for MenuItem {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for MenuItem {}
