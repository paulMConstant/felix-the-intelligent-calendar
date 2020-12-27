pub mod events;
pub mod fetch_ui;
pub mod helpers;
pub mod signals;

use glib::signal::SignalHandlerId;
use gtk::prelude::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};

use plan_backend::data::{Activity, Entity, Group};

pub struct Ui {
    builder: gtk::Builder,
    signals: HashMap<String, Vec<SignalHandlerId>>,
    current_entity: Option<Entity>,
    current_group: Option<Group>,
    current_activity: Option<Activity>,
    work_interval_builders: Arc<Mutex<Vec<gtk::Builder>>>,
    work_interval_editing_done_callback: Arc<dyn Fn(usize, gtk::Builder)>,
    work_interval_remove_callback: Arc<dyn Fn(usize)>,
}

impl Ui {
    pub fn new(builder: gtk::Builder) -> Ui {
        Ui {
            builder,
            signals: HashMap::new(),
            current_entity: None,
            current_group: None,
            current_activity: None,
            work_interval_builders: Arc::new(Mutex::new(Vec::new())),
            work_interval_editing_done_callback: Arc::new(Box::new(|_, _| {
                panic!("Work interval editing done callback was called before being set")
            })),
            work_interval_remove_callback: Arc::new(Box::new(|_| {
                panic!("Work interval remove callback was called before being set")
            })),
        }
    }

    #[must_use]
    pub fn current_entity(&self) -> Option<Entity> {
        self.current_entity.clone()
    }

    #[must_use]
    pub fn current_group(&self) -> Option<Group> {
        self.current_group.clone()
    }

    #[must_use]
    pub fn current_activity(&self) -> Option<Activity> {
        self.current_activity.clone()
    }

    #[must_use]
    pub fn work_interval_builders(&self) -> MutexGuard<Vec<gtk::Builder>> {
        self.work_interval_builders.lock().unwrap()
    }

    pub fn set_work_interval_editing_done_callback(
        &mut self,
        work_interval_editing_done_callback: Arc<dyn Fn(usize, gtk::Builder)>,
    ) {
        self.work_interval_editing_done_callback = work_interval_editing_done_callback;
    }

    pub fn set_work_interval_remove_callback(
        &mut self,
        work_interval_remove_callback: Arc<dyn Fn(usize)>,
    ) {
        self.work_interval_remove_callback = work_interval_remove_callback;
    }
    pub fn show_mainwindow(&mut self) {
        fetch_from!(self, main_window);
        main_window.show_all();
        self.init_ui_state();
    }
}