mod update_ui_state;

use crate::app::appdata::AppData;
use plan_backend::data::clean_string;

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
}
