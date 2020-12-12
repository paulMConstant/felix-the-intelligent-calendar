pub mod entities;

use crate::app::appdata::AppData;
use gtk::prelude::*;

impl AppData {
    #[must_use]
    pub fn main_window(&self) -> gtk::ApplicationWindow {
        self.builder
            .get_object("MainWindow")
            .expect("Could not get MainWindow from ui file.")
    }

    #[must_use]
    pub fn data_window(&self) -> gtk::Window {
        self.builder
            .get_object("DataWindow")
            .expect("Could not get DataWindow from ui file.")
    }

    #[must_use]
    pub fn data_button(&self) -> gtk::Button {
        self.builder
            .get_object("DataButton")
            .expect("Could not get DataButton from ui file")
    }
}
