use crate::app::App;

mod data;
mod ui;

impl App {
    pub fn connect_ui(&self) {
        self.connect_activities_tab();
        self.connect_entities_tab();
        self.connect_header_buttons();
        self.connect_groups_tab();
        self.connect_work_hours_tab();
        self.connect_activity_insertion();
    }

    pub fn connect_data(&mut self) {
        self.connect_data_events();
    }
}
