use crate::app::appdata::AppData;

pub mod entities;
pub mod groups;
pub mod helpers;

impl AppData {
    pub(super) fn event_init(&mut self) {
        self.event_init_entities();
        self.event_init_groups();
    }
}
