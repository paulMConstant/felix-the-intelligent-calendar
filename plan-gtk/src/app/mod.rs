pub mod app_builder;
pub mod connect;

use gtk::prelude::*;

pub struct App {
    pub builder: gtk::Builder,
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

        App { builder }
    }

    pub fn show_mainwindow(&self) {
        let main_window: gtk::ApplicationWindow = self
            .builder
            .get_object("MainWindow")
            .expect("Could not get MainWindow from ui file.");
        main_window.show_all();
    }
}
