pub mod activities_treeview_config;

mod drag;
mod update;
mod update_entities_list_store;

use crate::app::ui::{
    activities::activities_treeview_config::*,
    helpers::{collections::get_next_element, format::format_time_spin_button},
    Ui,
};

use felix_backend::data::{Activity, Data};

use gtk::prelude::*;

impl Ui {
    pub(super) fn on_init_activities(&mut self) {
        self.update_current_activity(&Vec::new(), None);
        self.expand_activity_groups_tree_view_name_col();
        self.expand_activity_entities_tree_view_name_col();
        self.set_duration_spinbutton_format();
    }

    fn expand_activity_groups_tree_view_name_col(&self) {
        fetch_from!(self, activity_groups_tree_view);
        activity_groups_tree_view
            .get_column(ACTIVITY_GROUPS_NAME_COLUMN)
            .unwrap()
            .set_expand(true);
    }

    fn expand_activity_entities_tree_view_name_col(&self) {
        fetch_from!(self, activity_entities_tree_view);
        activity_entities_tree_view
            .get_column(ACTIVITY_ENTITIES_NAME_COLUMN)
            .unwrap()
            .set_expand(true);
    }

    fn set_duration_spinbutton_format(&self) {
        fetch_from!(
            self,
            activity_duration_hour_spin,
            activity_duration_minute_spin
        );
        for spinbutton in &[activity_duration_hour_spin, activity_duration_minute_spin] {
            format_time_spin_button(spinbutton);
        }
    }

    pub fn on_activity_added(&mut self, data: &Data, activity: &Activity) {
        self.update_current_activity(&data.groups_sorted(), Some(activity.clone()));
        self.update_activities_treeview(data.activities_sorted());
    }

    pub fn on_activity_selected(&mut self, data: &Data, activity: Activity) {
        self.update_current_activity(&data.groups_sorted(), Some(activity));
    }

    pub fn on_activity_removed(&mut self, data: &Data, position_of_removed_activity: usize) {
        let activities = data.activities_sorted();
        let (new_current_activity, _) = get_next_element(position_of_removed_activity, &activities);
        self.update_current_activity(&data.groups_sorted(), new_current_activity);
        self.update_activities_treeview(activities);
    }

    pub fn on_activity_renamed(&mut self, data: &Data, activity: &Activity) {
        self.update_current_activity_without_ui(Some(activity.clone()));
        self.update_activities_treeview(data.activities_sorted());
    }

    pub fn on_group_members_changed_update_activity(&mut self, data: &Data) {
        if let Some(current_activity) = &self.current_activity {
            let activity = data.activity(current_activity.id()).clone();
            self.update_current_activity(&data.groups_sorted(), Some(activity));
        }
    }

    pub fn on_activity_changed_update_current_activity(
        &mut self,
        data: &Data,
        activity: &Activity,
    ) {
        self.update_current_activity(&data.groups_sorted(), Some(activity.clone()));
    }

    pub fn on_entities_or_groups_changed(&mut self, data: &Data) {
        self.update_entity_group_completion_list_store(data);
        self.refresh_current_activity_view(data);
    }

    pub fn refresh_current_activity_view(&mut self, data: &Data) {
        if let Some(current_activity) = &self.current_activity {
            let new_current_activity = data
                .activities_not_sorted()
                .into_iter()
                .find(|activity| activity.id() == current_activity.id());

            self.update_current_activity(&data.groups_sorted(), new_current_activity);
        }
    }
}
