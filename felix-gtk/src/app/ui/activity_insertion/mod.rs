pub mod activity_insertion_ui;
pub mod activity_to_show;
pub mod entity_to_show;

use crate::app::ui::{EntitiesAndInsertionTimes, Ui};
use entity_to_show::EntityToShow;

use felix_data::{ActivityId, Data, Entity, Time};

use glib::clone;
use gtk::prelude::*;

use gettextrs::gettext as tr;

use std::cell::RefCell;
use std::rc::Rc;

impl Ui {
    pub(super) fn on_init_activity_insertion(&self) {
        fetch_from!(self, insertion_box);
        // Put the activity insertion widget inside of the box
        insertion_box.pack_end(
            &self
                .activity_insertion
                .borrow()
                .get_activity_insertion_box(),
            true,
            true,
            0,
        );
    }

    pub fn set_activity_ui_callbacks(
        &mut self,
        possible_insertions_callback: Rc<dyn Fn(ActivityId) -> EntitiesAndInsertionTimes>,
        remove_activity_from_schedule_callback: Rc<dyn Fn(ActivityId)>,
    ) {
        self.setup_drag_from_activities_treeview(
            possible_insertions_callback.clone(),
            remove_activity_from_schedule_callback.clone(),
        );

        self.activity_insertion
            .borrow_mut()
            .setup_drag_from_schedules_drawing(
                possible_insertions_callback,
                remove_activity_from_schedule_callback,
            );
    }

    pub fn set_activity_try_insert_callback(
        &mut self,
        try_insert_activity_callback: Rc<dyn Fn(String, ActivityId, Time)>,
    ) {
        self.activity_insertion
            .borrow()
            .enable_drop(try_insert_activity_callback);
    }

    pub fn init_set_activity_duration_callback(&mut self, callback: Rc<dyn Fn(ActivityId, bool)>) {
        // Scrolling on activities with shift increases/decreases their duration
        // Connect events to check if shift is held
        macro_rules! connect_shift_held {
            ($shift_held: ident, $($window: ident),*) => {
                $(
                    $window.connect_key_press_event(clone!(@strong $shift_held
                                                      => move |_window, event| {
                    if event.get_keyval() == gdk::keys::constants::Shift_L {
                        *$shift_held.borrow_mut() = true;
                    }
                    glib::signal::Inhibit(false)
                }));

                $window.connect_key_release_event(clone!(@strong $shift_held
                                                        => move |_window, event| {
                    if event.get_keyval() == gdk::keys::constants::Shift_L {
                        *$shift_held.borrow_mut() = false;
                    }
                    glib::signal::Inhibit(false)
                }));
                )*
            }
        }

        let shift_held = Rc::new(RefCell::new(false));
        fetch_from!(self, main_window, data_window);
        connect_shift_held!(shift_held, main_window, data_window);

        self.activity_insertion
            .borrow()
            .connect_scroll_event(callback, shift_held);
    }

    pub fn on_show_entity_schedule(&mut self, entity_to_show: EntityToShow) {
        self.activity_insertion
            .borrow()
            .show_entities_schedule(vec![entity_to_show]);
    }

    pub fn update_schedules(&mut self, data: &Data) {
        let activity_insertion = self.activity_insertion.borrow();
        let entities_to_show: Vec<_> = activity_insertion
            .shown_entities()
            .iter()
            .map(|entity_name| EntityToShow::new(entity_name.clone(), data))
            .collect();

        activity_insertion.show_entities_schedule(entities_to_show);
    }

    pub fn on_entity_renamed_update_schedules(
        &mut self,
        data: &Data,
        entity: &Entity,
        old_name: &str,
    ) {
        let activity_insertion = self.activity_insertion.borrow();
        if activity_insertion
            .shown_entities()
            .contains(&old_name.into())
        {
            activity_insertion.remove_entity_schedule(old_name);
            let new_entity = EntityToShow::new(entity.name(), data);
            activity_insertion.show_entities_schedule(vec![new_entity]);
        }
    }

    pub fn on_entity_removed_update_schedules(&mut self, old_name: &str) {
        self.activity_insertion
            .borrow()
            .remove_entity_schedule(old_name);
    }

    pub fn on_left_click_over_schedules(&mut self, data: Rc<RefCell<Data>>, x: f64, y: f64) {
        self.activity_insertion
            .borrow()
            .update_activity_under_cursor(x, y);

        // Avoid multiple borrows => Introduce temp variable
        let maybe_activity = self.activity_insertion.borrow().get_activity_under_cursor();
        if let Some(activity) = maybe_activity {
            let activity = data.borrow().activity(activity.id());
            self.update_current_activity(&data.borrow().groups_sorted(), Some(activity));
        }
    }

    pub fn on_left_click_over_schedules_header(&mut self, x: f64, y: f64) {
        let maybe_entity_to_remove = self
            .activity_insertion
            .borrow()
            .get_entity_to_remove_under_cursor(x, y);
        if let Some(entity_to_remove) = maybe_entity_to_remove {
            self.activity_insertion
                .borrow_mut()
                .remove_entity_schedule(&entity_to_remove);
        }
    }

    pub fn on_right_click_over_schedules(&mut self, _data: Rc<RefCell<Data>>, x: f64, y: f64) {
        self.activity_insertion
            .borrow()
            .update_activity_under_cursor(x, y);
        //self.activity_insertion
        //.lock()
        //.unwrap()
        //.get_id_of_activity_under_cursor());
        // TODO pin activity
    }

    pub fn on_autoinsertion_done_update_state(&mut self) {
        fetch_from!(self, autoinsert_button);
        autoinsert_button.set_label(&tr("Auto-insert"));
        *self.autoinsertion_handle.borrow_mut() = None;
    }
}
