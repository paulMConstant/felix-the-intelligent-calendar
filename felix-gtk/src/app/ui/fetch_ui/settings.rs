use crate::app::ui::Ui;
use gtk::prelude::*;

impl Ui {
    #[must_use]
    pub fn settings_window(&self) -> gtk::Window {
        fetch_ui_from_builder!(self, "SettingsWindow")
    }

    #[must_use]
    pub fn settings_button(&self) -> gtk::Button {
        fetch_ui_from_builder!(self, "SettingsButton")
    }

    #[must_use]
    pub fn version_label(&self) -> gtk::Label {
        fetch_ui_from_builder!(self, "VersionLabel")
    }
}
