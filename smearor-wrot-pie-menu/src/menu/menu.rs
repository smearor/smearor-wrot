use crate::menu::item::MenuItem;
use dashmap::DashMap;
use std::ops::Deref;

pub struct Menu(DashMap<String, MenuItem>);

impl Menu {
    pub fn new() -> Self {
        Self(DashMap::new())
    }

    pub fn builder() -> MenuBuilder {
        MenuBuilder::default()
    }
}

impl Deref for Menu {
    type Target = DashMap<String, MenuItem>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Default)]
pub struct MenuBuilder {
    items: DashMap<String, MenuItem>,
}

impl MenuBuilder {
    pub fn item(self, item: MenuItem) -> Self {
        self.items.insert(item.id.clone(), item);
        self
    }

    pub fn build(self) -> Menu {
        Menu(self.items)
    }
}
