use crate::app::app_data::AppData;

use gtk::prelude::*;
use glib::signal::SignalHandlerId;

impl AppData {
    pub fn register_signal<T>(&mut self, widget: T, signal: SignalHandlerId)
    where
        T: IsA<gtk::Buildable>,
    {
        let widget_id = get_widget_id(&widget);
        self.signals.entry(widget_id).or_default().push(signal);
    }

    pub(super) fn get_registered_signals<T>(&self, widget: &T) -> Option<&Vec<SignalHandlerId>>
    where
        T: IsA<gtk::Buildable>,
    {
        let widget_id = get_widget_id(widget);
        self.signals.get(&widget_id)
    }
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
