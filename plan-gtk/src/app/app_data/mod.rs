pub mod events;
pub mod fetch_ui;
pub mod signals;

use plan_backend::data::{ActivityID, Data, Events};

use glib::signal::SignalHandlerId;
use gtk::prelude::*;
use std::collections::HashMap;

struct AppCurrentState {
    current_entity: Option<String>,
    current_group: Option<String>,
    current_activity_id: Option<ActivityID>,
}

pub struct AppData {
    builder: gtk::Builder,
    data: Data,
    state: AppCurrentState,
    signals: HashMap<String, Vec<SignalHandlerId>>,
}

impl AppData {
    pub fn new(builder: gtk::Builder) -> AppData {
        AppData {
            builder,
            data: Data::new(),
            state: AppCurrentState {
                current_entity: None,
                current_group: None,
                current_activity_id: None,
            },
            signals: HashMap::new(),
        }
    }

    pub fn show_mainwindow(&self) {
        fetch_from!(self, main_window);
        main_window.show_all();
    }

    pub fn events(&mut self) -> std::cell::RefMut<Events> {
        self.data.events()
    }
}
