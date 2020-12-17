mod activities;
mod entities;
mod groups;

use crate::app::app_data::AppData;
use gtk::prelude::*;

impl AppData {
    #[must_use]
    pub fn main_window(&self) -> gtk::ApplicationWindow {
        fetch_ui_from_builder!(self, "MainWindow")
    }

    #[must_use]
    pub fn data_window(&self) -> gtk::Window {
        fetch_ui_from_builder!(self, "DataWindow")
    }

    #[must_use]
    pub fn data_button(&self) -> gtk::Button {
        fetch_ui_from_builder!(self, "DataButton")
    }
}
