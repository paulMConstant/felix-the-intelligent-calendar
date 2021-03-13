mod drag;
mod drawing;
mod drop;
mod event_helpers;
mod fetch_activity_insertion_ui;
mod fetch_data_from_cursor_position;
mod schedules;

use crate::app::ui::{ActivityToDisplay, EntitiesAndInsertionTimes, EntityToShow};
use event_helpers::increase_duration_on_scroll;
use fetch_data_from_cursor_position::get_activity_under_cursor;
use schedules::Schedules;

use felix_backend::data::ActivityId;

use glib::clone;
use gtk::prelude::*;

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

const NUM_HOURS_IN_DAY: i32 = 24;

#[derive(Clone)]
pub struct ActivityInsertionUi {
    builder: gtk::Builder,
    schedules_to_show: Arc<Mutex<Schedules>>,
    activity_under_cursor: Rc<RefCell<Option<ActivityToDisplay>>>,
    possible_insertions_callback: Arc<dyn Fn(ActivityId) -> EntitiesAndInsertionTimes>,
    remove_activity_from_schedule_callback: Arc<dyn Fn(ActivityId)>,
}

impl ActivityInsertionUi {
    #[must_use]
    pub fn new() -> ActivityInsertionUi {
        let builder = gtk::Builder::new();
        builder
            .add_from_resource("/com/github/paulmconstant/felix/ui/activity_insertion.ui")
            .expect("Could not load ui file: activity_insertion.ui");

        let activity_insertion = ActivityInsertionUi {
            builder,
            schedules_to_show: Arc::new(Mutex::new(Schedules::new())),
            activity_under_cursor: Rc::new(RefCell::new(None)),
            possible_insertions_callback: Arc::new(Box::new(|_| {
                panic!("possible_insertions_callback was not initialized")
            })),
            remove_activity_from_schedule_callback: Arc::new(Box::new(|_| {
                panic!("remove_activity_from_schedule_callback was not initialized")
            })),
        };

        activity_insertion.connect_draw();
        activity_insertion.connect_schedule_window_scroll();

        activity_insertion
    }

    pub fn show_possible_activity_insertions(
        &self,
        concerned_entities_and_possible_insertion_times: EntitiesAndInsertionTimes,
    ) {
        let mut schedules = self.schedules_to_show.lock().unwrap();
        schedules.possible_activity_insertion_times =
            concerned_entities_and_possible_insertion_times.insertion_times;
        schedules.activity_insertion_concerned_entities =
            concerned_entities_and_possible_insertion_times.entities;
        drop(schedules);

        self.draw_schedules_sorted();
    }

    #[must_use]
    pub(super) fn get_activity_insertion_box(&self) -> gtk::Box {
        fetch_from!(self, activity_insertion_box);
        activity_insertion_box
    }

    #[must_use]
    pub(super) fn get_activity_under_cursor(&self) -> Option<ActivityToDisplay> {
        self.activity_under_cursor.borrow().clone()
    }

    #[must_use]
    pub(super) fn shown_entities(&self) -> Vec<String> {
        self.schedules_to_show
            .lock()
            .unwrap()
            .entities_to_show
            .iter()
            .map(|entity| entity.name().clone())
            .collect()
    }

    pub(super) fn show_entities_schedule(&self, entities_to_show: Vec<EntityToShow>) {
        let mut schedules_to_show = self.schedules_to_show.lock().unwrap();
        // First push all entities
        for entity_to_show in entities_to_show {
            if let Some(index) = schedules_to_show
                .entities_to_show
                .iter()
                .position(|entity| entity == &entity_to_show)
            {
                schedules_to_show.entities_to_show.remove(index);
            }

            schedules_to_show.entities_to_show.push(entity_to_show);
        }

        // Then sort and draw
        drop(schedules_to_show); // Unlock
        self.draw_schedules_sorted();
    }

    pub(super) fn remove_entity_schedule(&self, name_of_entity_to_remove: &str) {
        let mut schedules_to_show = self.schedules_to_show.lock().unwrap();
        if let Some(position) = schedules_to_show
            .entities_to_show
            .iter()
            .position(|entity| entity.name() == name_of_entity_to_remove)
        {
            schedules_to_show.entities_to_show.remove(position);
            drop(schedules_to_show);
            self.draw_schedules_sorted();
        }
    }

    fn draw_schedules_sorted(&self) {
        let mut schedules_to_show = self.schedules_to_show.lock().unwrap();

        schedules_to_show
            .entities_to_show
            .sort_by(|a, b| a.name().cmp(b.name()));

        fetch_from!(self, header_drawing, schedules_drawing);
        for drawing in &[header_drawing, schedules_drawing] {
            drawing.queue_draw();
        }
    }

