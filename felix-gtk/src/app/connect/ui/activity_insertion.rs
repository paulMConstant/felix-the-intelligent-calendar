use crate::app::{
    connect::ui::wrap_duration::wrap_duration, ui::EntitiesAndInsertionTimes,
    ui::EntityToShow, App,
};

use felix_backend::data::{clean_string, ActivityId, Time, MIN_TIME_DISCRETIZATION};
use felix_backend::errors::does_not_exist::DoesNotExist;

use std::convert::TryFrom;
use std::rc::Rc;

use glib::clone;
use gtk::prelude::*;

impl App {
    pub fn connect_activity_insertion(&self) {
        self.connect_show_schedule();
        self.connect_clicks();
        self.connect_drag_enable();
        self.connect_insert_activity_switch();
        self.connect_change_duration_of_inserted_activity();

        self.set_activity_try_insert_callback();
        self.set_activity_remove_and_get_possible_insertions_callback();
        self.init_set_activity_duration_callback();

        self.connect_clean_show_schedule_entry();
    }

    fn connect_show_schedule(&self) {
        macro_rules! show_schedule_closure {
            ($data: ident, $ui: ident, $entry: ident) => {
                clone!(@strong $data, @strong $ui, @weak $entry => move |_| {
                let entity_or_group_to_show = $entry.get_text();
                let mut ui = $ui.borrow_mut();
                with_blocked_signals!(ui, $entry.set_text(""), $entry);

                no_notify_assign_or_return!(
                    entity_or_group_to_show,
                    clean_string(entity_or_group_to_show)
                );

                let data = $data.borrow();
                if let Ok(entity) = data.entity(&entity_or_group_to_show) {
                    ui.on_show_entity_schedule(EntityToShow::new(entity.name(), &data));
                } else if let Ok(group) = data.group(&entity_or_group_to_show) {
                    for entity_name in group.entities_sorted() {
                        ui.on_show_entity_schedule(EntityToShow::new(entity_name, &data));
                    }
                } else {
                    let err = DoesNotExist::entity_does_not_exist(entity_or_group_to_show);
                    ui.notify_err(err);
                }
                })
            };
        }

        fetch_from!(self.ui.borrow(), show_schedule_entry, show_schedule_button);

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

    fn connect_clicks(&self) {
        fetch_from!(
            self.ui.borrow().activity_insertion().borrow(),
            schedule_scrolled_window
        );

        let data = self.data.clone();
        let ui = self.ui.clone();

        app_register_signal!(
            self,
            schedule_scrolled_window,
            schedule_scrolled_window.connect_button_press_event(move |_window, event| {
                let (x, y) = event.get_position();

                const RIGHT_CLICK: u32 = 3;
                const LEFT_CLICK: u32 = 1;
                match event.get_button() {
                    RIGHT_CLICK => ui.borrow_mut().on_right_click(data.clone(), x, y),
                    LEFT_CLICK => ui.borrow_mut().on_left_click(data.clone(), x, y),
                    _ => { // Do nothing
                    }
                }
                glib::signal::Inhibit(false)
            })
        );
    }

    /// Enables drag on schedule\_scrolled\_window when clicking is done or when we enter the
    /// window.
    /// Drag is disabled automatically if we click on nothing.
    /// If we wanted to add drag on click, we would need to click twice on an activity to trigger
    /// the drag (once to enable it, one to trigger).
    /// Using this method, one click triggers the drag.
    fn connect_drag_enable(&self) {
        let activity_insertion = self.ui.borrow().activity_insertion();
        fetch_from!(activity_insertion.borrow(), schedule_scrolled_window);

        let schedule_scrolled_window_clone = schedule_scrolled_window.clone();
        app_register_signal!(
            self,
            schedule_scrolled_window_clone,
            schedule_scrolled_window_clone.connect_button_release_event(
                clone!(@strong activity_insertion => move |_, event| {

                    const LEFT_CLICK: u32 = 1;
                    if event.get_button() == LEFT_CLICK {
                        // Left click released => Prepare drag drop for next click
                        activity_insertion.borrow().enable_drag_from_schedules_drawing();
                    }
                    glib::signal::Inhibit(false)
                })
            )
        );

        app_register_signal!(
            self,
            schedule_scrolled_window,
            schedule_scrolled_window.connect_enter_notify_event(move |_window, _event| {
                // Enter the window => Prepare drag drop for next click
                activity_insertion
                    .borrow()
                    .enable_drag_from_schedules_drawing();
                glib::signal::Inhibit(false)
            })
        );
    }

    fn connect_insert_activity_switch(&self) {
        fetch_from!(self.ui.borrow(), activity_inserted_switch);

        let data = self.data.clone();
        let ui = self.ui.clone();

        app_register_signal!(
            self,
            activity_inserted_switch,
            activity_inserted_switch.connect_property_active_notify(move |switch| {
                let insert_activity = switch.get_active();
                let id = ui
                    .borrow()
                    .current_activity()
                    .expect("Current activity does not exist")
                    .id();
                if insert_activity {
                    // Try to insert the activity in the best spot
                    let mut data = data.borrow_mut();

                    if let Some(insertion_costs) = data.activity(id).insertion_costs() {
                        if let Some(best_spot) = insertion_costs
                            .iter()
                            .min_by_key(|insertion_cost| insertion_cost.cost)
                        {
                            return_if_err!(ui, data.insert_activity(id, Some(best_spot.beginning)));
                        } else {
                            // Insertion costs is empty
                            return_if_err!(ui, data.insert_activity(id, None));
                        }
                    } else {
                        // Insertion costs not computed yet
                        return_if_err!(ui, data.insert_activity(id, None));
                    }
                } else {
                    // Remove the activity from the schedule
                    return_if_err!(ui, data.borrow_mut().insert_activity(id, None));
                }
            })
        );
    }

    fn connect_change_duration_of_inserted_activity(&self) {
        fetch_from!(
            self.ui.borrow(),
            activity_beginning_hour_spin,
            activity_beginning_minute_spin
        );

        let data = &self.data;
        let ui = &self.ui;

        macro_rules! set_beginning_closure {
            ($data: ident, $ui: ident, $minutes_spin: ident, $hours_spin: ident) => {
                safe_spinbutton_to_i8!($minutes_spin => minutes, $hours_spin => hours);

                let id = $ui
                    .borrow()
                    .current_activity()
                    .expect("Current activity should be set before setting duration")
                    .id();

                let mut data = $data.borrow_mut();

                let activity_beginning = data
                    .activity(id)
                    .insertion_interval()
                    .expect("Changing the beginning of an activity which is not inserted")
                    .beginning();

                let new_beginning = wrap_duration(activity_beginning, Time::new(hours, minutes));

                if let Err(e) = data.insert_activity(id, Some(new_beginning)) {
                    $ui.borrow().notify_err(e);

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
            .borrow_mut()
            .set_activity_try_insert_callback(Rc::new(Box::new(
                move |entity_name: String, activity_id: ActivityId, insertion_time: Time| {
                    let mut data = data.borrow_mut();
                    let activity = data.activity(activity_id);

                    if activity.entities_sorted().contains(&entity_name) {
                        // Ignore errors - no spamming on the user when he drag drops
                        let _ = data.insert_activity(activity_id, Some(insertion_time));
                    }
                },
            )));
    }

    fn set_activity_remove_and_get_possible_insertions_callback(&self) {
        let data = self.data.clone();
        let possible_insertion_times_of_activity_callback =
            Rc::new(Box::new(move |id: ActivityId| {
                let data = data.borrow();
                let activity_participants = data.activity(id).entities_sorted();

                let maybe_possible_insertion_times = data.activity(id).insertion_costs();

                EntitiesAndInsertionTimes {
                    entities: activity_participants,
                    insertion_times: maybe_possible_insertion_times,
                }
            }));

        let data = self.data.clone();
        let remove_activity_from_schedule_callback = Rc::new(Box::new(move |id: ActivityId| {
            let mut data = data.borrow_mut();

            if data.activity(id).insertion_interval().is_some() {
                data.insert_activity(id, None)
                    .expect("Could not remove activity from schedule");
            }
        }));

        self.ui.borrow_mut().set_activity_ui_callbacks(
            possible_insertion_times_of_activity_callback,
            remove_activity_from_schedule_callback,
        );
    }

    fn init_set_activity_duration_callback(&self) {
        let data = self.data.clone();
        let ui = self.ui.clone();

        self.ui
            .borrow_mut()
            .init_set_activity_duration_callback(Rc::new(Box::new(
                move |id: ActivityId, increase_duration: bool| {
                    let mut data = data.borrow_mut();
                    let activity_duration = data.activity(id).duration();

                    let new_duration = if increase_duration {
                        activity_duration + MIN_TIME_DISCRETIZATION
                    } else {
                        activity_duration - MIN_TIME_DISCRETIZATION
                    };
                    return_if_err!(ui, data.set_activity_duration(id, new_duration));
                },
            )));
    }

    fn connect_clean_show_schedule_entry(&self) {
        connect_clean!(self, show_schedule_entry);
    }
}
