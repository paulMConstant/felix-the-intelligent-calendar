use crate::app::{
    connect::ui::wrap_duration::wrap_duration, notify::notify_err, ui::EntitiesAndInsertionTimes,
    ui::EntityToShow, App,
};

use felix_backend::data::{clean_string, ActivityId, Time, MIN_TIME_DISCRETIZATION};
use felix_backend::errors::does_not_exist::DoesNotExist;

use std::convert::TryFrom;
use std::sync::Arc;

use glib::clone;
use gtk::prelude::*;

impl App {
    pub fn connect_activity_insertion(&self) {
        self.connect_show_schedule();
        self.connect_activity_clicked();
        self.connect_insert_activity_switch();
        self.connect_change_duration_of_inserted_activity();

        self.set_activity_try_insert_callback();
        self.set_activity_get_possible_insertions_callback();
        self.init_set_activity_duration_callback();

        self.connect_clean_show_schedule_entry();
    }

    fn connect_show_schedule(&self) {
        macro_rules! show_schedule_closure {
            ($data: ident, $ui: ident, $entry: ident) => {
                clone!(@strong $data, @strong $ui, @weak $entry => move |_| {
                let entity_or_group_to_show = $entry.get_text();
                let mut ui = $ui.lock().unwrap();
                with_blocked_signals!(ui, $entry.set_text(""), $entry);

                no_notify_assign_or_return!(
                    entity_or_group_to_show,
                    clean_string(entity_or_group_to_show)
                );

                let data = $data.lock().unwrap();
                if let Ok(entity) = data.entity(&entity_or_group_to_show) {
                    ui.on_show_entity_schedule(EntityToShow::new(entity.name(), &data));
                } else if let Ok(group) = data.group(&entity_or_group_to_show) {
                    for entity_name in group.entities_sorted() {
                        ui.on_show_entity_schedule(EntityToShow::new(entity_name, &data));
                    }
                } else {
                    let err = DoesNotExist::entity_does_not_exist(entity_or_group_to_show);
                    notify_err(err);
                }
                })
            };
        }

        fetch_from!(self.ui(), show_schedule_entry, show_schedule_button);

        let ui = &self.ui;
        let data = &self.data;

        app_register_signal!(
            self,
            show_schedule_button,
            show_schedule_button.connect_clicked(show_schedule_closure!(
                data,
                ui,
                show_schedule_entry
            ))
        );

        app_register_signal!(
            self,
            show_schedule_entry,
            show_schedule_entry.connect_activate(show_schedule_closure!(
                data,
                ui,
                show_schedule_entry
            ))
        );
    }

    fn connect_activity_clicked(&self) {
        fetch_from!(self.ui(), main_window);

        let ui = self.ui.clone();
        let data = self.data.clone();

        app_register_signal!(
            self,
            main_window,
            main_window.connect_button_press_event(move |_window, event| {
                const RIGHT_CLICK: u32 = 3;
                const LEFT_CLICK: u32 = 1;

                match event.get_button() {
                    RIGHT_CLICK => ui.lock().unwrap().on_right_click(data.clone()),
                    LEFT_CLICK => ui.lock().unwrap().on_left_click(data.clone()),
                    _ => { // Do nothing
                    }
                }
                glib::signal::Inhibit(false)
            })
        );
    }

    fn connect_insert_activity_switch(&self) {
        fetch_from!(self.ui(), activity_inserted_switch);

        let data = self.data.clone();
        let ui = self.ui.clone();

        app_register_signal!(
            self,
            activity_inserted_switch,
            activity_inserted_switch.connect_property_active_notify(move |switch| {
                let insert_activity = switch.get_active();
                let id = ui
                    .lock()
                    .unwrap()
                    .current_activity()
                    .expect("Current activity does not exist")
                    .id();
                if insert_activity {
                    // TODO data.insert_activity_in_best_spot(id) ;
                } else {
                    return_if_err!(data.lock().unwrap().insert_activity(id, None));
                }
            })
        );
    }

