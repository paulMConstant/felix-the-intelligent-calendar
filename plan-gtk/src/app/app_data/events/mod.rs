use crate::app::app_data::AppData;

mod activities;
mod entities;
mod general;
mod groups;
mod helpers;

impl AppData {
    pub(in super::super) fn event_init(&mut self) {
        self.event_init_activities();
        self.event_init_entities();
        self.event_init_groups();
    }
}
