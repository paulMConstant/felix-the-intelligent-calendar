pub mod update;

use crate::app::ui::Ui;

use plan_backend::data::Group;

use gtk::prelude::*;

impl Ui {
    pub(super) fn on_init_groups(&mut self) {
        //self.update_current_group(&None);
        self.expand_group_members_tree_view_name_col();
    }

    fn expand_group_members_tree_view_name_col(&self) {
        fetch_from!(self, group_members_tree_view);
        group_members_tree_view
            .get_column(0)
            .unwrap()
            .set_expand(true);
    }

    pub fn on_add_group(&mut self) {
        // Fetch where the group was added
        //let position_of_new_group = self
        //.data
        //.groups_sorted()
        //.into_iter()
        //.position(|group| group.name() == group_name)
        //.expect("The group was added successfuly so this should be valid");

        //let position_of_new_group = i32::try_from(position_of_new_group)
        //.expect("There should not be 2 billion groups, we should be safe");

        //let group = group.clone();
        //self.update_current_group(&Some(group));
        //self.update_groups_treeview(Some(position_of_new_group));
        //self.update_activities_completion_list_store();
    }

    pub fn on_group_selected(&mut self, group: Group) {
        //self.update_current_group(&Some(group));
    }

    pub fn on_group_removed(&mut self) {
        //let position_of_removed_group = self
        //.data
        //.groups_sorted()
        //.iter()
        //.position(|other_groups| other_groups.name() == *group_to_remove);

        //let position_of_removed_group = position_of_removed_group
        //.expect("Group existed because it was removed, therefore this is valid");

        //let (new_current_group, position_of_new_current_group) =
        //get_next_element(position_of_removed_group, self.data.groups_sorted());
        //self.update_current_group(&new_current_group);
        //self.update_groups_treeview(position_of_new_current_group);
        //self.update_current_activity_groups();
        //self.update_activities_completion_list_store();
    }

    pub fn on_group_renamed(&mut self) {
        //self.update_current_group_name_only(Some(new_name));
        //self.update_groups_treeview(None);
        //self.update_current_activity_groups();
        //self.update_activities_completion_list_store();
    }

    pub fn on_entity_added_to_group(&mut self) {
        //self.update_current_group_members();
        //self.update_current_activity_entities();
    }

    pub fn on_entity_removed_from_group(&mut self) {
        //self.update_current_group_members();
        //self.update_current_activity_entities();
    }
}
