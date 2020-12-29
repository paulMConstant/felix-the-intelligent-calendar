use crate::app::{notify::notify_err, App};

use plan_backend::data::clean_string;
use plan_backend::errors::does_not_exist::DoesNotExist;

use glib::clone;
use gtk::prelude::*;

impl App {
    pub fn connect_activity_insertion(&self) {
        self.connect_show_schedule();

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
                    ui.on_show_entity_schedule(entity.name());
                } else if let Ok(group) = data.group(&entity_or_group_to_show) {
                    for entity_name in group.entities_sorted() {
                        ui.on_show_entity_schedule(entity_name);
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

    fn connect_clean_show_schedule_entry(&self) {
        connect_clean!(self, show_schedule_entry);
    }
}
