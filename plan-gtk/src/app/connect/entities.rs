use glib::clone;
use gtk::prelude::*;

use crate::app::App;

impl App {
    pub fn connect_entities_tab(&self) {
        self.connect_add_entity();
        self.connect_entity_selected();
        self.connect_remove_entity();
        self.connect_rename_entity();
        self.connect_set_entity_mail();
        self.connect_set_send_mail_to();

        self.connect_clean_add_entity_entry();
        self.connect_clean_entity_name_entry();
    }

    fn connect_add_entity(&self) {
        fetch_from!(
            self.app_data.lock().unwrap(),
            entity_add_button,
            entity_add_entry
        );

        let app_data = self.app_data.clone();
        app_register_signal!(
            self,
            entity_add_button,
            entity_add_button.connect_clicked(clone!(@strong app_data => move |_| {
                app_data.lock().unwrap().event_add_entity()
            }))
        );

        app_register_signal!(
            self,
            entity_add_entry,
            entity_add_entry.connect_activate(clone!(@strong app_data => move |_| {
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
            entities_tree_view.connect_cursor_changed(clone!(@strong app_data => move |_| {
                app_data.lock().unwrap().event_entity_selected();
            }),)
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

    fn connect_rename_entity(&self) {
        fetch_from!(self.app_data.lock().unwrap(), entity_name_entry);

        let app_data = self.app_data.clone();
        app_register_signal!(
            self,
            entity_name_entry,
            entity_name_entry.connect_changed(clone!(@strong app_data => move |_| {
                app_data.lock().unwrap().event_rename_entity();
            }))
        );
    }

    fn connect_set_entity_mail(&self) {
        fetch_from!(self.app_data.lock().unwrap(), entity_mail_entry);

        let app_data = self.app_data.clone();
        app_register_signal!(
            self,
            entity_mail_entry,
            entity_mail_entry.connect_changed(clone!(@strong app_data => move |_| {
                app_data.lock().unwrap().event_set_entity_mail();
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

    fn connect_clean_add_entity_entry(&self) {
        fetch_from!(self.app_data.lock().unwrap(), entity_add_entry);

        let app_data = self.app_data.clone();
        app_register_signal!(
            self,
            entity_add_entry,
            entity_add_entry.connect_changed(
                clone!(@strong app_data, @weak entity_add_entry => move |_| {
                    app_data.lock().unwrap().event_clean_entry_content(entity_add_entry);
                })
            )
        );
    }

    fn connect_clean_entity_name_entry(&self) {
        fetch_from!(self.app_data.lock().unwrap(), entity_name_entry);

        let app_data = self.app_data.clone();
        app_register_signal!(
            self,
            entity_name_entry,
            entity_name_entry.connect_changed(
                clone!(@strong app_data, @weak entity_name_entry => move |_| {
                    app_data.lock().unwrap().event_clean_entry_content(entity_name_entry);
                })
            )
        );
    }
}
