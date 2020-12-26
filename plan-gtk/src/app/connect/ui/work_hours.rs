use crate::app::App;

use glib::clone;
use gtk::prelude::*;

impl App {
    pub fn connect_work_hours_tab(&self) {
        self.connect_add_work_hour();
    }

    fn connect_add_work_hour(&self) {
        fetch_from!(self.ui(), work_hour_add_button);

        let ui = self.ui.clone();
        let data = self.data.clone();
        app_register_signal!(
            self,
            work_hour_add_button,
            work_hour_add_button.connect_clicked(clone!(@strong data, @strong ui => move |_| {
                let current_work_hours = data.lock().unwrap().work_hours();
                ui.lock().unwrap().on_add_work_hour(current_work_hours);
            }))
        );
    }
}
