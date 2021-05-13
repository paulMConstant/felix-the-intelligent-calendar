use crate::app::App;
use gtk::prelude::*;

impl App {
    pub fn connect_clear_notification(&self) {
        fetch_from!(
            self.ui.borrow(),
            notification_revealer,
            clear_notification_button
        );
        app_register_signal!(
            self,
            clear_notification_button,
            clear_notification_button.connect_clicked(move |_| {
                notification_revealer.set_reveal_child(false);
            })
        );
    }
}