    fn connect_change_duration_of_inserted_activity(&self) {
        fetch_from!(
            self.ui(),
            activity_beginning_hour_spin,
            activity_beginning_minute_spin
        );

        let data = &self.data;
        let ui = &self.ui;

        macro_rules! set_beginning_closure {
            ($data: ident, $ui: ident, $minutes_spin: ident, $hours_spin: ident) => {
                safe_spinbutton_to_i8!($minutes_spin => minutes, $hours_spin => hours);

                let id = $ui
                    .lock()
                    .unwrap()
                    .current_activity()
                    .expect("Current activity should be set before setting duration")
                    .id();

                let mut data = $data.lock().unwrap();

                let activity_beginning = data
                    .activity(id)
                    .expect("Setting duration of activity which does not exist")
                    .insertion_interval()
                    .expect("Changing the beginning of an activity which is not inserted")
                    .beginning();

                let new_beginning = wrap_duration(activity_beginning, Time::new(hours, minutes));

                if let Err(e) = data.insert_activity(id, Some(new_beginning)) {
                    notify_err(e);

                    // Update the spinbuttons to the old value
                    $minutes_spin.set_value(activity_beginning.minutes() as f64);
                    $hours_spin.set_value(activity_beginning.hours() as f64);
                }
            }
        }

        let minute_spin = activity_beginning_minute_spin.clone();
        app_register_signal!(
            self,
            minute_spin,
            minute_spin.connect_changed(clone!(@strong data,
                        @strong ui,
                        @weak activity_beginning_hour_spin
                        => move |activity_beginning_minute_spin| {
                 set_beginning_closure!(data,
                                        ui,
                                        activity_beginning_minute_spin,
                                        activity_beginning_hour_spin);
            }))
        );

        app_register_signal!(
            self,
            activity_beginning_hour_spin,
            activity_beginning_hour_spin.connect_changed(clone!(@strong data,
                    @strong ui,
                    @weak activity_beginning_minute_spin
                    => move |activity_beginning_hour_spin| {
             set_beginning_closure!(data,
                                    ui,
                                    activity_beginning_minute_spin,
                                    activity_beginning_hour_spin);
            }))
        );
    }

    fn set_activity_try_insert_callback(&self) {
        let data = self.data.clone();

        self.ui
            .lock()
            .unwrap()
            .set_activity_try_insert_callback(Arc::new(Box::new(
                move |entity_name: String, activity_id: ActivityId, insertion_time: Time| {
                    let mut data = data.lock().unwrap();
                    let activity = data
                        .activity(activity_id)
                        .expect("The activity we are inserting does not exist");

                    if activity.entities_sorted().contains(&entity_name) {
                        // Ignore errors - no spamming on the user when he drag drops
                        let _ = data.insert_activity(activity_id, Some(insertion_time));
                    }
                },
            )));
    }

    fn set_activity_get_possible_insertions_callback(&self) {
        let data = self.data.clone();
        let possible_insertion_times_of_activity_callback =
            Arc::new(Box::new(move |id: ActivityId| {
                let mut data = data.lock().unwrap();
                let activity_participants = data
                    .activity(id)
                    .expect(
                        "Trying to get possible insertion times of activity which does not exist !",
                    )
                    .entities_sorted();
                let maybe_possible_insertion_times =
                    data.possible_insertion_times_of_activity(id).expect(
                        "Trying to get possible insertion times of activity which does not exist !",
                    );
                EntitiesAndInsertionTimes {
                    entities: activity_participants,
                    insertion_times: maybe_possible_insertion_times,
                }
            }));

        let data = self.data.clone();
        let remove_activity_from_schedule_callback = Arc::new(Box::new(move |id: ActivityId| {
            // Error should never happen here
            data.lock()
                .unwrap()
                .insert_activity(id, None)
                .expect("Could not remove activity from schedule");
        }));

        self.ui.lock().unwrap().set_activity_ui_callbacks(
            possible_insertion_times_of_activity_callback,
            remove_activity_from_schedule_callback,
        );
    }

    fn init_set_activity_duration_callback(&self) {
        let data = self.data.clone();

        self.ui
            .lock()
            .unwrap()
            .init_set_activity_duration_callback(Arc::new(Box::new(
                move |id: ActivityId, increase_duration: bool| {
                    let mut data = data.lock().unwrap();
                    let activity_duration = data
                        .activity(id)
                        .expect("Setting duraion of activity which does not exist")
                        .duration();
                    let new_duration = if increase_duration {
                        activity_duration + MIN_TIME_DISCRETIZATION
                    } else {
                        activity_duration - MIN_TIME_DISCRETIZATION
                    };
                    return_if_err!(data.set_activity_duration(id, new_duration));
                },
            )));
    }

    fn connect_clean_show_schedule_entry(&self) {
        connect_clean!(self, show_schedule_entry);
    }
}
