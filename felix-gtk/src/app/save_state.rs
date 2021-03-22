use crate::app::App;
use crate::config::DATA_CONF_FILE;

use std::fs;

impl App {
    pub fn save_data(&self) {
        let json_data =
            serde_json::to_string(&*self.data.borrow()).expect("Could not serialize data");
        fs::write(DATA_CONF_FILE, json_data).expect("Could not write data to filesystem");
    }
}
