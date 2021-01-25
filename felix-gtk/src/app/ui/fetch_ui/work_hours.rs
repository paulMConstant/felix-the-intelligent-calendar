use gtk::prelude::*;

use crate::app::ui::Ui;

impl Ui {
    #[must_use]
    pub fn work_hour_add_button(&self) -> gtk::Button {
        fetch_ui_from_builder!(self, "AddWorkHourButton")
    }

    #[must_use]
    pub fn work_hours_scrolled_window(&self) -> gtk::ScrolledWindow {
        fetch_ui_from_builder!(self, "WorkHoursScrolledWindow")
    }
}
