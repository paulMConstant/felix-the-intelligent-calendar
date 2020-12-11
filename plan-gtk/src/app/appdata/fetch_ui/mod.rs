pub mod entities;

use crate::app::appdata::AppData;
use gtk::prelude::*;

/// Creates variables with the same name as object methods.
///
/// # Example
///
/// fetch\_from(self, x, y) expands to :
/// let x = self.x();
/// let y = self.y();
macro_rules! fetch_from {
    ($from: expr, $x: ident) => {
        let $x = $from.$x();
    };

    ($from: expr, $x: ident, $($rest:ident),*) => {
        fetch_from!($from, $x);
        fetch_from!($from, $($rest),*);
    };
}

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
