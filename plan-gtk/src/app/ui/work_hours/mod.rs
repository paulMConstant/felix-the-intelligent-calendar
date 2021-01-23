mod create_work_intervals_box;

use crate::app::ui::Ui;

use plan_backend::data::{Data, TimeInterval};

use gtk::prelude::*;

impl Ui {
    pub fn on_add_work_hour(&self, current_work_hours: Vec<TimeInterval>) {
        self.remove_work_hours_if_any();
        self.add_new_work_hours(current_work_hours, true);
    }

    pub fn on_work_hours_changed(&self, data: &Data) {
        self.remove_work_hours_if_any();
        self.add_new_work_hours(data.work_hours(), false);
    }

    fn remove_work_hours_if_any(&self) {
        fetch_from!(self, work_hours_scrolled_window);

        for child in work_hours_scrolled_window.get_children() {
            work_hours_scrolled_window.remove(&child);
        }
    }

    fn add_new_work_hours(&self, current_work_hours: Vec<TimeInterval>, add_work_hour: bool) {
        fetch_from!(self, work_hours_scrolled_window);
        let work_intervals_box = self.create_work_intervals_box(current_work_hours, add_work_hour);
        work_hours_scrolled_window.add(&work_intervals_box);
        work_intervals_box.show();
    }
}
