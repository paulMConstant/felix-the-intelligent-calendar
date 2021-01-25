use glib::clone;
use gtk::prelude::*;

use crate::app::notify::notify_err;
use crate::app::ui::helpers::tree::get_selection_from_treeview;
use crate::app::App;
use felix_backend::data::{clean_string, ActivityID, Time};
use felix_backend::errors::does_not_exist::DoesNotExist;
use std::convert::TryFrom;

impl App {
    pub fn connect_activities_tab(&self) {
        self.connect_add_activity();
        self.connect_activity_selected();
        self.connect_remove_activity();
        self.connect_rename_activity();
        self.connect_set_activity_duration();
        self.connect_add_to_activity();
        self.connect_remove_group_from_activity();
        self.connect_remove_entity_from_activity();

        self.connect_clean_activity_entries();
    }

    fn connect_add_activity(&self) {
        macro_rules! add_activity_closure {
            ($data: ident, $ui: ident, $activity_add_entry: ident) => {
                clone!(@strong $data, @strong $ui, @weak $activity_add_entry => move |_| {
                    let activity_name = $activity_add_entry.get_text();
                    with_blocked_signals!($ui.lock().unwrap(), $activity_add_entry.set_text(""), $activity_add_entry);

                    no_notify_assign_or_return!(activity_name, clean_string(activity_name));
                    return_if_err!($data.lock().unwrap().add_activity(activity_name));
                })
            };
        }

        fetch_from!(self.ui(), activity_add_button, activity_add_entry);

        let data = self.data.clone();
        let ui = self.ui.clone();

        app_register_signal!(
            self,
            activity_add_button,
            activity_add_button.connect_clicked(add_activity_closure!(
                data,
                ui,
                activity_add_entry
            ))
        );

        app_register_signal!(
            self,
            activity_add_entry,
            activity_add_entry.connect_activate(add_activity_closure!(
                data,
                ui,
                activity_add_entry
            ))
        );
    }

    fn connect_activity_selected(&self) {
        fetch_from!(self.ui(), activities_tree_view);

        let data = self.data.clone();
        let ui = self.ui.clone();
        app_register_signal!(
            self,
            activities_tree_view,
            activities_tree_view.connect_cursor_changed(
                clone!(@strong ui, @strong data, @weak activities_tree_view => move |_| {
                let selected_activity_id = get_selection_from_treeview(activities_tree_view);
                if let Some(activity_id_str) = selected_activity_id {
                    let activity_id = activity_id_str
                        .parse::<ActivityID>()
                        .expect("Error when parsing activity ID from model");

                    let data = data.lock().unwrap();
                    assign_or_return!(activity, data.activity(activity_id));
                    ui.lock().unwrap().on_activity_selected(&data, activity.clone());
                }
                })
            )
        );
    }

    fn connect_remove_activity(&self) {
        fetch_from!(self.ui(), activity_remove_button);

        let data = self.data.clone();
        let ui = self.ui.clone();
        app_register_signal!(
            self,
            activity_remove_button,
            activity_remove_button.connect_clicked(clone!(@strong data, @strong ui => move |_| {
                let id = ui.lock().unwrap().current_activity().expect(
                    "Current activity should be selected before accessing the remove activity button",
                ).id();
                return_if_err!(data.lock().unwrap().remove_activity(id));
            }))
        );
    }

    fn connect_rename_activity(&self) {
        fetch_from!(self.ui(), activity_name_entry);

        let data = self.data.clone();
        let ui = self.ui.clone();
        app_register_signal!(
            self,
            activity_name_entry,
            activity_name_entry.connect_changed(clone!(@strong ui, @strong data, @weak activity_name_entry => move |_| {
            let activity_to_rename_id = ui.lock().unwrap()
                .current_activity()
                .expect("Current activity should be selected before accessing the activity name entry")
                .id();
            let new_name = activity_name_entry.get_text();
            no_notify_assign_or_return!(new_name, clean_string(new_name));
            return_if_err!(data.lock().unwrap().set_activity_name(activity_to_rename_id, new_name));
            }))
        );
    }

    fn connect_set_activity_duration(&self) {
        fetch_from!(
            self.ui(),
            activity_duration_minute_spin,
            activity_duration_hour_spin
        );

        let data = self.data.clone();
        let ui = self.ui.clone();

        macro_rules! set_duration_closure {
            ($data:ident, $ui:ident, $minutes_spin:ident, $hours_spin:ident) => {
                safe_spinbutton_to_i8!($minutes_spin => minutes, $hours_spin => hours);

                let id = $ui
                    .lock()
                    .unwrap()
                    .current_activity()
                    .expect("Current activity should be set before setting duration")
                    .id();
                let mut data = $data.lock().unwrap();
                if let Err(e) = data.set_activity_duration(id, Time::new(hours, minutes)) {
                    let duration = data
                        .activity(id)
                        .expect("If current activity is set then id is valid")
                        .duration();
                    $minutes_spin.set_value(duration.minutes() as f64);
                    $hours_spin.set_value(duration.hours() as f64);
                    notify_err(e);
                }
            };
        }

        app_register_signal!(
            self,
            activity_duration_minute_spin,
            activity_duration_minute_spin.connect_changed(clone!(@strong data, @strong ui,
                                                                 @weak activity_duration_minute_spin, @weak activity_duration_hour_spin=> move |_| {
                set_duration_closure!(data, ui, activity_duration_minute_spin, activity_duration_hour_spin);
            }))
        );

        fetch_from!(
            self.ui(),
            activity_duration_minute_spin,
            activity_duration_hour_spin
        );

        app_register_signal!(
            self,
            activity_duration_hour_spin,
            activity_duration_hour_spin.connect_changed(
                clone!(@strong data, @strong ui, @weak activity_duration_minute_spin,
                           @weak activity_duration_hour_spin=> move |_| {
                    set_duration_closure!(data,
                                          ui,
                                          activity_duration_minute_spin,
                                          activity_duration_hour_spin);
                })
            )
        );
    }

