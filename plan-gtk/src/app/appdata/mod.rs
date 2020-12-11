#[macro_use]
pub mod fetch_ui;
mod events;

use gtk::prelude::*;
use plan_backend::data::Data;

pub struct AppData {
    builder: gtk::Builder,
    data: Data,
}

impl AppData {
    pub fn new(builder: gtk::Builder) -> AppData {
        AppData {
            builder,
            data: Data::new(),
        }
    }

    pub fn show_mainwindow(&self) {
        fetch_from!(self, main_window);
        main_window.show_all();
    }
}
