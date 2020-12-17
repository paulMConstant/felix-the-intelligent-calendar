use crate::app::App;

mod gtk;
mod data;

impl App {
    pub fn connect_gtk(&self) {
        self.connect_activities_tab();
        self.connect_entities_tab();
        self.connect_header_buttons();
        self.connect_groups_tab();
    }

    pub fn connect_data(&mut self) {
        self.connect_entity_data_events();
    }
}
