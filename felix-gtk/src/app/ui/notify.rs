use crate::app::ui::Ui;
use gtk::prelude::*;

impl Ui {
    pub fn notify_err(&self, error: Box<dyn std::error::Error>) {
        self.notify_str(&error.to_string());
    }

    pub fn notify_str(&self, s: &str) {
        fetch_from!(self, data_window);
        if data_window.get_property_has_toplevel_focus() {
            // Display in data window
            fetch_from!(self, data_notification_label, data_notification_revealer);
            display_str_notification(s, &data_notification_label, &data_notification_revealer);
        } else {
            // Display in main window
            fetch_from!(self, main_notification_revealer, main_notification_label);
            display_str_notification(s, &main_notification_label, &main_notification_revealer);
        }
    }
}

fn display_str_notification(
    s: &str,
    notification_label: &gtk::Label,
    notification_revealer: &gtk::Revealer,
) {
    notification_label.set_text(s);
    notification_revealer.set_reveal_child(true);
}
