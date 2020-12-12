use crate::app::appdata::AppData;

pub mod entities;

impl AppData {
    pub(super) fn event_init(&mut self) {
        self.init_entity_events();
    }
}
