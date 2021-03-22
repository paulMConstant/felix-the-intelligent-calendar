#[macro_use]
pub mod macros;

pub mod app_builder;
pub mod connect;
pub mod notify;
pub mod save_state;
pub mod ui;

use crate::config::{APP_NAME, DATA_CONF_FILE};
use felix_backend::data::Data;
use ui::Ui;

use gio::ApplicationExt;
use gtk::prelude::*;
use std::cell::RefCell;
use std::fs;
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
        let ui = init_ui(&application);

        let app = App { data, ui };

        serialize_data_on_shutdown(&app, &application);
        app
    }
}

fn init_data() -> Rc<RefCell<Data>> {
    let config_file_contents = fs::read_to_string(DATA_CONF_FILE);

    let data = if let Ok(contents) = config_file_contents {
        let data_value: serde_json::Result<Data> = serde_json::from_str(&contents);
        if let Ok(mut data) = data_value {
            data.queue_every_activity_for_beginning_computation();
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

    let ui = Rc::new(RefCell::new(Ui::new(builder)));

    fetch_from!(ui.borrow(), main_window);
    main_window.set_application(Some(application));
    main_window.set_title(APP_NAME);
    ui
}

fn serialize_data_on_shutdown(app: &App, application: &gtk::Application) {
    let app = app.clone();
    application.connect_shutdown(move |_app| {
        app.save_data();
    });
}
