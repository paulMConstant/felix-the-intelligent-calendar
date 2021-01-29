use crate::app::{notify::notify_err, ui::EntityToShow, App};

use felix_backend::data::{clean_string, ActivityID, Time};
use felix_backend::errors::does_not_exist::DoesNotExist;

use std::sync::Arc;

use glib::clone;
use gtk::prelude::*;

impl App {
    pub fn connect_activity_insertion(&self) {
        self.connect_show_schedule();
        self.set_activity_try_insert_callback();

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

        let ui = self.ui.clone();
        let data = self.data.clone();

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

    fn set_activity_try_insert_callback(&self) {
        let data = self.data.clone();

        self.ui
            .lock()
            .unwrap()
            .set_activity_try_insert_callback(Arc::new(Box::new(
                clone!(@strong data => move
                        |entity_name: String, activity_id: ActivityID, insertion_time: Time| {
            let data = data.lock().unwrap();
            let activity = data.activity(activity_id)
                .expect("The activity we are inserting does not exist");

            if activity.entities_sorted().contains(&entity_name) == false {
                // Inserting activity for wrong entity
                return;
            }

            if activity.possible_insertion_beginnings().contains(&insertion_time) == false {
                // Inserting activity at wrong time
                return;
            }
            // TODO
             //data.insert_activity(activity_id, insetion_time)
                 //.expect("Error while inserting activity, should have been checked for);
            println!("Insert activity ID {} at time {} for entity {}", activity_id, insertion_time, entity_name);
        }))));
    }

    fn connect_clean_show_schedule_entry(&self) {
        connect_clean!(self, show_schedule_entry);
    }
}
