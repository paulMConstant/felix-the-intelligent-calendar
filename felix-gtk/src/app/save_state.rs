use crate::app::App;
use crate::config::{DATA_CONF_FILE, UI_STATE_FILE};

use std::fs;

impl App {
    pub fn save_data(&self) {
        let json_data =
            serde_json::to_string(&*self.data.borrow()).expect("Could not serialize data");
        fs::write(DATA_CONF_FILE, json_data).expect("Could not write data to filesystem");
    }

    pub fn save_ui_state(&self) {
        let json_data = serde_json::to_string(&self.ui.borrow().create_state_for_serialization())
            .expect("Could not serialize ui state");
        fs::write(UI_STATE_FILE, json_data).expect("Could not write data to filesystem");
    }
}
