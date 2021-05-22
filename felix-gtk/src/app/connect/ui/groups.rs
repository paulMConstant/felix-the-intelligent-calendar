use crate::app::App;

use crate::app::ui::{
    groups_treeview_config::*,
    helpers::{format::cleaned_input, tree::get_selection_from_treeview},
};

use felix_backend::clean_string;

use glib::clone;
use gtk::prelude::*;

impl App {
    pub fn connect_groups_tab(&self) {
        self.connect_add_group();
        self.connect_group_selected();
        self.connect_add_entity_to_group();
        self.connect_remove_group();
        self.connect_rename_group();
        self.connect_remove_entity_from_group();

        self.connect_clean_group_entries();
    }

    fn connect_add_group(&self) {
        fetch_from!(self.ui.borrow(), group_add_button, group_add_entry);

        let data = self.data.clone();
        let ui = self.ui.clone();

        app_register_signal!(
            self,
            group_add_button,
            group_add_button.connect_clicked(
                clone!(@strong ui, @strong data, @weak group_add_entry => move |_| {
                let group_to_add = group_add_entry.get_text();
                with_blocked_signals!(ui.borrow(), group_add_entry.set_text(""), group_add_entry);

                no_notify_assign_or_return!(group_to_add, clean_string(group_to_add));
                return_if_err!(ui, data.borrow_mut().add_group(group_to_add));
                    })
            )
        );

        let data = self.data.clone();
        let ui = self.ui.clone();

        app_register_signal!(
            self,
            group_add_entry,
            group_add_entry.connect_activate(move |entry| {
                let group_to_add = entry.get_text();
                let entry = entry.clone();
                with_blocked_signals!(ui.borrow(), entry.set_text(""), entry);

                no_notify_assign_or_return!(group_to_add, clean_string(group_to_add));
                return_if_err!(ui, data.borrow_mut().add_group(group_to_add));
            })
        );
    }

    fn connect_group_selected(&self) {
        fetch_from!(self.ui.borrow(), groups_tree_view);

        let data = self.data.clone();
        let ui = self.ui.clone();
        app_register_signal!(
            self,
            groups_tree_view,
            groups_tree_view.connect_cursor_changed(move |tree_view| {
                let selected_group = get_selection_from_treeview(&tree_view, GROUP_NAME_COLUMN);
                if let Some(group_name) = selected_group {
                    assign_or_return!(ui, group, data.borrow().group(group_name));
                    ui.borrow_mut().on_group_selected(group);
                }
            })
        );
    }

    fn connect_remove_group(&self) {
        fetch_from!(self.ui.borrow(), group_remove_button);

        let data = self.data.clone();
        let ui = self.ui.clone();
        app_register_signal!(
            self,
            group_remove_button,
            group_remove_button.connect_clicked(move |_| {
                let group_to_remove = ui
                    .borrow()
                    .current_group()
                    .as_ref()
                    .expect(
                        "Current group should be selected before accessing any group-related filed",
                    )
                    .name();
                return_if_err!(ui, data.borrow_mut().remove_group(group_to_remove));
            })
        );
    }

    fn connect_rename_group(&self) {
        fetch_from!(self.ui.borrow(), group_name_entry);

        let data = self.data.clone();
        let ui = self.ui.clone();
        app_register_signal!(
            self,
            group_name_entry,
            group_name_entry.connect_changed(move |entry| {
                let group_to_rename = ui
                    .borrow()
                    .current_group()
                    .as_ref()
                    .expect(
                        "Current group should be selected before accessing any group-related field",
                    )
                    .name();
                let new_name = entry.get_text();
                no_notify_assign_or_return!(new_name, clean_string(new_name));
                if cleaned_input(&new_name) == group_to_rename {
                    return;
                }
                return_if_err!(
                    ui,
                    data.borrow_mut().set_group_name(group_to_rename, new_name)
                );
            })
        );
    }

    fn connect_add_entity_to_group(&self) {
        macro_rules! add_entity_to_group_closure {
            ($data:ident, $ui: ident, $entity_into_group_name_entry:ident,) => {
                clone!(@strong $ui,
                       @strong $data,
                       @weak $entity_into_group_name_entry => move |_| {
                let mut data = $data.borrow_mut();
                let group_in_which_to_add = $ui.borrow().current_group().as_ref()
                    .expect("Current group should be selected before accessing any group-related filed")
                    .name();
                let entity_name = $entity_into_group_name_entry.get_text();
                with_blocked_signals!(
                    $ui.borrow(),
                    $entity_into_group_name_entry.set_text(""),
                    $entity_into_group_name_entry
                );

                return_if_err!(
                    $ui,
                    data
                    .add_entity_to_group(group_in_which_to_add, entity_name));
                    })
            };
        }

        let data = self.data.clone();
        let ui = self.ui.clone();

        fetch_from!(
            self.ui.borrow(),
            entity_into_group_name_entry,
            add_to_group_button
        );

        let entity_into_group_name_entry_cloned = entity_into_group_name_entry.clone();
        app_register_signal!(
            self,
            entity_into_group_name_entry_cloned,
            entity_into_group_name_entry.connect_activate(add_entity_to_group_closure!(
                data,
                ui,
                entity_into_group_name_entry,
            ))
        );

        app_register_signal!(
            self,
            add_to_group_button,
            add_to_group_button.connect_clicked(add_entity_to_group_closure!(
                data,
                ui,
                entity_into_group_name_entry,
            ))
        );
    }

    fn connect_remove_entity_from_group(&self) {
        fetch_from!(
            self.ui.borrow(),
            group_members_tree_view,
            group_members_list_store
        );

        let data = self.data.clone();
        let ui = self.ui.clone();
        app_register_signal!(
            self,
            group_members_tree_view,
            group_members_tree_view.connect_row_activated(move |tree_view, treepath, treeview_column| {
        let delete_column = tree_view
            .get_column(GROUP_DELETE_COLUMN)
            .expect("Group Members tree view should have at least 2 columns");
        if &delete_column == treeview_column {
            let iter = group_members_list_store
                .get_iter(treepath)
                .expect("Row was activated, path should be valid");
            let entity_to_remove = group_members_list_store.get_value(&iter, 0);
            let entity_to_remove = entity_to_remove
                .get::<&str>()
                .expect("Value should be gchararray")
                .expect("Value should be gchararray");

            let current_group_name = ui.borrow().current_group().as_ref().expect("Current group should be set before performing any action on a group").name();
            return_if_err!(ui, data.borrow_mut()
                .remove_entity_from_group(current_group_name, entity_to_remove));
        }
            }));
    }

    fn connect_clean_group_entries(&self) {
        connect_clean!(
            self,
            entity_into_group_name_entry,
            group_add_entry,
            group_name_entry
        );
    }
}
