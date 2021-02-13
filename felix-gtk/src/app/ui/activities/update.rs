use crate::app::ui::helpers::tree::tree_path_from_selection_index;
use crate::app::ui::Ui;
use felix_backend::data::{Activity, Group};

use gettextrs::gettext as tr;
use gtk::prelude::*;

impl Ui {
    pub(super) fn update_current_activity_name_only(&mut self, activity: Option<Activity>) {
        self.current_activity = activity;

        if let Some(activity) = &self.current_activity {
            fetch_from!(self, activity_add_to_button);

            with_blocked_signals!(
                self,
                {
                    activity_add_to_button.set_label(&format!(
                        "{} '{}'",
                        tr("Add to"),
                        activity.name()
                    ));
                },
                activity_add_to_button
            );
        }
    }

    /// Updates the state of AppData and Activity-specific UI.
    pub(super) fn update_current_activity(
        &mut self,
        groups: &Vec<&Group>,
        activity: Option<Activity>,
    ) {
        self.update_current_activity_name_only(activity);

        if self.current_activity.is_some() {
            self.update_current_activity_view(groups);
        } else {
            self.hide_current_activity_view();
        }
    }

    /// Updates the treeview of activities and selects the given row if not None.
    /// If the given row is None, keeps the originally selected row.
    pub(super) fn update_activities_treeview(&mut self, activities: &Vec<&Activity>) {
        self.update_activities_list_store(activities);
        self.update_activities_treeview_selection();
    }

    fn update_activities_list_store(&self, activities: &Vec<&Activity>) {
        fetch_from!(self, activities_list_store, activities_tree_view);

        with_blocked_signals!(
            self,
            {
                activities_list_store.clear();
                for activity in activities {
                    activities_list_store.insert_with_values(
                        None,
                        &[0, 1],
                        &[&(activity.id() as u32), &activity.name()],
                    );
                }
            },
            activities_tree_view
        );
    }

    fn update_activities_treeview_selection(&self) {
        fetch_from!(self, activities_tree_view, activities_list_store);

        let current_activity_id_as_string = self
            .current_activity
            .as_ref()
            .and_then(|activity| Some(format!("{}", activity.id())));
        let selection_tree_path = tree_path_from_selection_index(
            None,
            activities_list_store,
            current_activity_id_as_string,
        );
        let focus_column = None::<&gtk::TreeViewColumn>;
        with_blocked_signals!(
            self,
            activities_tree_view.set_cursor(&selection_tree_path, focus_column, false),
            activities_tree_view
        );
    }

    fn update_current_activity_view(&self, groups: &Vec<&Group>) {
        fetch_from!(
            self,
            activity_specific_box,
            activity_name_entry,
            activity_duration_hour_spin,
            activity_duration_minute_spin,
            activity_beginning_hour_spin,
            activity_beginning_minute_spin,
            activity_end_hour_spin,
            activity_end_minute_spin,
            activity_inserted_switch,
            activity_insertion_time_box,
            activity_color_button
        );

        activity_specific_box.show();

        let activity = self
            .current_activity
            .as_ref()
            .expect("Current activity ID should be set before updating activity view");

        with_blocked_signals!(
            self,
            {
                activity_name_entry.set_text(&activity.name());

                let activity_duration = activity.duration();
                activity_duration_hour_spin.set_value(activity_duration.hours() as f64);
                activity_duration_minute_spin.set_value(activity_duration.minutes() as f64);
                let activity_color = activity.color();
                activity_color_button.set_rgba(&gdk::RGBA {
                    red: activity_color.red,
                    green: activity_color.green,
                    blue: activity_color.blue,
                    alpha: activity_color.alpha,
                });

                if let Some(interval) = activity.insertion_interval() {
                    activity_inserted_switch.set_active(true);
                    activity_insertion_time_box.set_visible(true);

                    let beginning = interval.beginning();
                    activity_beginning_hour_spin.set_value(beginning.hours() as f64);
                    activity_beginning_minute_spin.set_value(beginning.minutes() as f64);

                    let end = interval.end();
                    activity_end_hour_spin.set_value(end.hours() as f64);
                    activity_end_minute_spin.set_value(end.minutes() as f64);
                } else {
                    activity_inserted_switch.set_active(false);
                    activity_insertion_time_box.set_visible(false);
                    for spinbutton in &[
                        &activity_beginning_hour_spin,
                        &activity_beginning_minute_spin,
                        &activity_end_hour_spin,
                        &activity_end_minute_spin,
                    ] {
                        spinbutton.set_value(0.0);
                    }
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

        self.update_current_activity_entities(groups);
        self.update_current_activity_groups();
    }

    pub(super) fn update_current_activity_groups(&self) {
        fetch_from!(self, activity_groups_list_store, activity_groups_tree_view);
        if let Some(activity) = &self.current_activity {
            let groups = activity.groups_sorted();

            with_blocked_signals!(
                self,
                {
                    activity_groups_list_store.clear();
                    for group in groups {
                        activity_groups_list_store.insert_with_values(
                            None,
                            &[0, 1],
                            &[&group, &"action-unavailable-symbolic"],
                        );
                    }
                },
                activity_groups_tree_view
            );
        }
    }

    fn hide_current_activity_view(&self) {
        fetch_from!(self, activity_specific_box);
        activity_specific_box.hide();
    }
}
