pub mod update;

use crate::app::ui::helpers::get_next_element;
use crate::app::ui::Ui;

use plan_backend::data::Group;

use gtk::prelude::*;

impl Ui {
    pub(super) fn on_init_groups(&mut self) {
        self.update_current_group(None);
        self.expand_group_members_tree_view_name_col();
    }

    fn expand_group_members_tree_view_name_col(&self) {
        fetch_from!(self, group_members_tree_view);
        group_members_tree_view
            .get_column(0)
            .unwrap()
            .set_expand(true);
    }

    pub fn on_group_added(&mut self, group: &Group, groups: &Vec<&Group>) {
        self.update_current_group(Some(group.clone()));
        self.update_groups_treeview(groups);
        self.update_activities_completion_list_store();
    }

    pub fn on_group_selected(&mut self, group: Group) {
        self.update_current_group(Some(group));
    }

    pub fn on_group_removed(&mut self, position_of_removed_group: usize, groups: &Vec<&Group>) {
        let (new_current_group, _) = get_next_element(position_of_removed_group, groups);
        self.update_current_group(new_current_group);
        self.update_groups_treeview(groups);
        self.update_current_activity_groups();
        self.update_activities_completion_list_store();
    }

    pub fn on_group_renamed(&mut self, group: &Group, groups: &Vec<&Group>) {
        self.update_current_group_name_only(Some(group.clone()));
        self.update_groups_treeview(groups);
        self.update_current_activity_groups();
        self.update_activities_completion_list_store();
    }

    pub fn on_group_members_changed(&mut self) {
        self.update_current_group_members();
        self.update_current_activity_entities();
    }
}
