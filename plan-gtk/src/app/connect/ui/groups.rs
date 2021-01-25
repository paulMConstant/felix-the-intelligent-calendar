use crate::app::App;

use crate::app::ui::helpers::{format::cleaned_input, tree::get_selection_from_treeview};

use plan_backend::data::clean_string;

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
        fetch_from!(self.ui(), group_add_button, group_add_entry);

        let data = self.data.clone();
        let ui = self.ui.clone();
        app_register_signal!(
            self,
            group_add_button,
            group_add_button.connect_clicked(clone!(@strong ui, @strong data, @weak group_add_entry => move |_| {
        let group_to_add = group_add_entry.get_text();
        with_blocked_signals!(ui.lock().unwrap(), group_add_entry.set_text(""), group_add_entry);

        no_notify_assign_or_return!(group_to_add, clean_string(group_to_add));
        return_if_err!(data.lock().unwrap().add_group(group_to_add));
            }))
        );

        app_register_signal!(
            self,
            group_add_entry,
            group_add_entry.connect_activate(clone!(@strong ui, @strong data, @weak group_add_entry => move |_| {
        let group_to_add = group_add_entry.get_text();
        with_blocked_signals!(ui.lock().unwrap(), group_add_entry.set_text(""), group_add_entry);

        no_notify_assign_or_return!(group_to_add, clean_string(group_to_add));
        return_if_err!(data.lock().unwrap().add_group(group_to_add));
            }))
        );
    }

    fn connect_group_selected(&self) {
        fetch_from!(self.ui(), groups_tree_view);

        let data = self.data.clone();
        let ui = self.ui.clone();
        app_register_signal!(
            self,
            groups_tree_view,
            groups_tree_view.connect_cursor_changed(
                clone!(@strong ui, @strong data, @weak groups_tree_view => move |_| {
                let selected_group = get_selection_from_treeview(groups_tree_view);
                if let Some(group_name) = selected_group {
                    assign_or_return!(group, data.lock().unwrap().group(group_name));
                    ui.lock().unwrap().on_group_selected(group);
                }})
            )
        );
    }

    fn connect_remove_group(&self) {
        fetch_from!(self.ui(), group_remove_button);

        let data = self.data.clone();
        let ui = self.ui.clone();
        app_register_signal!(
            self,
            group_remove_button,
            group_remove_button.connect_clicked(clone!(@strong data => move |_| {
            let group_to_remove =
                ui.lock().unwrap().current_group().as_ref().expect(
                    "Current group should be selected before accessing any group-related filed",
                ).name();
               return_if_err!(data.lock().unwrap().remove_group(group_to_remove));
            }))
        );
    }

    fn connect_rename_group(&self) {
        fetch_from!(self.ui(), group_name_entry);

        let data = self.data.clone();
        let ui = self.ui.clone();
        app_register_signal!(
            self,
            group_name_entry,
            group_name_entry.connect_changed(
                clone!(@strong ui, @strong data, @weak group_name_entry => move |_| {
                let group_to_rename = ui.lock().unwrap().current_group().as_ref().expect(
                        "Current group should be selected before accessing any group-related field",
                    ).name();
                let new_name = group_name_entry.get_text();
                no_notify_assign_or_return!(new_name, clean_string(new_name));
                if cleaned_input(&new_name) == group_to_rename {
                    return;
                }
                return_if_err!(
                    data.lock().unwrap().set_group_name(group_to_rename, new_name)
                );
                    })
            )
        );
    }

    fn connect_add_entity_to_group(&self) {
        macro_rules! add_entity_to_group_closure {
            ($data:ident, $ui: ident, $entity_into_group_name_entry:ident,
             $create_entity_before_adding_to_group_switch:ident) => {
                clone!(@strong $ui,
                       @strong $data,
                       @weak $entity_into_group_name_entry,
                       @weak $create_entity_before_adding_to_group_switch => move |_| {
                let mut data = $data.lock().unwrap();
                let group_in_which_to_add = $ui.lock().unwrap().current_group().as_ref()
                    .expect("Current group should be selected before accessing any group-related filed")
                    .name();
                let entity_name = $entity_into_group_name_entry.get_text();
                with_blocked_signals!(
                    $ui.lock().unwrap(),
                    $entity_into_group_name_entry.set_text(""),
                    $entity_into_group_name_entry
                );

                no_notify_assign_or_return!(entity_name, clean_string(entity_name));
                if $create_entity_before_adding_to_group_switch.get_active()
                    && data.entity(&entity_name).is_err() {
                    return_if_err!(data.add_entity(&entity_name));
                }

                return_if_err!(
                    data
                    .add_entity_to_group(group_in_which_to_add, entity_name));
                    })
            };
        }

        let data = self.data.clone();
        let ui = self.ui.clone();

        fetch_from!(
            self.ui(),
            entity_into_group_name_entry,
            create_entity_before_adding_to_group_switch
        );

        app_register_signal!(
            self,
            entity_into_group_name_entry,
            entity_into_group_name_entry.connect_activate(add_entity_to_group_closure!(
                data,
                ui,
                entity_into_group_name_entry,
                create_entity_before_adding_to_group_switch
            ))
        );

        fetch_from!(
            self.ui(),
            entity_into_group_name_entry,
            add_to_group_button,
            create_entity_before_adding_to_group_switch
        );

        app_register_signal!(
            self,
            add_to_group_button,
            add_to_group_button.connect_clicked(add_entity_to_group_closure!(
                data,
                ui,
                entity_into_group_name_entry,
                create_entity_before_adding_to_group_switch
            ))
        );
    }

    fn connect_remove_entity_from_group(&self) {
        fetch_from!(self.ui(), group_members_tree_view, group_members_list_store);

        let data = self.data.clone();
        let ui = self.ui.clone();
        app_register_signal!(
            self,
            group_members_tree_view,
            group_members_tree_view.connect_row_activated(clone!(@strong ui, @strong data, @weak group_members_tree_view => move |_self, treepath, treeview_column| {
        let delete_column = group_members_tree_view
            .get_column(1)
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

            let current_group_name = ui.lock().unwrap().current_group().as_ref().expect("Current group should be set before performing any action on a group").name();
            return_if_err!(data.lock().unwrap()
                .remove_entity_from_group(current_group_name, entity_to_remove));
        }
            })));
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
