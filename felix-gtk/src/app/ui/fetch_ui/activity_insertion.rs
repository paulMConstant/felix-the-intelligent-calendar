use gtk::prelude::*;

use crate::app::ui::Ui;

impl Ui {
    #[must_use]
    pub fn insertion_box(&self) -> gtk::Box {
        fetch_ui_from_builder!(self, "InsertionBox")
    }

    #[must_use]
    pub fn show_schedule_entry(&self) -> gtk::Entry {
        fetch_ui_from_builder!(self, "ShowScheduleEntry")
    }

    #[must_use]
    pub fn show_schedule_button(&self) -> gtk::Button {
        fetch_ui_from_builder!(self, "ShowScheduleButton")
    }

    #[must_use]
    pub fn schedules_top_box(&self) -> gtk::Box {
        fetch_ui_from_builder!(self, "SchedulesTopBox")
    }

    #[must_use]
    pub fn clear_activities_button(&self) -> gtk::Button {
        fetch_ui_from_builder!(self, "ClearActivitiesButton")
    }
}
