use glib::clone;
use gtk::prelude::*;

use crate::app::App;

impl App {
    pub fn connect_entities_tab(&self) {
        self.connect_add_entity();
        self.connect_entity_selected();
        self.connect_remove_entity();
        self.connect_set_entity_mail();
        self.connect_set_send_mail_to();
    }

    fn connect_add_entity(&self) {
        fetch_from!(
            self.app_data.lock().unwrap(),
            add_entity_button,
            add_entity_entry
        );

        let app_data = self.app_data.clone();
        app_register_signal!(
            self,
            add_entity_button,
            add_entity_button.connect_clicked(clone!(@strong app_data => move |_| {
                app_data.lock().unwrap().event_add_entity()
            }))
        );

        app_register_signal!(
            self,
            add_entity_entry,
            add_entity_entry.connect_activate(clone!(@strong app_data => move |_| {
                app_data.lock().unwrap().event_add_entity();
            }))
        );
    }

    fn connect_entity_selected(&self) {
        fetch_from!(self.app_data.lock().unwrap(), entities_tree_view);

        let app_data = self.app_data.clone();
        app_register_signal!(
            self,
            entities_tree_view,
            entities_tree_view.connect_row_activated(
                clone!(@strong app_data => move |_self, path, _col| {
                    app_data.lock().unwrap().event_entity_selected(path);
                }),
            )
        );
    }

    fn connect_remove_entity(&self) {
        fetch_from!(self.app_data.lock().unwrap(), entity_remove_button);

        let app_data = self.app_data.clone();
        app_register_signal!(
            self,
            entity_remove_button,
            entity_remove_button.connect_clicked(clone!(@strong app_data => move |_| {
               app_data.lock().unwrap().event_remove_entity();
            }))
        );
    }

    fn connect_set_entity_mail(&self) {
        fetch_from!(self.app_data.lock().unwrap(), entity_mail_entry);

        let app_data = self.app_data.clone();
        app_register_signal!(
            self,
            entity_mail_entry,
            entity_mail_entry.connect_key_release_event(clone!(@strong app_data => move |_, _| {
                app_data.lock().unwrap().event_set_entity_mail();
                glib::signal::Inhibit(false)
            }))
        );
    }

    fn connect_set_send_mail_to(&self) {
        fetch_from!(self.app_data.lock().unwrap(), entity_send_mail_switch);

        let app_data = self.app_data.clone();
        app_register_signal!(
            self,
            entity_send_mail_switch,
            entity_send_mail_switch.connect_property_active_notify(
                clone!(@weak app_data => move |_| {
                    app_data.lock().unwrap().event_set_send_mail();
                })
            )
        );
    }
}