    fn connect_schedule_window_scroll(&self) {
        fetch_from!(
            self,
            hours_scrolled_window,
            header_scrolled_window,
            schedule_scrolled_window
        );

        header_scrolled_window.get_hadjustment().unwrap()
           .connect_value_changed(clone!(@weak schedule_scrolled_window => move |hadjustment|
             schedule_scrolled_window.get_hadjustment().unwrap().set_value(hadjustment.get_value()))
        );

        schedule_scrolled_window
            .get_hadjustment()
            .unwrap()
            .connect_value_changed(clone!(@weak header_scrolled_window => move |hadjustment|
             header_scrolled_window.get_hadjustment().unwrap().set_value(hadjustment.get_value())));

        hours_scrolled_window.get_vadjustment().unwrap()
           .connect_value_changed(clone!(@weak schedule_scrolled_window => move |vadjustment|
             schedule_scrolled_window.get_vadjustment().unwrap().set_value(vadjustment.get_value()))
        );

        schedule_scrolled_window
            .get_vadjustment()
            .unwrap()
            .connect_value_changed(clone!(@weak hours_scrolled_window => move |vadjustment|
             hours_scrolled_window.get_vadjustment().unwrap().set_value(vadjustment.get_value())));
    }

    pub(super) fn connect_mouse_events(
        &self,
        set_activity_duration_callback: Arc<dyn Fn(ActivityId, bool)>,
        shift_held: Rc<RefCell<bool>>,
        left_mouse_button_pressed: Rc<RefCell<bool>>,
    ) {
        fetch_from!(self, schedule_scrolled_window);

        schedule_scrolled_window.connect_motion_notify_event(
            clone!(@strong left_mouse_button_pressed,
                   @strong self as this
                   => move |scrolled_window, event| {
               this.maybe_update_activity_under_cursor(scrolled_window,
                                                 event.get_position(),
                                                 &left_mouse_button_pressed);
                glib::signal::Inhibit(false)
            }),
        );

        schedule_scrolled_window.connect_scroll_event(clone!(@strong self as this,
               @strong left_mouse_button_pressed
                   => move |scrolled_window, event| {
           this.maybe_update_activity_under_cursor(scrolled_window,
                                             event.get_position(),
                                             &left_mouse_button_pressed);

           let scroll_captured = if !*shift_held.borrow() {
                // Shift is not held, pretend nothing happened
                false
            } else {
                // Check first if the mouse is on an activity
                if let Some(activity) = &*this.activity_under_cursor.borrow() {
                    // Check if the scroll direction is vertical
                    if let Some(increase) = increase_duration_on_scroll(event) {
                        // Vertical scroll
                        (set_activity_duration_callback)(activity.id(), increase);
                        true
                    } else {
                        // Horizontal scroll
                        false
                    }
                } else {
                    // We are not on an activity
                    false
                }
            };
            glib::signal::Inhibit(scroll_captured)
        }));
    }

    fn maybe_update_activity_under_cursor(
        &self,
        window: &gtk::ScrolledWindow,
        event_position: (f64, f64),
        left_mouse_button_pressed: &Rc<RefCell<bool>>,
    ) {
        let (x_event, y_event) = event_position;
        let (x_scrollbar_offset, y_scrollbar_offset) = (
            window.get_hadjustment().unwrap().get_value(),
            window.get_vadjustment().unwrap().get_value(),
        );
        let (x, y) = (x_event + x_scrollbar_offset, y_event + y_scrollbar_offset);

        let new_activity =
            get_activity_under_cursor(x as i32, y as i32, &self.schedules_to_show.lock().unwrap());

        let current_activity = self.activity_under_cursor.borrow();

        let update_activity_to_some = current_activity.is_none() && new_activity.is_some();
        let update_activity_to_none = current_activity.is_some()
            && new_activity.is_none()
            // Do not invalidate current activity while the mouse is held - otherwise some
            // drag & drop operations are hard to do
            && !*left_mouse_button_pressed.borrow();

        let update_activity = update_activity_to_some || update_activity_to_none;
        if update_activity {
            drop(current_activity);
            self.set_activity_under_cursor(new_activity);
        }
    }

    /// Sets the activity under cursor and enables/disable drag if the activity_under_cursor is
    /// some.
    fn set_activity_under_cursor(&self, activity: Option<ActivityToDisplay>) {
        match activity {
            Some(_) => self.enable_drag_from_schedules_drawing(),
            None => self.disable_drag_from_schedules_drawing(),
        };
        *self.activity_under_cursor.borrow_mut() = activity;
    }
}
