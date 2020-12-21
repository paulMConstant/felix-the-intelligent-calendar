use crate::app::App;

mod data;
pub mod ui;

impl App {
    pub fn connect_ui(&self) {
        self.connect_activities_tab();
        self.connect_entities_tab();
        self.connect_header_buttons();
        self.connect_groups_tab();
    }

    pub fn connect_data(&mut self) {
        self.connect_entity_data_events();
        self.connect_group_data_events();
    }
}
