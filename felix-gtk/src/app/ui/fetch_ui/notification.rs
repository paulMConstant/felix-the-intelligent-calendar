use crate::app::ui::Ui;
use gtk::prelude::*;

impl Ui {
    #[must_use]
    pub fn main_notification_revealer(&self) -> gtk::Revealer {
        fetch_ui_from_builder!(self, "MainNotificationRevealer")
    }

    #[must_use]
    pub fn main_notification_label(&self) -> gtk::Label {
        fetch_ui_from_builder!(self, "MainNotificationLabel")
    }

    #[must_use]
    pub fn clear_main_notification_button(&self) -> gtk::Button {
        fetch_ui_from_builder!(self, "ClearMainNotificationButton")
    }

    #[must_use]
    pub fn data_notification_revealer(&self) -> gtk::Revealer {
        fetch_ui_from_builder!(self, "DataNotificationRevealer")
    }

    #[must_use]
    pub fn data_notification_label(&self) -> gtk::Label {
        fetch_ui_from_builder!(self, "DataNotificationLabel")
    }

    #[must_use]
    pub fn clear_data_notification_button(&self) -> gtk::Button {
        fetch_ui_from_builder!(self, "ClearDataNotificationButton")
    }
}
