use crate::menu::item::MenuItem;
use crate::menu::menu::Menu;

pub trait MenuProvider {
    fn get_menu_items() -> Menu;
}

pub struct EmptyMenuProvider;

impl MenuProvider for EmptyMenuProvider {
    fn get_menu_items() -> Menu {
        Menu::builder().build()
    }
}

pub struct DefaultMenuProvider;

impl MenuProvider for DefaultMenuProvider {
    fn get_menu_items() -> Menu {
        Menu::builder()
            .item(
                MenuItem::builder()
                    .id("rotate-cw")
                    .label("Rotate CW")
                    .icon_name("object-rotate-right-symbolic")
                    .color("#00000077")
                    .angle(0.0)
                    .radius(30.0)
                    .event("rotate-cw")
                    .build(),
            )
            .item(
                MenuItem::builder()
                    .id("screenshot")
                    .label("Screenshot")
                    .icon_name("camera-photo-symbolic")
                    .color("#00000077")
                    .angle(45.0)
                    .radius(30.0)
                    .event("screenshot")
                    .build(),
            )
            .item(
                MenuItem::builder()
                    .id("settings")
                    .label("Settings")
                    .icon_name("settings-symbolic")
                    .color("#00000077")
                    .angle(90.0)
                    .radius(30.0)
                    .event("settings")
                    .build(),
            )
            .item(
                MenuItem::builder()
                    .id("exit")
                    .label("Exit")
                    .icon_name("window-close-symbolic")
                    .color("#55222277")
                    .angle(135.0)
                    .radius(30.0)
                    .event("exit")
                    .build(),
            )
            .item(
                MenuItem::builder()
                    .id("rotate-ccw")
                    .label("Rotate CCW")
                    .icon_name("object-rotate-left-symbolic")
                    .color("#00000077")
                    .angle(180.0)
                    .radius(30.0)
                    .event("rotate-ccw")
                    .build(),
            )
            .item(
                MenuItem::builder()
                    .id("toggle-maximize")
                    .label("Maximize")
                    .icon_name("window-maximize-symbolic")
                    .color("#00000077")
                    .angle(225.0)
                    .radius(30.0)
                    .event("toggle-maximize")
                    .build(),
            )
            .item(
                MenuItem::builder()
                    .id("minimize")
                    .label("Minimize")
                    .icon_name("window-minimize-symbolic")
                    .color("#00000077")
                    .angle(270.0)
                    .radius(30.0)
                    .event("minimize")
                    .build(),
            )
            .item(
                MenuItem::builder()
                    .id("toggle-fullscreen")
                    .label("Fullscreen")
                    .icon_name("view-fullscreen-symbolic")
                    .color("#00000077")
                    .angle(315.0)
                    .radius(30.0)
                    .event("toggle-fullscreen")
                    .build(),
            )
            .build()
    }
}
