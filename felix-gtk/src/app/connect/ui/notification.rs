use crate::app::App;
use gtk::prelude::*;

impl App {
    pub fn connect_notifications(&self) {
        self.connect_clear_mainwindow_notification();
    }

    fn connect_clear_mainwindow_notification(&self) {
        fetch_from!(
            self.ui.borrow(),
            main_notification_revealer,
            clear_main_notification_button
        );
        app_register_signal!(
            self,
            clear_main_notification_button,
            clear_main_notification_button.connect_clicked(move |_| {
                main_notification_revealer.set_reveal_child(false);
            })
        );
    }
}
