pub mod groups_treeview_config;
mod update;

use crate::app::ui::{groups_treeview_config::*, helpers::collections::get_next_element, Ui};

use felix_backend::data::{Data, Entity, Group};

use gtk::prelude::*;

impl Ui {
    pub(super) fn on_init_groups(&mut self) {
        self.update_current_group(None);
        self.expand_group_members_tree_view_name_col();
    }

    fn expand_group_members_tree_view_name_col(&self) {
        fetch_from!(self, group_members_tree_view);
        group_members_tree_view
            .get_column(GROUP_NAME_COLUMN)
            .unwrap()
            .set_expand(true);
    }

    pub fn on_group_added(&mut self, data: &Data, group: &Group) {
        self.update_current_group(Some(group.clone()));
        self.update_groups_treeview(&data.groups_sorted());
    }

    pub fn on_group_selected(&mut self, group: Group) {
        self.update_current_group(Some(group));
    }

    pub fn on_group_removed(&mut self, data: &Data, position_of_removed_group: usize) {
        let groups = &data.groups_sorted();
        let (new_current_group, _) = get_next_element(position_of_removed_group, groups);
        self.update_current_group(new_current_group);
        self.update_groups_treeview(groups);
    }

    pub fn on_group_renamed(&mut self, data: &Data, group: &Group) {
        self.update_current_group_without_ui(Some(group.clone()));
        self.update_groups_treeview(&data.groups_sorted());
    }

    pub fn on_entity_in_group_renamed(&mut self, data: &Data, entity: &Entity, _old_name: &str) {
        self.on_group_members_changed(data, entity);
    }

    pub fn on_entity_in_group_removed(
        &mut self,
        data: &Data,
        position: usize,
        _name_of_removed_entity: &str,
    ) {
        self.on_group_members_changed(data, position);
    }

    pub fn on_group_members_changed<T>(&mut self, data: &Data, _anything: T) {
        if let Some(group) = &self.current_group {
            let updated_group = data.group(group.name()).expect(
                "A group with the current group name should exist if only its members changed",
            );
            self.update_current_group(Some(updated_group));
        }
    }
}
