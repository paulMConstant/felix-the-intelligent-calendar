use crate::app::App;

use glib::clone;

use std::sync::{Arc, Mutex};

impl App {
    pub(super) fn connect_data_events(&mut self) {
        let events = self.data.lock().unwrap().events();
        let mut events = events.borrow_mut();
        let ui = &self.ui;

        events.connect_entity_added(Box::new(clone!(@strong ui => move |data, entity| {
            let mut ui = ui.lock().unwrap();
            ui.on_entity_added(data, entity);
            ui.on_entities_or_groups_changed(data);
        })));

        events.connect_entity_renamed(Box::new(
            clone!(@strong ui => move |data, entity, _old_name| {
                let mut ui = ui.lock().unwrap();
                ui.on_entity_renamed(data, entity);
                ui.on_group_members_changed(data);
                ui.on_entities_or_groups_changed(data);
            }),
        ));

        events.connect_entity_removed(Box::new(clone!(@strong ui => move |data, position, name| {
            let mut ui = ui.lock().unwrap();
            ui.on_entity_removed(data, position);
            ui.on_group_members_changed(data);
            ui.on_entities_or_groups_changed(data);
            ui.on_entity_removed_update_schedules(name);
        })));

        events.connect_group_added(Box::new(clone!(@strong ui => move |data, group| {
            let mut ui = ui.lock().unwrap();
            ui.on_group_added(data, group);
            ui.on_entities_or_groups_changed(data);
        })));

        events.connect_group_renamed(Box::new(clone!(@strong ui => move |data, group| {
            let mut ui = ui.lock().unwrap();
            ui.on_group_renamed(data, group);
            ui.on_entities_or_groups_changed(data);
        })));

        events.connect_group_removed(Box::new(clone!(@strong ui => move |data, position| {
            let mut ui = ui.lock().unwrap();
            ui.on_group_removed(data, position);
            ui.on_entities_or_groups_changed(data);
        })));

        events.connect_entity_added_to_group(Box::new(clone!(@strong ui => move |data, _group| {
            let mut ui = ui.lock().unwrap();
            ui.on_group_members_changed(data);
            ui.on_group_members_changed_update_activity(data);
        })));

        events.connect_entity_removed_from_group(Box::new(
            clone!(@strong ui => move |data, _group| {
                let mut ui = ui.lock().unwrap();
                ui.on_group_members_changed(data);
                ui.on_group_members_changed_update_activity(data);
            }),
        ));

        events.connect_activity_added(Box::new(clone!(@strong ui => move |data, activity| {
            let mut ui = ui.lock().unwrap();
            ui.on_activity_added(data, activity);
        })));

        events.connect_activity_removed(Box::new(clone!(@strong ui => move |data, position| {
            let mut ui = ui.lock().unwrap();
            ui.on_activity_removed(data, position);
            ui.update_schedules(data);
        })));

        events.connect_activity_renamed(Box::new(clone!(@strong ui => move |data, activity| {
            let mut ui = ui.lock().unwrap();
            ui.on_activity_renamed(data, activity);
            ui.update_schedules(data);
        })));

        // Data stored in the closure which polls to insert an activity after its duration has been
        // updated
        let app = self.clone();
        let polling_duration_counter = Arc::new(Mutex::new(0));
        events.connect_activity_duration_changed(Box::new(clone!(@strong ui
                                                                 => move |data, activity| {
            let mut ui = ui.lock().unwrap();
            ui.on_activity_changed_update_current_activity(data, activity);
            ui.update_schedules(data);
            app.on_activity_duration_changed_start_polling_to_insert_it_again(
                data, polling_duration_counter.clone());
        })));

        events.connect_activity_inserted(Box::new(clone!(@strong ui => move |data, activity| {
            let mut ui = ui.lock().unwrap();
            ui.on_activity_changed_update_current_activity(data, activity);
            ui.update_schedules(data);
        })));

        events.connect_activity_color_changed(Box::new(
            clone!(@strong ui => move |data, _activity| {
                let mut ui = ui.lock().unwrap();
                ui.update_schedules(data);
            }),
        ));

        events.connect_entity_added_to_activity(Box::new(
            clone!(@strong ui => move |data, activity| {
                let mut ui = ui.lock().unwrap();
                ui.on_activity_changed_update_current_activity(data, activity);
                ui.update_schedules(data);
            }),
        ));

        events.connect_entity_removed_from_activity(Box::new(
            clone!(@strong ui => move |data, activity| {
                let mut ui = ui.lock().unwrap();
                ui.on_activity_changed_update_current_activity(data, activity);
                ui.update_schedules(data);
            }),
        ));

        events.connect_group_added_to_activity(Box::new(
            clone!(@strong ui => move |data, activity| {
                let mut ui = ui.lock().unwrap();
                ui.on_activity_changed_update_current_activity(data, activity);
                ui.update_schedules(data);
            }),
        ));

        events.connect_group_removed_from_activity(Box::new(
            clone!(@strong ui => move |data, activity| {
                let mut ui = ui.lock().unwrap();
                ui.on_activity_changed_update_current_activity(data, activity);
                ui.update_schedules(data);
            }),
        ));

        events.connect_group_removed_from_activity(Box::new(
            clone!(@strong ui => move |data, activity| {
                let mut ui = ui.lock().unwrap();
                ui.on_activity_changed_update_current_activity(data, activity);
                ui.update_schedules(data);
            }),
        ));

        events.connect_work_hours_changed(Box::new(clone!(@strong ui => move |data| {
            let mut ui = ui.lock().unwrap();
            ui.on_work_hours_changed(data);
            ui.update_schedules(data);
        })));

        events.connect_custom_work_hours_changed(Box::new(clone!(@strong ui => move |data| {
            let mut ui = ui.lock().unwrap();
            ui.on_custom_work_hours_changed(data);
            ui.update_schedules(data);
        })));
    }
}
