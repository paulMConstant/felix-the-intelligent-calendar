mod activities_tree;
mod header_buttons;

use crate::app::App;

impl App {
    pub fn connect_gtk(&self) {
        self.connect_header_buttons();
        self.connect_activities_tree_buttons();
    }
}
