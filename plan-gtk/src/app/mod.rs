pub mod app_builder;
#[macro_use]
pub mod notify;
#[macro_use]
pub mod appdata;
pub mod connect;

use gtk::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

use appdata::AppData;

pub struct App {
    app_data: Rc<RefCell<AppData>>,
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

        let main_window: gtk::ApplicationWindow = builder
            .get_object("MainWindow")
            .expect("Could not get MainWindow from ui file.");

        main_window.set_application(Some(application));
        main_window.set_title("Plan");

        App {
            app_data: Rc::new(RefCell::new(AppData::new(builder))),
        }
    }

    pub fn show_mainwindow(&self) {
        self.app_data.borrow().show_mainwindow();
    }
}
