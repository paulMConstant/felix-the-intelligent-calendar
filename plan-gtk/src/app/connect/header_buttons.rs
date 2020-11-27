use glib::clone;
use gtk::prelude::*;

use crate::app::App;

impl App {
    pub fn connect_header_buttons(&self) {
        // Connect data button
        let data_button: gtk::Button = self
            .builder
            .get_object("DataButton")
            .expect("Could not get DataButton from ui file");

        let data_window: gtk::Window = self
            .builder
            .get_object("DataWindow")
            .expect("Could not get DataWindow from ui file.");

        data_window.resize(800, 600);
        data_window.connect_delete_event(clone!(@strong data_window => move |_, _|
        {
            data_window.hide();
            glib::signal::Inhibit(true)
        }
        ));
        data_button.connect_clicked(move |_| data_window.show());
    }
}
