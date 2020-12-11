#[macro_use]
pub mod fetch_ui;
mod events;

use glib::signal::SignalHandlerId;
use gtk::prelude::*;
use plan_backend::data::Data;
use std::collections::HashMap;

struct AppCurrentState {
    current_entity: Option<String>,
    current_group: Option<String>,
    current_activity_id: Option<u16>,
}

pub struct AppData {
    builder: gtk::Builder,
    data: Data,
    state: AppCurrentState,
    signals: HashMap<String, Vec<SignalHandlerId>>,
}

fn get_widget_id<T>(widget: &T) -> String
where
    T: IsA<gtk::Buildable>,
{
    widget
        .get_buildable_name()
        .expect("Widget has no ID !")
        .to_string()
}

impl AppData {
    pub fn new(builder: gtk::Builder) -> AppData {
        AppData {
            builder,
            data: Data::new(),
            state: AppCurrentState {
                current_entity: None,
                current_group: None,
                current_activity_id: None,
            },
            signals: HashMap::new(),
        }
    }

    pub fn show_mainwindow(&self) {
        fetch_from!(self, main_window);
        main_window.show_all();
    }

    pub fn register_signal<T>(&mut self, widget: T, signal: SignalHandlerId)
    where
        T: IsA<gtk::Buildable>,
    {
        let widget_id = get_widget_id(&widget);
        self.signals.entry(widget_id).or_default().push(signal);
    }

    pub fn get_registered_signals<T>(&self, widget: &T) -> Option<&Vec<SignalHandlerId>>
    where
        T: IsA<gtk::Buildable>,
    {
        let widget_id = get_widget_id(widget);
        self.signals.get(&widget_id)
    }
}