    fn connect_add_to_activity(&self) {
        fetch_from!(self.ui(), activity_add_to_entry);

        let data = self.data.clone();
        let ui = self.ui.clone();

        macro_rules! add_to_activity_closure {
            ($data: ident, $ui: ident, $entry: ident) => {
            clone!(@strong $data, @strong $ui, @weak $entry => move |_| {
                let activity_id = $ui
                    .lock()
                    .unwrap()
                    .current_activity()
                    .expect("Current activity should be set before adding into it")
                    .id();
                let entity_or_group_to_add = $entry.get_text();
                with_blocked_signals!($ui.lock().unwrap(), $entry.set_text(""), $entry);

                no_notify_assign_or_return!(
                    entity_or_group_to_add,
                    clean_string(entity_or_group_to_add)
                );

                let mut data = $data.lock().unwrap();
                if let Ok(entity) = data.entity(&entity_or_group_to_add) {
                    let entity_name = entity.name();
                    return_if_err!(data.add_entity_to_activity(activity_id, entity_name));
                } else if let Ok(group) = data.group(&entity_or_group_to_add) {
                    let group_name = group.name();
                    return_if_err!(data.add_group_to_activity(activity_id, group_name));
                } else {
                    let err = DoesNotExist::entity_does_not_exist(entity_or_group_to_add);
                    notify_err(err);
                }
            })
            };
        }

        app_register_signal!(
            self,
            activity_add_to_entry,
            activity_add_to_entry.connect_activate(add_to_activity_closure!(
                data,
                ui,
                activity_add_to_entry
            ))
        );

        fetch_from!(self.ui(), activity_add_to_entry, activity_add_to_button);

        app_register_signal!(
            self,
            activity_add_to_button,
            activity_add_to_button.connect_clicked(add_to_activity_closure!(
                data,
                ui,
                activity_add_to_entry
            ))
        );
    }

    fn connect_remove_group_from_activity(&self) {
        fetch_from!(
            self.ui(),
            activity_groups_tree_view,
            activity_groups_list_store
        );

        let data = self.data.clone();
        let ui = self.ui.clone();
        app_register_signal!(
            self,
            activity_groups_tree_view,
            activity_groups_tree_view.connect_row_activated(
                clone!(@strong ui, @strong data, @weak activity_groups_tree_view
                       => move |_self, treepath, treeview_column| {
        let delete_column = activity_groups_tree_view
            .get_column(1)
            .expect("Activity Groups tree view should have at least 2 columns");
        if &delete_column == treeview_column {
            let iter = activity_groups_list_store
                .get_iter(treepath)
                .expect("Row was activated, path should be valid");
            let group_to_remove = activity_groups_list_store.get_value(&iter, 0);
            let group_to_remove = group_to_remove
                .get::<&str>()
                .expect("Value should be gchararray")
                .expect("Value should be gchararray");

            let current_activity_id = ui.lock().unwrap().current_activity().as_ref()
                .expect("Current activity should be set before performing any action on a group").id();
            return_if_err!(data.lock().unwrap()
                .remove_group_from_activity(current_activity_id, group_to_remove));
        }
            })));
    }

    fn connect_remove_entity_from_activity(&self) {
        fetch_from!(
            self.ui(),
            activity_entities_tree_view,
            activity_entities_list_store
        );

        let data = self.data.clone();
        let ui = self.ui.clone();
        app_register_signal!(
            self,
            activity_entities_tree_view,
            activity_entities_tree_view.connect_row_activated(
                clone!(@strong ui, @strong data, @weak activity_entities_tree_view =>
                       move |_self, treepath, treeview_column| {
        let delete_column = activity_entities_tree_view
            .get_column(1)
            .expect("Activity Entities tree view should have at least 2 columns");
        if &delete_column == treeview_column {
            let iter = activity_entities_list_store
                .get_iter(treepath)
                .expect("Row was activated, path should be valid");
            let entity_to_remove = activity_entities_list_store.get_value(&iter, 0);
            let entity_to_remove = entity_to_remove
                .get::<&str>()
                .expect("Value should be gchararray")
                .expect("Value should be gchararray");

            let current_activity_id = ui.lock().unwrap().current_activity().as_ref()
                .expect("Current activity should be set before performing any action on a group").id();

            let mut data = data.lock().unwrap();
            assign_or_return!(activity, data.activity(current_activity_id));
            let activity_entities = activity.entities_sorted();

            if activity_entities.contains(&entity_to_remove.to_owned()) {
                return_if_err!(data.remove_entity_from_activity(current_activity_id, entity_to_remove));
            } else {
                // The entity was removed from a group of the activity and should be readded.
                return_if_err!(data.add_entity_to_activity(current_activity_id, entity_to_remove));
            }
        }
            })));
    }

    fn connect_clean_activity_entries(&self) {
        connect_clean!(
            self,
            activity_add_entry,
            activity_add_to_entry,
            activity_name_entry
        );
    }
}
