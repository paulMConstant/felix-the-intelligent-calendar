#[macro_use]
pub mod macros;

pub mod app_builder;
pub mod connect;
pub mod save_state;
pub mod ui;

use crate::config;
use felix_data::Data;
use ui::Ui;

use gio::ApplicationExt;
use gtk::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct App {
    data: Rc<RefCell<Data>>,
    ui: Rc<RefCell<Ui>>,
}

impl App {
    /// Loads UI files in UI builder, binds mainwindow to application and sets title.
    pub fn new(application: &gtk::Application) -> App {
        let data = init_data();
        let ui = init_ui(application);

        let app = App { data, ui };

        serialize_data_on_shutdown(&app, application);
        serialize_ui_state_on_shutdown(&app, application);
        app
    }
}

fn init_data() -> Rc<RefCell<Data>> {
    let config_file_contents = std::fs::read_to_string(config::DATA_CONF_FILE);

    let data = if let Ok(contents) = config_file_contents {
        let data_value: serde_json::Result<Data> = serde_json::from_str(&contents);
        if let Ok(mut data) = data_value {
            data.init_computation_module();
            data
        } else {
            // TODO error message then start
            Data::new()
        }
    } else {
        Data::new()
    };

    Rc::new(RefCell::new(data))
}

fn init_ui(application: &gtk::Application) -> Rc<RefCell<Ui>> {
    // Initialize UI
    let builder = gtk::Builder::new();
    builder
        .add_from_resource("/com/github/paulmconstant/felix/ui/main_window.ui")
        .expect("Could not load ui file: main_window.ui");

    builder
        .add_from_resource("/com/github/paulmconstant/felix/ui/data_window.ui")
        .expect("Could not load ui file: data_window.ui");

    builder
        .add_from_resource("/com/github/paulmconstant/felix/ui/settings_window.ui")
        .expect("Could not load ui file: settings_window.ui");

    let ui = Rc::new(RefCell::new(Ui::new(builder)));

    fetch_from!(ui.borrow(), main_window, version_label);
    main_window.set_application(Some(application));
    main_window.set_title(config::APP_NAME);

    version_label.set_text(config::VERSION);

    ui
}

fn serialize_data_on_shutdown(app: &App, application: &gtk::Application) {
    let app = app.clone();
    application.connect_shutdown(move |_gtk_app| {
        app.save_data();
    });
}

fn serialize_ui_state_on_shutdown(app: &App, application: &gtk::Application) {
    let app = app.clone();
    application.connect_shutdown(move |_gtk_app| {
        app.save_ui_state();
    });
}
