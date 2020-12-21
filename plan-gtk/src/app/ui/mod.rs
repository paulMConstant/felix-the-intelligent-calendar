pub mod events;
pub mod fetch_ui;
pub mod helpers;
pub mod signals;

use glib::signal::SignalHandlerId;
use gtk::prelude::*;
use std::collections::HashMap;

use plan_backend::data::{Activity, Entity, Group};

pub struct Ui {
    builder: gtk::Builder,
    signals: HashMap<String, Vec<SignalHandlerId>>,
    current_entity: Option<Entity>,
    current_group: Option<Group>,
    current_activity: Option<Activity>,
}

impl Ui {
    pub fn new(builder: gtk::Builder) -> Ui {
        Ui {
            builder,
            signals: HashMap::new(),
            current_entity: None,
            current_group: None,
            current_activity: None,
        }
    }

    pub fn current_entity(&self) -> Option<Entity> {
        self.current_entity.clone()
    }

    pub fn current_group(&self) -> Option<Group> {
        self.current_group.clone()
    }

    pub fn current_activity(&self) -> Option<Activity> {
        self.current_activity.clone()
    }

    pub fn show_mainwindow(&mut self) {
        fetch_from!(self, main_window);
        main_window.show_all();
        self.init_ui_state();
    }
}
