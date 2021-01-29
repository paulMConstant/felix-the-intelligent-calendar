use crate::app::ui::helpers::tree::tree_path_from_selection_index;
use felix_backend::data::Group;

use crate::app::ui::Ui;
use gettextrs::gettext as tr;
use gtk::prelude::*;

impl Ui {
    pub(super) fn update_current_group_name_only(&mut self, group: Option<Group>) {
        self.current_group = group;

        if let Some(group) = &self.current_group {
            fetch_from!(self, add_to_group_button);
            add_to_group_button.set_label(&format!("{} '{}'", tr("Add to"), group.name()));
        }
    }

    pub(super) fn update_current_group(&mut self, group: Option<Group>) {
        self.update_current_group_name_only(group);

        if self.current_group.is_some() {
            self.update_current_group_view();
        } else {
            self.hide_current_group_view();
        };
    }

    fn update_current_group_view(&self) {
        fetch_from!(self, group_name_entry);
        let current_group = self
            .current_group
            .as_ref()
            .expect("Current group should be set before updating the fields");

        self.show_current_group_view();

        with_blocked_signals!(
            self,
            group_name_entry.set_text(&current_group.name()),
            group_name_entry
        );
        self.update_current_group_members();
    }

    pub(super) fn update_current_group_members(&self) {
        fetch_from!(self, group_members_tree_view, group_members_list_store);

        if let Some(current_group) = self.current_group.as_ref() {
            with_blocked_signals!(
                self,
                {
                    group_members_list_store.clear();
                    for entity_name in current_group.entities_sorted() {
                        group_members_list_store.insert_with_values(
                            None,
                            &[0, 1],
                            &[&entity_name, &"action-unavailable-symbolic"],
                        );
                    }
                },
                group_members_tree_view
            );
        }
    }

    fn show_current_group_view(&self) {
        fetch_from!(self, group_specific_box, add_entity_to_group_box);
        group_specific_box.show();
        add_entity_to_group_box.show();
    }

    fn hide_current_group_view(&self) {
        fetch_from!(self, group_specific_box, add_entity_to_group_box);
        group_specific_box.hide();
        add_entity_to_group_box.hide();
    }

    pub(super) fn update_groups_treeview(&self, groups: &Vec<&Group>) {
        self.update_groups_list_store(groups);
        self.update_groups_treeview_selection();
    }

    fn update_groups_list_store(&self, groups: &Vec<&Group>) {
        fetch_from!(self, groups_list_store, groups_tree_view);

        with_blocked_signals!(
            self,
            {
                groups_list_store.clear();
                for group_name in groups.into_iter().map(|group| group.name()) {
                    groups_list_store.insert_with_values(None, &[0], &[&group_name]);
                }
            },
            groups_tree_view
        );
    }

    fn update_groups_treeview_selection(&self) {
        fetch_from!(self, groups_tree_view, groups_list_store);

        let current_group = self.current_group.as_ref();
        if let Some(group) = current_group {
            let selection_tree_path =
                tree_path_from_selection_index(None, groups_list_store, Some(group.name()));
            let focus_column = None::<&gtk::TreeViewColumn>;
            with_blocked_signals!(
                self,
                groups_tree_view.set_cursor(&selection_tree_path, focus_column, false),
                groups_tree_view
            );
        }
    }
}
