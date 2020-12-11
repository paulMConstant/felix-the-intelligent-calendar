use glib::clone;
use gtk::prelude::*;

use crate::app::App;

impl App {
    pub fn connect_entities_tab(&self) {
        self.connect_add_entity();
        self.connect_entity_selected();
    }

    fn connect_add_entity(&self) {
        fetch_from!(self.app_data.borrow(), add_entity_button, add_entity_entry);

        let app_data = self.app_data.clone();
        add_entity_button.connect_clicked(clone!(@strong app_data => move |_| {
            app_data.borrow_mut().add_entity_event();
        }));

        add_entity_entry.connect_activate(clone!(@strong app_data => move |_| {
            app_data.borrow_mut().add_entity_event();
        }));
    }

    fn connect_entity_selected(&self) {
        fetch_from!(self.app_data.borrow(), entities_tree_view);

        let app_data = self.app_data.clone();
        entities_tree_view.connect_row_activated(
            clone!(@strong app_data => move |_self, path, _col| {
                app_data.borrow().entity_selected_event(path);
            }),
        );
    }
}
