use crate::app::ui::Ui;

use glib::signal::SignalHandlerId;
use gtk::prelude::*;

impl Ui {
    pub(in super::super) fn register_signal<T>(&mut self, widget: T, signal: SignalHandlerId)
    where
        T: IsA<gtk::Buildable>,
    {
        let widget_id = get_widget_id(&widget);
        self.signals.entry(widget_id).or_default().push(signal);
    }

    pub(in super::super) fn get_registered_signals<T>(
        &self,
        widget: &T,
    ) -> Option<&Vec<SignalHandlerId>>
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
    widget.get_buildable_name().expect("Widget has no ID !")
}
