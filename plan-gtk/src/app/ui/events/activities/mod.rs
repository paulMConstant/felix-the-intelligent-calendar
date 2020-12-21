mod update;

use crate::app::ui::Ui;

use plan_backend::data::Activity;

impl Ui {
    pub(super) fn on_init_activities(&mut self) {
        self.update_current_activity(None);
    }

    pub fn on_add_activity(&mut self, activity: Activity, activities: Vec<&Activity>) {
        self.update_current_activity(Some(activity));
        self.update_activities_treeview(activities);
    }

    pub fn on_activity_selected(&mut self, activity: Activity) {
        self.update_current_activity(Some(activity));
    }

    pub fn on_activity_removed(&mut self) {
        //let (new_current_activity, position_of_new_current_activity) =
        //get_next_element(position_of_removed_activity, self.data.activities_sorted());
        //self.update_current_activity(&new_current_activity);
        //self.update_activities_treeview(position_of_new_current_activity);
    }

    pub fn on_activity_renamed(&mut self, activities: Vec<&Activity>) {
        //self.update_current_activity_without_ui(Some(activity_to_rename_id));
        self.update_activities_treeview(activities);
    }

    pub fn on_entity_added_to_activity(&mut self) {
        self.update_current_activity_entities();
    }

    fn on_group_added_to_activity(&mut self) {
        self.update_current_activity_groups();
        // Entities in the group are added to the activity, so need to refresh the view as well
        self.update_current_activity_entities();
    }
}
