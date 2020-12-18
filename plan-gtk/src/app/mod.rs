#[macro_use]
pub mod macros;

pub mod app_builder;
//pub mod app_data;
pub mod connect;
pub mod notify;
pub mod ui;

use super::config::APP_NAME;
use gtk::prelude::*;
use std::sync::{Arc, Mutex, MutexGuard};

use plan_backend::data::Data;
use ui::Ui;

pub struct App {
    data: Arc<Mutex<Data>>,
    ui: Arc<Mutex<Ui>>,
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

        let ui = Ui::new(builder);
        fetch_from!(ui, main_window);
        main_window.set_application(Some(application));
        main_window.set_title(APP_NAME);

        let data = Data::new();
        App {
            data: Arc::new(Mutex::new(data)),
            ui: Arc::new(Mutex::new(ui)),
        }
    }

    pub fn ui(&self) -> MutexGuard<Ui> {
        self.ui.lock().unwrap()
    }

    pub fn data(&self) -> MutexGuard<Data> {
        self.data.lock().unwrap()
    }
}
