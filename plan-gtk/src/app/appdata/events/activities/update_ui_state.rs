use crate::app::appdata::{events::helpers::tree_path_from_selection_index, AppData};
use plan_backend::data::{Activity, ActivityID};

use gtk::prelude::*;

impl AppData {
    pub(super) fn update_current_activity_without_ui(&mut self, activity_id: Option<ActivityID>) {
        self.state.current_activity_id = activity_id;
    }

    /// Updates the state of AppData and Activity-specific UI.
    pub(super) fn update_current_activity(&mut self, activity: &Option<Activity>) {
        self.update_current_activity_without_ui(activity.as_ref().map(|activity| activity.id()));

        if activity.is_some() {
            self.update_current_activity_view();
        } else {
            self.hide_current_activity_view();
        }
    }

    /// Updates the treeview of activities and selects the given row if not None.
    /// If the given row is None, keeps the originally selected row.
    pub(super) fn update_activities_treeview(&mut self, selection_row: Option<i32>) {
        self.update_activities_list_store();
        self.update_activities_treeview_selection(selection_row);
    }

    fn update_activities_list_store(&self) {
        fetch_from!(self, activities_list_store, activities_tree_view);

        with_blocked_signals!(
            self,
            {
                activities_list_store.clear();
                for activity in self.data.activities_sorted().into_iter() {
                    activities_list_store.insert_with_values(
                        None,
                        &[0, 1],
                        &[&activity.id(), &activity.name()],
                    );
                }
            },
            activities_tree_view
        );
    }

    fn update_activities_treeview_selection(&self, selection_index: Option<i32>) {
        fetch_from!(self, activities_tree_view, activities_list_store);
        let id_as_string = self
            .state
            .current_activity_id
            .and_then(|activity_id| Some(format!("{}", activity_id)));
        let selection_tree_path = tree_path_from_selection_index(
            selection_index,
            activities_list_store,
            id_as_string.as_ref(),
        );
        let focus_column = None::<&gtk::TreeViewColumn>;
        with_blocked_signals!(
            self,
            activities_tree_view.set_cursor(&selection_tree_path, focus_column, false),
            activities_tree_view
        );
    }

    fn update_current_activity_view(&self) {
        fetch_from!(
            self,
            activity_specific_pane,
            activity_name_entry,
            activity_duration_hour_spin,
            activity_duration_minute_spin,
            activity_beginning_hour_spin,
            activity_beginning_minute_spin,
            activity_end_hour_spin,
            activity_end_minute_spin,
            activity_inserted_switch,
            activity_insertion_time_box
        );

        activity_specific_pane.show();

        let activity_id = self
            .state
            .current_activity_id
            .expect("Current activity ID should be set before updating activity view");
        assign_or_return!(activity, self.data.activity(activity_id));

        with_blocked_signals!(
            self,
            {
                activity_name_entry.set_text(&activity.name());

                let activity_duration = activity.duration();
                activity_duration_hour_spin.set_value(activity_duration.hours() as f64);
                activity_duration_minute_spin.set_value(activity_duration.minutes() as f64);

                if let Some(interval) = activity.insertion_interval() {
                    activity_inserted_switch.set_active(true);
                    activity_insertion_time_box.show();

                    let beginning = interval.beginning();
                    activity_beginning_hour_spin.set_value(beginning.hours() as f64);
                    activity_beginning_minute_spin.set_value(beginning.minutes() as f64);

                    let end = interval.end();
                    activity_end_hour_spin.set_value(end.hours() as f64);
                    activity_end_minute_spin.set_value(end.minutes() as f64);
                } else {
                    activity_inserted_switch.set_active(false);
                    activity_insertion_time_box.hide();
                }
            },
            activity_name_entry,
            activity_duration_hour_spin,
            activity_duration_minute_spin,
            activity_beginning_hour_spin,
            activity_beginning_minute_spin,
            activity_end_hour_spin,
            activity_end_minute_spin,
            activity_inserted_switch,
            activity_insertion_time_box
        );

        self.update_current_activity_entities();
        self.update_current_activity_groups();
    }

    pub(in super::super) fn update_current_activity_entities(&self) {}

    pub(in super::super) fn update_current_activity_groups(&self) {}

    fn hide_current_activity_view(&self) {
        fetch_from!(self, activity_specific_pane);
        activity_specific_pane.hide();
    }

    pub(in super::super) fn update_activities_completion_list_store(&self) {
        fetch_from!(self, activity_participants_completion_list_store);
        activity_participants_completion_list_store.clear();
        for entity_name in self
            .data
            .entities_sorted()
            .into_iter()
            .map(|entity| entity.name())
        {
            activity_participants_completion_list_store.insert_with_values(
                None,
                &[0, 1],
                &[&entity_name, &format!("avatar-default-symbolic")],
            );
        }
        for group_name in self
            .data
            .groups_sorted()
            .into_iter()
            .map(|group| group.name())
        {
            activity_participants_completion_list_store.insert_with_values(
                None,
                &[0, 1],
                &[&group_name, &format!("system-users-symbolic")],
            );
        }
    }
}
