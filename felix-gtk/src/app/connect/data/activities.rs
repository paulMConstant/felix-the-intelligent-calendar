use crate::app::App;

use felix_backend::data::Data;

use gettextrs::gettext as tr;
use glib::clone;
use gtk::prelude::*;

use std::cell::RefCell;
use std::rc::Rc;

impl App {
    pub(in super::super) fn connect_activity_events(&self) {
        let events = self.data.borrow().events();
        let mut events = events.borrow_mut();

        events.connect_activity_added(Box::new(
            clone!(@strong self.ui as ui => move |data, activity| {
                let mut ui = ui.borrow_mut();
                ui.on_activity_added(data, activity);
            }),
        ));

        events.connect_activity_removed(Box::new(
            clone!(@strong self.ui as ui => move |data, position| {
                let mut ui = ui.borrow_mut();
                ui.on_activity_removed(data, position);
                ui.update_schedules(data);
            }),
        ));

        events.connect_activity_renamed(Box::new(
            clone!(@strong self.ui as ui => move |data, activity| {
                let mut ui = ui.borrow_mut();
                ui.on_activity_renamed(data, activity);
                ui.update_schedules(data);
            }),
        ));

        // Data stored in the closure which polls to insert an activity after its duration has been
        // updated
        let app = self.clone();
        let polling_duration_counter = Rc::new(RefCell::new(0));
        events.connect_activity_duration_changed(Box::new(
            clone!(@strong self.ui as ui => move |data, activity| {
                let mut ui = ui.borrow_mut();
                ui.on_activity_changed_update_current_activity(data, activity);
                ui.update_schedules(data);
                app.on_activity_duration_changed_start_polling_to_insert_it_again(
                    data, polling_duration_counter.clone());
            }),
        ));

        events.connect_activity_inserted(Box::new(
            clone!(@strong self.ui as ui => move |data, activity| {
                let mut ui = ui.borrow_mut();
                ui.on_activity_changed_update_current_activity(data, activity);
                ui.update_schedules(data);
            }),
        ));

        events.connect_activity_color_changed(Box::new(
            clone!(@strong self.ui as ui => move |data, _activity| {
                let mut ui = ui.borrow_mut();
                ui.update_schedules(data);
            }),
        ));

        events.connect_entity_added_to_activity(Box::new(
            clone!(@strong self.ui as ui => move |data, activity| {
                let mut ui = ui.borrow_mut();
                ui.on_activity_changed_update_current_activity(data, activity);
                ui.update_schedules(data);
            }),
        ));

        events.connect_entity_removed_from_activity(Box::new(
            clone!(@strong self.ui as ui => move |data, activity| {
                let mut ui = ui.borrow_mut();
                ui.on_activity_changed_update_current_activity(data, activity);
                ui.update_schedules(data);
            }),
        ));

        events.connect_group_added_to_activity(Box::new(
            clone!(@strong self.ui as ui => move |data, activity| {
                let mut ui = ui.borrow_mut();
                ui.on_activity_changed_update_current_activity(data, activity);
                ui.update_schedules(data);
            }),
        ));

        events.connect_group_removed_from_activity(Box::new(
            clone!(@strong self.ui as ui => move |data, activity| {
                let mut ui = ui.borrow_mut();
                ui.on_activity_changed_update_current_activity(data, activity);
                ui.update_schedules(data);
            }),
        ));

        events.connect_autoinsertion_done(Box::new(clone!(@strong self.ui as ui => move |data| {
            let mut ui = ui.borrow_mut();
            ui.update_schedules(data);
            fetch_from!(ui, autoinsert_button);
            autoinsert_button.set_label(&tr("Auto-insert"));
        })));
    }

    fn on_activity_duration_changed_start_polling_to_insert_it_again(
        &self,
        data: &Data,
        polling_duration_counter: Rc<RefCell<u32>>,
    ) {
        if data.activities_were_uninserted_and_can_maybe_be_inserted_back() {
            const FREQUENCY_CHECK_COMPUTATION_RESULT_DONE_MS: u32 = 5;
            const TIMEOUT_CHECK_COMPUTATION_RESULT_DONE_MS: u32 = 1000;
            const TIMEOUT_MAX_COUNTER_VALUE: u32 = TIMEOUT_CHECK_COMPUTATION_RESULT_DONE_MS
                / FREQUENCY_CHECK_COMPUTATION_RESULT_DONE_MS;

            let mut counter = polling_duration_counter.borrow_mut();
            if *counter == 0 {
                // The polling function is not currently running
                // Add one preemptively so that the function is never called twice
                *counter += 1;

                let data = self.data.clone();
                let counter = polling_duration_counter.clone();
                // Launch polling function
                glib::timeout_add_local(FREQUENCY_CHECK_COMPUTATION_RESULT_DONE_MS, move || {
                    let mut counter = counter.borrow_mut();
                    if *counter > TIMEOUT_MAX_COUNTER_VALUE {
                        *counter = 0;
                        data.borrow_mut()
                            .clear_list_activities_removed_because_duration_increased();
                        glib::Continue(false)
                    } else {
                        *counter += 1;
                        data.borrow_mut()
                            .insert_activities_removed_because_duration_increased_in_closest_spot();
                        glib::Continue(true)
                    }
                });
            } else {
                //Extend polling function duration
                *counter = 0;
            }
        }
    }
}
