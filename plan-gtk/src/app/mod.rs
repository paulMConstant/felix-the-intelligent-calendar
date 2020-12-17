#[macro_use]
pub mod macros;

pub mod app_builder;
pub mod app_data;
pub mod app_ui;
pub mod connect;
pub mod notify;

use super::config::APP_NAME;
use gtk::prelude::*;
use std::sync::{Arc, Mutex};

use app_data::AppData;

pub struct App {
    app_data: Arc<Mutex<AppData>>,
}

impl App {
    /// Loads UI files in UI builder, binds mainwindow to application and sets title.
    pub fn new(application: &gtk::Application) -> App {
        let builder = gtk::Builder::new();
        builder
            .add_from_resource("/com/github/paulmconstant/plan/ui/main_window.ui")
            .expect("Could not load ui file: main_window.ui");

        builder
            .add_from_resource("/com/github/paulmconstant/plan/ui/data_window.ui")
            .expect("Could not load ui file: data_window.ui");

        builder
            .add_from_resource("/com/github/paulmconstant/plan/ui/time_interval.ui")
            .expect("Could not load ui file: time_interval.ui");

        let app_data = AppData::new(builder);

        fetch_from!(app_data, main_window);
        main_window.set_application(Some(application));
        main_window.set_title(APP_NAME);

        App {
            app_data: Arc::new(Mutex::new(app_data)),
        }
    }

    pub fn show_mainwindow(&self) {
        self.app_data.lock().unwrap().show_mainwindow();
        self.app_data.lock().unwrap().event_init();
    }
}
