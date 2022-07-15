use gtk::traits::{GtkMenuItemExt, MenuShellExt};
use gtk::MenuItem;

use crate::gui::handler;
use crate::power::modes::POWER_MODES;
use crate::shell_utils::CctkCommand;

pub fn menu_items() -> [MenuItem; 4] {
    POWER_MODES.map(|mode| {
        let item = MenuItem::with_label(mode.name);
        item.connect_activate(|_| {
            handler(mode.command_as_root(), &format!("power mode {}", mode.name))
        });
        item
    })
}

pub fn power_submenu() -> MenuItem {
    let power_menu_item = MenuItem::with_label("Power");
    let power_menu = gtk::Menu::new();
    for item in menu_items().iter() {
        power_menu.append(item)
    }
    power_menu_item.set_submenu(Some(&power_menu));
    power_menu_item
}
