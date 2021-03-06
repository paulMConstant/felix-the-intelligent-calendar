use glib::clone;
use gtk::prelude::*;

use crate::app::ui::helpers::{format::cleaned_input, tree::get_selection_from_treeview};
use crate::app::App;

use felix_data::clean_string;

const ENTITY_NAME_COLUMN: i32 = 0;

impl App {
    pub fn connect_entities_tab(&self) {
        self.connect_add_entity();
        self.connect_entity_selected();
        self.connect_remove_entity();
        self.connect_rename_entity();

        self.connect_clean_entity_entries();
    }

    fn connect_add_entity(&self) {
        fetch_from!(self.ui.borrow(), entity_add_button, entity_add_entry);

        let data = &self.data;
        let ui = &self.ui;

        macro_rules! add_entity_closure {
            ($data:ident, $ui:ident, $entry:ident) => {
                let entity_name = $entry.get_text();
                with_blocked_signals!($ui.borrow_mut(), $entry.set_text(""), $entry);
                // If the name is empty, return without doing anything
                no_notify_assign_or_return!(entity_name, clean_string(entity_name));
                return_if_err!($ui, $data.borrow_mut().add_entity(&entity_name));
            };
        }

        app_register_signal!(
            self,
            entity_add_button,
            entity_add_button.connect_clicked(
                clone!(@strong data, @strong ui, @weak entity_add_entry => move |_| {
                    add_entity_closure!(data, ui, entity_add_entry);
                })
            )
        );

        app_register_signal!(
            self,
            entity_add_entry,
            entity_add_entry.connect_activate(clone!(@strong data, @strong ui => move |entry| {
                let entry = entry.clone();
                add_entity_closure!(data, ui, entry);
            }))
        );
    }

    fn connect_entity_selected(&self) {
        fetch_from!(self.ui.borrow(), entities_tree_view);

        let data = self.data.clone();
        let ui = self.ui.clone();
        app_register_signal!(
            self,
            entities_tree_view,
            entities_tree_view.connect_cursor_changed(move |tree_view| {
                let selected_entity = get_selection_from_treeview(&tree_view, ENTITY_NAME_COLUMN);
                if let Some(entity_name) = selected_entity {
                    assign_or_return!(ui, entity, data.borrow().entity(entity_name));
                    ui.borrow_mut().on_entity_selected(&data.borrow(), entity);
                }
            })
        );
    }

    fn connect_remove_entity(&self) {
        fetch_from!(self.ui.borrow(), entity_remove_button);

        let data = self.data.clone();
        let ui = self.ui.clone();
        app_register_signal!(
            self,
            entity_remove_button,
            entity_remove_button.connect_clicked(move |_| {
                let entity_to_remove = ui.borrow().current_entity().expect(
                    "Current entity should be selected before accessing any entity-related filed",
                );
                return_if_err!(ui, data.borrow_mut().remove_entity(entity_to_remove.name()));
            })
        );
    }

    fn connect_rename_entity(&self) {
        fetch_from!(self.ui.borrow(), entity_name_entry);

        let data = self.data.clone();
        let ui = self.ui.clone();
        app_register_signal!(
            self,
            entity_name_entry,
            entity_name_entry.connect_changed(move |entry| {
                let entity_to_rename = ui.borrow().current_entity().expect(
                        "Current entity should be selected before accessing any entity-related field",
                        ).name();
                let new_name = entry.get_text();
                no_notify_assign_or_return!(new_name, clean_string(new_name));
                if cleaned_input(&new_name) == entity_to_rename {
                    return;
                }
                no_notify_return_if_err!(
                    data.borrow_mut().set_entity_name(entity_to_rename, new_name)
                    );
            })
        );
    }

    fn connect_clean_entity_entries(&self) {
        connect_clean!(self, entity_add_entry, entity_name_entry);
    }
}
