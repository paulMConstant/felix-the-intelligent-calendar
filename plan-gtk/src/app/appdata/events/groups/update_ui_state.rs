use crate::app::appdata::{events::helpers::tree_path_from_selection_index, AppData};
use plan_backend::data::Group;

use gtk::prelude::*;

impl AppData {
    pub(in super::super) fn update_current_group_members(&mut self) {
        fetch_from!(self, group_members_tree_view, group_members_list_store);

        let current_group = self
            .state
            .current_group
            .as_ref()
            .expect("Current group should be set before updating the fields");
        assign_or_return!(current_group, self.data.group(current_group));

        with_blocked_signals!(
            self,
            {
                group_members_list_store.clear();
                for entity_name in current_group.entities_sorted() {
                    group_members_list_store.insert_with_values(
                        None,
                        &[0, 1],
                        &[&entity_name, &"user-trash-symbolic"],
                    );
                }
            },
            group_members_tree_view
        );
    }

    pub(super) fn update_current_group(&mut self, group: &Option<Group>) {
        match group {
            Some(group) => {
                self.state.current_group = Some(group.name());
                self.update_current_group_view();
            }
            None => {
                self.state.current_group = None;
                self.hide_current_group_view();
            }
        };
    }

    pub(super) fn update_groups_treeview(&self, selection_row: Option<i32>) {
        self.update_groups_list_store();
        self.update_groups_treeview_selection(selection_row);
    }

    fn update_groups_list_store(&self) {
        fetch_from!(self, groups_list_store, groups_tree_view);

        with_blocked_signals!(
            self,
            {
                groups_list_store.clear();
                for group_name in self
                    .data
                    .groups_sorted()
                    .into_iter()
                    .map(|group| group.name())
                {
                    groups_list_store.insert_with_values(None, &[0], &[&group_name]);
                }
            },
            groups_tree_view
        );
    }

    fn update_groups_treeview_selection(&self, selection_index: Option<i32>) {
        fetch_from!(self, groups_tree_view, groups_list_store);

        let selection_tree_path = tree_path_from_selection_index(
            selection_index,
            groups_list_store,
            self.state.current_group.as_ref(),
        );
        let focus_column = None::<&gtk::TreeViewColumn>;
        with_blocked_signals!(
            self,
            groups_tree_view.set_cursor(&selection_tree_path, focus_column, false),
            groups_tree_view
        );
    }

    fn update_current_group_view(&mut self) {
        fetch_from!(self, group_specific_box, group_name_entry);
        let current_group = self
            .state
            .current_group
            .as_ref()
            .expect("Current group should be set before updating the fields");

        group_specific_box.show();

        with_blocked_signals!(
            self,
            group_name_entry.set_text(current_group),
            group_name_entry
        );
        self.update_current_group_members();
    }

    fn hide_current_group_view(&self) {
        fetch_from!(self, group_specific_box);
        group_specific_box.hide();
    }
}
