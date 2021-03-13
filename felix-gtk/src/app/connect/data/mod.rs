use crate::app::App;

mod activities;
mod entities;
mod groups;
mod work_hours;

impl App {
    pub(super) fn connect_data_events(&mut self) {
        self.connect_entity_events();
        self.connect_group_events();
        self.connect_activity_events();
        self.connect_work_hour_events();
    }
}
