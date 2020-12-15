use crate::app::appdata::AppData;

mod entities;
mod general;
mod groups;
mod helpers;

impl AppData {
    pub(super) fn event_init(&mut self) {
        self.event_init_entities();
        self.event_init_groups();
    }
}
