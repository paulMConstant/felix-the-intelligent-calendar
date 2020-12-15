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

    fn update_current_activity_view(&self) {}

    fn hide_current_activity_view(&self) {
        fetch_from!(self, activity_specific_pane);
        activity_specific_pane.hide();
    }
}
