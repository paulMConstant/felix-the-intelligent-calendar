//extern crate byteorder;
//extern crate gdk;
//extern crate gio;
//extern crate gtk;

pub mod app;
pub mod config;

use app::app_builder::build_app;
use gio::prelude::*;

fn main() {
    let application = gtk::Application::new(Some(config::APP_ID), Default::default())
        .expect("Initialization failed.");

    application.connect_activate(move |app| {
        build_app(app);
    });

    application.run(&std::env::args().collect::<Vec<_>>());
}
