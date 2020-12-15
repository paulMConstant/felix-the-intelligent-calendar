use crate::app::App;

mod activities;
mod entities;
//mod work_hours;
mod groups;
//mod activity_insertion;
mod header;

impl App {
    pub fn connect_gtk(&self) {
        self.connect_activities_tab();
        self.connect_entities_tab();
        self.connect_header_buttons();
        self.connect_groups_tab();
    }
}
