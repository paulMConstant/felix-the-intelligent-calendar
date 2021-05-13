use crate::app::ui::Ui;
use gtk::prelude::*;

impl Ui {
    pub fn notify_str(&self, s: &str) {
        fetch_from!(self, notification_revealer, notification_label);
        notification_label.set_text(s);
        notification_revealer.set_reveal_child(true);
    }

    pub fn notify_err(&self, error: Box<dyn std::error::Error>) {
        self.notify_str(&error.to_string());
    }
}
