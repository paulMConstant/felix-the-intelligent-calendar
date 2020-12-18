use glib::clone;
use gtk::prelude::*;

use super::helpers::get_selection_from_treeview;
use crate::app::notify::notify_err;
use crate::app::App;
use plan_backend::data::{clean_string, ActivityID, Time};
use plan_backend::errors::does_not_exist::DoesNotExist;
use std::convert::TryFrom;

impl App {
    pub fn connect_activities_tab(&self) {
        self.connect_add_activity();
        self.connect_activity_selected();
        self.connect_remove_activity();
        self.connect_rename_activity();
        self.connect_set_activity_duration();
        self.connect_add_to_activity();

        self.connect_clean_add_activity_entry();
        self.connect_clean_activity_name_entry();
        self.connect_clean_activity_add_to_entry();
    }

    fn connect_add_activity(&self) {
        fetch_from!(self.ui(), activity_add_button, activity_add_entry);

        let data = self.data.clone();
        let ui = self.ui.clone();
        app_register_signal!(
            self,
            activity_add_button,
            activity_add_button.connect_clicked(clone!(@strong data, @strong ui, @weak activity_add_entry => move |_| {
                let activity_name = activity_add_entry.get_text();
                with_blocked_signals!(ui.lock().unwrap(), activity_add_entry.set_text(""), activity_add_entry);

                no_notify_assign_or_return!(activity_name, clean_string(activity_name));
                return_if_err!(data.lock().unwrap().add_activity(activity_name));
                                                                    }))
        );

        app_register_signal!(
            self,
            activity_add_entry,
            activity_add_entry.connect_activate(clone!(@strong data, @strong ui, @weak activity_add_entry => move |_| {

                let activity_name = activity_add_entry.get_text();
                with_blocked_signals!(ui.lock().unwrap(), activity_add_entry.set_text(""), activity_add_entry);

                no_notify_assign_or_return!(activity_name, clean_string(activity_name));
                return_if_err!(data.lock().unwrap().add_activity(activity_name));
                             }))
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
                    assign_or_return!(activity, data.lock().unwrap().activity(activity_id));
                    ui.lock().unwrap().on_activity_selected(activity.clone());
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

    fn connect_clean_add_activity_entry(&self) {
        fetch_from!(self.ui(), activity_add_entry);

        let ui = self.ui.clone();
        app_register_signal!(
            self,
            activity_add_entry,
            activity_add_entry.connect_changed(
                clone!(@strong ui, @weak activity_add_entry => move |_| {
                ui.lock().unwrap().event_clean_entry_content(activity_add_entry);
                                     })
            )
        );
    }

    fn connect_clean_activity_name_entry(&self) {
        fetch_from!(self.ui(), activity_name_entry);

        let ui = self.ui.clone();
        app_register_signal!(
            self,
            activity_name_entry,
            activity_name_entry.connect_changed(
                clone!(@strong ui, @weak activity_name_entry => move |_| {

                ui.lock().unwrap().event_clean_entry_content(activity_name_entry);
                                         })
            )
        );
    }

    fn connect_clean_activity_add_to_entry(&self) {
        fetch_from!(self.ui(), activity_add_to_entry);

        let ui = self.ui.clone();
        app_register_signal!(
            self,
            activity_add_to_entry,
            activity_add_to_entry.connect_changed(
                clone!(@strong ui, @weak activity_add_to_entry => move |_| {
                    ui.lock().unwrap().event_clean_entry_content(activity_add_to_entry);
                })
            )
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
        app_register_signal!(
            self,
            activity_duration_minute_spin,
            activity_duration_minute_spin.connect_changed(clone!(@strong data, @strong ui,
                                                                 @weak activity_duration_minute_spin, @weak activity_duration_hour_spin=> move |_| {
        let minutes = i8::try_from(activity_duration_minute_spin.get_value().trunc() as i64)
            .expect("Spin value should be between 0 and 55");
        let hours = i8::try_from(activity_duration_hour_spin.get_value().trunc() as i64)
            .expect("Spin value should be between 0 and 23");

        let id = ui.lock().unwrap().current_activity()
            .expect("Current activity should be set before setting duration").id();
        return_if_err!(data.lock().unwrap().set_activity_duration(id, Time::new(hours, minutes)));
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
            activity_duration_hour_spin.connect_changed(clone!(@strong data, @strong ui,
                                                                 @weak activity_duration_minute_spin, @weak activity_duration_hour_spin=> move |_| {
        let minutes = i8::try_from(activity_duration_minute_spin.get_value().trunc() as i64)
            .expect("Spin value should be between 0 and 55");
        let hours = i8::try_from(activity_duration_hour_spin.get_value().trunc() as i64)
            .expect("Spin value should be between 0 and 23");

        let id = ui.lock().unwrap().current_activity()
            .expect("Current activity should be set before setting duration").id();
        return_if_err!(data.lock().unwrap().set_activity_duration(id, Time::new(hours, minutes)));
            }))
        );
    }

    fn connect_add_to_activity(&self) {
        fetch_from!(self.ui(), activity_add_to_entry);

        let data = self.data.clone();
        let ui = self.ui.clone();
        app_register_signal!(
            self,
            activity_add_to_entry,
            activity_add_to_entry.connect_activate(clone!(@strong ui, @strong data, @weak activity_add_to_entry => move |_| {
                let activity_id = ui.lock().unwrap().current_activity().expect("Current activity should be set before adding into it").id();
        let entity_or_group_to_add = activity_add_to_entry.get_text();
        with_blocked_signals!(
            ui.lock().unwrap(),
            activity_add_to_entry.set_text(""),
            activity_add_to_entry
        );

        no_notify_assign_or_return!(entity_or_group_to_add, clean_string(entity_or_group_to_add));

        if let Ok(entity) = data.lock().unwrap().entity(&entity_or_group_to_add) {
            let entity_name = entity.name();
            data.lock().unwrap().add_entity_to_activity(activity_id, entity_name);
        } else if let Ok(group) = data.lock().unwrap().group(&entity_or_group_to_add) {
            let group_name = group.name();
            data.lock().unwrap().add_entity_to_activity(activity_id, group_name);
        } else {
            let err = DoesNotExist::entity_does_not_exist(entity_or_group_to_add);
            notify_err(err);
        }
            }))
        );

        fetch_from!(self.ui(), activity_add_to_entry, activity_add_to_button);
        app_register_signal!(
            self,
            activity_add_to_button,
            activity_add_to_button.connect_clicked(clone!(@strong data, @strong ui, @weak activity_add_to_entry => move |_| {
                let activity_id = ui.lock().unwrap().current_activity().expect("Current activity should be set before adding into it").id();
        let entity_or_group_to_add = activity_add_to_entry.get_text();
        with_blocked_signals!(
            ui.lock().unwrap(),
            activity_add_to_entry.set_text(""),
            activity_add_to_entry
        );

        no_notify_assign_or_return!(entity_or_group_to_add, clean_string(entity_or_group_to_add));

        if let Ok(entity) = data.lock().unwrap().entity(&entity_or_group_to_add) {
            let entity_name = entity.name();
            data.lock().unwrap().add_entity_to_activity(activity_id, entity_name);
        } else if let Ok(group) = data.lock().unwrap().group(&entity_or_group_to_add) {
            let group_name = group.name();
            data.lock().unwrap().add_entity_to_activity(activity_id, group_name);
        } else {
            let err = DoesNotExist::entity_does_not_exist(entity_or_group_to_add);
            notify_err(err);
        }
            }))
        );
    }
}
