//mod activities;
mod entities;
//mod work_hours;
//mod groups;
//mod activity_insertion;
mod header;

use crate::app::App;

impl App {
    pub fn connect_gtk(&self) {
        self.connect_header_buttons();
        self.connect_entities_tab();
    }
}
