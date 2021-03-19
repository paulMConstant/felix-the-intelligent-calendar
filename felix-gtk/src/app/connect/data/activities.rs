use crate::app::App;

use glib::clone;

use std::cell::RefCell;
use std::rc::Rc;

impl App {
    pub(super) fn connect_activity_events(&self) {
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
    }
}
