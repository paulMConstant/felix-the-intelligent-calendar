use crate::app::App;

//mod activities;
pub mod entities;
//mod work_hours;
pub mod groups;
//mod activity_insertion;
pub mod header;

impl App {
    pub fn connect_gtk(&self) {
        self.connect_header_buttons();
        self.connect_entities_tab();
        self.connect_groups_tab();
    }
}
