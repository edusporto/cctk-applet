mod power;

use gtk::traits::{GtkMenuItemExt, MenuShellExt};
use gtk::{Menu, MenuItem};

pub fn create_menu() -> Menu {
    let menu = gtk::Menu::new();

    menu.append(&power::power_submenu());
    menu.append(&gtk::SeparatorMenuItem::new());
    menu.append(&quit_item());

    menu
}

fn quit_item() -> MenuItem {
    let quit_item = gtk::MenuItem::with_label("Quit");
    quit_item.connect_activate(|_| {
        gtk::main_quit();
    });

    quit_item
}
