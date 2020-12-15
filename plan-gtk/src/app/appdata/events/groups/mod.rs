pub mod update_ui_state;

use super::helpers::get_selection_from_treeview;
use crate::app::appdata::AppData;
use plan_backend::data::{clean_string, Group};

use gtk::prelude::*;
use std::convert::TryFrom;

impl AppData {
    pub(super) fn event_init_groups(&mut self) {
        self.update_current_group(&None);
        self.expand_group_members_tree_view_name_col();
    }

    fn expand_group_members_tree_view_name_col(&self) {
        fetch_from!(self, group_members_tree_view);
        group_members_tree_view
            .get_column(0)
            .unwrap()
            .set_expand(true);
    }

    pub fn event_add_group(&mut self) {
        fetch_from!(self, group_add_entry);
        let group_to_add = group_add_entry.get_text();
        with_blocked_signals!(self, group_add_entry.set_text(""), group_add_entry);

        no_notify_assign_or_return!(group_to_add, clean_string(group_to_add));
        assign_or_return!(group_name, self.data.add_group(group_to_add));
        assign_or_return!(group, self.data.group(&group_name));

        // Fetch where the group was added
        let position_of_new_group = self
            .data
            .groups_sorted()
            .into_iter()
            .position(|group| group.name() == group_name)
            .expect("The group was added successfuly so this should be valid");

        let position_of_new_group = i32::try_from(position_of_new_group)
            .expect("There should not be 2 billion groups, we should be safe");

        let group = group.clone();
        self.update_current_group(&Some(group));
        self.update_groups_treeview(Some(position_of_new_group));
    }

    pub fn event_group_selected(&mut self) {
        fetch_from!(self, groups_tree_view);
        let selected_group = get_selection_from_treeview(groups_tree_view);
        if let Some(group_name) = selected_group {
            assign_or_return!(group, self.data.group(group_name));
            let group = group.clone();
            self.update_current_group(&Some(group));
        }
    }

    pub fn event_remove_group(&mut self) {
        // TODO same as entities
        let group_to_remove =
            self.state.current_group.as_ref().expect(
                "Current group should be selected before accessing any group-related filed",
            );
        let position_of_removed_group = self
            .data
            .groups_sorted()
            .iter()
            .position(|other_groups| other_groups.name() == *group_to_remove);
        return_if_err!(self.data.remove_group(group_to_remove));

        let position_of_removed_group = position_of_removed_group
            .expect("Group existed because it was removed, therefore this is valid");
        let groups = self.data.groups_sorted();
        let len = groups.len();

        let (new_current_group, position_of_new_current_group) = if len == 0 {
            //No group left
            (None::<Group>, None::<i32>)
        } else {
            let position_of_new_current_group = if len <= position_of_removed_group {
                // The removed group was the last. Show the previous one.
                position_of_removed_group - 1
            } else {
                // Show the next group
                position_of_removed_group
            };

            let new_current_group = Some(groups[position_of_new_current_group].clone());
            let position_of_next_group = i32::try_from(position_of_new_current_group)
                .expect("There should not be 2 billion groups, we should be safe");

            (new_current_group, Some(position_of_next_group))
        };

        self.update_current_group(&new_current_group);
        self.update_groups_treeview(position_of_new_current_group);
    }

    pub fn event_rename_group(&mut self) {
        fetch_from!(self, group_name_entry);
        let group_to_rename =
            self.state.current_group.as_ref().expect(
                "Current group should be selected before accessing any group-related field",
            );
        let new_name = group_name_entry.get_text();
        no_notify_assign_or_return!(
            new_name,
            self.data.set_group_name(group_to_rename, new_name)
        );
        self.update_current_group_without_ui(Some(new_name));
        self.update_groups_treeview(None);
    }

    pub fn event_add_entity_to_group(&mut self) {
        fetch_from!(
            self,
            entity_into_group_name_entry,
            create_entity_before_adding_to_group_switch
        );
        let group_in_which_to_add =
            self.state.current_group.as_ref().expect(
                "Current group should be selected before accessing any group-related filed",
            );
        let entity_name = entity_into_group_name_entry.get_text();
        with_blocked_signals!(
            self,
            entity_into_group_name_entry.set_text(""),
            entity_into_group_name_entry
        );

        no_notify_assign_or_return!(entity_name, clean_string(entity_name));
        if create_entity_before_adding_to_group_switch.get_active() {
            if let Err(_) = self.data.entity(&entity_name) {
                return_if_err!(self.data.add_entity(&entity_name));
                self.update_entities_treeview(None);
            }
        }

        return_if_err!(self
            .data
            .add_entity_to_group(group_in_which_to_add, entity_name));

        self.update_current_group_members();
    }

    pub fn event_remove_entity_from_group(
        &mut self,
        path: &gtk::TreePath,
        col: &gtk::TreeViewColumn,
    ) {
        fetch_from!(self, group_members_list_store, group_members_tree_view);
        let delete_column = group_members_tree_view
            .get_column(1)
            .expect("Group Members tree view should have at least 2 columns");
        if &delete_column == col {
            let iter = group_members_list_store
                .get_iter(path)
                .expect("Row was activated, path should be valid");
            let entity_to_remove = group_members_list_store.get_value(&iter, 0);
            let entity_to_remove = entity_to_remove
                .get::<&str>()
                .expect("Value should be gchararray")
                .expect("Value should be gchararray");

            let current_group_name = self
                .state
                .current_group
                .as_ref()
                .expect("Current group should be set before performing any action on a group");
            return_if_err!(self
                .data
                .remove_entity_from_group(current_group_name, entity_to_remove));

            self.update_current_group_members();
        }
    }
}
