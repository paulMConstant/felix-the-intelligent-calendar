mod update;

use crate::app::ui::helpers::get_next_element;
use crate::app::ui::Ui;

use plan_backend::data::{Activity, Data};

impl Ui {
    pub(super) fn on_init_activities(&mut self) {
        self.update_current_activity(None);
    }

    pub fn on_activity_added(&mut self, data: &Data, activity: &Activity) {
        self.update_current_activity(Some(activity.clone()));
        self.update_activities_treeview(&data.activities_sorted());
    }

    pub fn on_activity_selected(&mut self, activity: Activity) {
        self.update_current_activity(Some(activity));
    }

    pub fn on_activity_removed(&mut self, data: &Data, position_of_removed_activity: usize) {
        let activities = &data.activities_sorted();
        let (new_current_activity, _) = get_next_element(position_of_removed_activity, activities);
        self.update_current_activity(new_current_activity);
        self.update_activities_treeview(&activities);
    }

    pub fn on_activity_renamed(&mut self, data: &Data, activity: &Activity) {
        self.update_current_activity_name_only(Some(activity.clone()));
        self.update_activities_treeview(&data.activities_sorted());
    }

    pub fn on_activity_entities_changed(&mut self, _data: &Data, activity: &Activity) {
        self.update_current_activity(Some(activity.clone()));
        self.update_current_activity_entities();
    }

    pub fn on_activity_groups_changed(&mut self, _data: &Data, activity: &Activity) {
        self.update_current_activity(Some(activity.clone()));
        self.update_current_activity_groups();
        // Entities in the group are added to the activity, so need to refresh the view as well
        self.update_current_activity_entities();
    }

    pub fn on_entities_or_groups_changed<T>(&mut self, data: &Data, _anything: T) {
        self.update_activities_completion_list_store(data);

        if let Some(current_activity) = &self.current_activity {
            let new_current_activity = data
                .activities_sorted()
                .into_iter()
                .find(|activity| activity.id() == current_activity.id());
            self.update_current_activity(new_current_activity.cloned());
            self.update_current_activity_entities();
            self.update_current_activity_groups();
        }
    }
}
