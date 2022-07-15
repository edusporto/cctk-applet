use gtk::prelude::*;

use cctk_applet::gui::indicator;
use cctk_applet::gui::menu;

fn main() {
    gtk::init().unwrap();

    let mut indicator = indicator::create_indicator();
    let mut menu = menu::create_menu();

    indicator.set_menu(&mut menu);
    menu.show_all();

    gtk::main();
}
