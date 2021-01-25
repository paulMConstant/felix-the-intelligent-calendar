#[macro_use]
pub mod macros;

pub mod app_builder;
pub mod connect;
pub mod notify;
pub mod ui;

use gtk::prelude::*;
use std::sync::{Arc, Mutex, MutexGuard};

use crate::config::APP_NAME;
use felix_backend::data::Data;
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
            .add_from_resource("/com/github/paulmconstant/felix/ui/main_window.ui")
            .expect("Could not load ui file: main_window.ui");

        builder
            .add_from_resource("/com/github/paulmconstant/felix/ui/data_window.ui")
            .expect("Could not load ui file: data_window.ui");

        let data = Arc::new(Mutex::new(Data::new()));
        let ui = Arc::new(Mutex::new(Ui::new(builder)));

        fetch_from!(ui.lock().unwrap(), main_window);
        main_window.set_application(Some(application));
        main_window.set_title(APP_NAME);

        App { data, ui }
    }

    pub fn ui(&self) -> MutexGuard<Ui> {
        self.ui.lock().unwrap()
    }

    pub fn data(&self) -> MutexGuard<Data> {
        self.data.lock().unwrap()
    }
}
