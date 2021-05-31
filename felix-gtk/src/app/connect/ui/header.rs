use gtk::prelude::*;

use crate::app::App;

impl App {
    pub fn connect_header_buttons(&self) {
        self.connect_show_data_window();
        self.connect_show_settings_window();
    }
    
    fn connect_show_data_window(&self) {
        fetch_from!(self.ui.borrow(), data_window, data_button);

        data_window.connect_delete_event(move |window, _| {
            window.hide();
            glib::signal::Inhibit(true)
        });
        data_button.connect_clicked(move |_| data_window.show());
    }

    fn connect_show_settings_window(&self) {
        fetch_from!(self.ui.borrow(), settings_window, settings_button);

        settings_window.connect_delete_event(move |window, _| {
            window.hide();
            glib::signal::Inhibit(true)
        });
        settings_button.connect_clicked(move |_| settings_window.show());
    }
}
