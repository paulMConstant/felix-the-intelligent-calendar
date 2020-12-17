mod update_ui_state;

use super::helpers::{get_next_element, get_selection_from_treeview};
use crate::app::app_data::AppData;
use crate::app::notify::notify_err;
use plan_backend::data::{clean_string, ActivityID, Time};
use plan_backend::errors::does_not_exist::DoesNotExist;

use gtk::prelude::*;
use std::convert::TryFrom;

impl AppData {
    pub(super) fn event_init_activities(&mut self) {
        self.update_current_activity(&None);
    }

    pub fn event_add_activity(&mut self) {
        fetch_from!(self, activity_add_entry);
        let activity_name = activity_add_entry.get_text();
        with_blocked_signals!(self, activity_add_entry.set_text(""), activity_add_entry);

        no_notify_assign_or_return!(activity_name, clean_string(activity_name));
        assign_or_return!(activity, self.data.add_activity(activity_name));
        let activity = activity.clone();

        // Fetch where the activity was added
        let position_of_new_activity = self
            .data
            .activities_sorted()
            .into_iter()
            .position(|other_activity| other_activity == &activity)
            .expect("The activity was added succesfully so this should be valid");

        let position_of_new_activity = i32::try_from(position_of_new_activity)
            .expect("There should not be 2 billion activities, we should be safe");

        self.update_current_activity(&Some(activity));
        self.update_activities_treeview(Some(position_of_new_activity));
    }

    pub fn event_activity_selected(&mut self) {
        fetch_from!(self, activities_tree_view);
        let selected_activity_id = get_selection_from_treeview(activities_tree_view);
        if let Some(activity_id_str) = selected_activity_id {
            let activity_id = activity_id_str
                .parse::<ActivityID>()
                .expect("Error when parsing activity ID from model");
            assign_or_return!(activity, self.data.activity(activity_id));
            let activity = activity.clone();
            self.update_current_activity(&Some(activity));
        }
    }

    pub fn event_remove_activity(&mut self) {
        let activity_to_remove_id = self.state.current_activity_id.as_ref().expect(
            "Current activity should be selected before accessing the remove activity button",
        );
        let position_of_removed_activity = self
            .data
            .activities_sorted()
            .iter()
            .position(|other_activity| other_activity.id() == *activity_to_remove_id);
        return_if_err!(self.data.remove_activity(*activity_to_remove_id));

        let position_of_removed_activity = position_of_removed_activity
            .expect("Activity existed because it was removed therefore it had a position");

        let (new_current_activity, position_of_new_current_activity) =
            get_next_element(position_of_removed_activity, self.data.activities_sorted());
        self.update_current_activity(&new_current_activity);
        self.update_activities_treeview(position_of_new_current_activity);
    }

    pub fn event_rename_activity(&mut self) {
        fetch_from!(self, activity_name_entry);
        let activity_to_rename_id = self
            .state
            .current_activity_id
            .as_ref()
            .expect("Current activity should be selected before accessing the activity name entry")
            .clone();
        let new_name = activity_name_entry.get_text();

        no_notify_return_if_err!(self.data.set_activity_name(activity_to_rename_id, new_name));
        self.update_current_activity_without_ui(Some(activity_to_rename_id));
        self.update_activities_treeview(None);
    }

    pub fn event_set_activity_duration(&mut self) {
        fetch_from!(
            self,
            activity_duration_minute_spin,
            activity_duration_hour_spin
        );
        let minutes = i8::try_from(activity_duration_minute_spin.get_value().trunc() as i64)
            .expect("Spin value should be between 0 and 55");
        let hours = i8::try_from(activity_duration_hour_spin.get_value().trunc() as i64)
            .expect("Spin value should be between 0 and 23");

        let id = self
            .state
            .current_activity_id
            .expect("Current activity should be set before setting duration");
        return_if_err!(self
            .data
            .set_activity_duration(id, Time::new(hours, minutes)));
    }

    pub fn event_add_to_activity(&mut self) {
        fetch_from!(self, activity_add_to_entry);

        let entity_or_group_to_add = activity_add_to_entry.get_text();
        with_blocked_signals!(
            self,
            activity_add_to_entry.set_text(""),
            activity_add_to_entry
        );

        no_notify_assign_or_return!(entity_or_group_to_add, clean_string(entity_or_group_to_add));

        if let Ok(entity) = self.data.entity(&entity_or_group_to_add) {
            let entity_name = entity.name();
            self.add_entity_to_activity(entity_name);
        } else if let Ok(group) = self.data.group(&entity_or_group_to_add) {
            let group_name = group.name();
            self.add_group_to_activity(group_name);
        } else {
            let err = DoesNotExist::entity_does_not_exist(entity_or_group_to_add);
            notify_err(err);
        }
    }

    fn add_entity_to_activity(&mut self, entity_name: String) {
        let current_activity = self
            .state
            .current_activity_id
            .expect("Current activity should be set before adding entities to it");
        return_if_err!(self
            .data
            .add_entity_to_activity(current_activity, entity_name));
        self.update_current_activity_entities();
    }

    fn add_group_to_activity(&mut self, group_name: String) {
        let current_activity = self
            .state
            .current_activity_id
            .expect("Current activity should be set before adding groups to it");
        return_if_err!(self
            .data
            .add_group_to_activity(current_activity, group_name));
        self.update_current_activity_groups();
        // Entities in the group are added to the activity, so need to refresh the view as well
        self.update_current_activity_entities();
    }
}

// TODO bouton activité qui change de nom + màj durée non valide
