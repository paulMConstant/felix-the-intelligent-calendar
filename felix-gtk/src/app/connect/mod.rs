use crate::app::App;

mod data;
mod ui;

impl App {
    pub fn connect_ui(&self) {
        self.connect_activities_tab();
        self.connect_entities_tab();
        self.connect_custom_work_hours();
        self.connect_header_buttons();
        self.connect_groups_tab();
        self.connect_work_hours_tab();
        self.connect_activity_insertion();
        self.connect_clear_notification();
        self.connect_export();
    }

    pub fn connect_data(&mut self) {
        self.connect_entity_events();
        self.connect_group_events();
        self.connect_activity_events();
        self.connect_work_hour_events();
    }
}
