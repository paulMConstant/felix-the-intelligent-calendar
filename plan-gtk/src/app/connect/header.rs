use glib::clone;
use gtk::prelude::*;

use crate::app::App;

impl App {
    pub fn connect_header_buttons(&self) {
        fetch_from!(self.app_data.borrow(), data_window, data_button);

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
