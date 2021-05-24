mod costs_to_rgb;
mod drag;
mod drawing;
mod drop;
mod event_helpers;
mod fetch_activity_insertion_ui;
mod fetch_data_from_cursor_position;
mod schedules;

use crate::app::ui::{ActivityToShow, EntitiesAndInsertionTimes, EntityToShow};
use event_helpers::increase_duration_on_scroll;
use fetch_data_from_cursor_position::{
    get_activity_under_cursor, get_entity_to_remove_under_cursor,
};
use schedules::Schedules;

use felix_data::ActivityId;

use glib::clone;
use gtk::prelude::*;

use std::cell::RefCell;
use std::rc::Rc;

const NUM_HOURS_IN_DAY: i32 = 24;

#[derive(Clone)]
pub struct ActivityInsertionUi {
    builder: gtk::Builder,
    schedules_to_show: Rc<RefCell<Schedules>>,
    last_activity_under_cursor: Rc<RefCell<Option<ActivityToShow>>>,
    possible_insertions_callback: Rc<dyn Fn(ActivityId) -> EntitiesAndInsertionTimes>,
    remove_activity_from_schedule_callback: Rc<dyn Fn(ActivityId)>,
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
            schedules_to_show: Rc::new(RefCell::new(Schedules::new())),
            last_activity_under_cursor: Rc::new(RefCell::new(None)),
            possible_insertions_callback: Rc::new(Box::new(|_| {
                panic!("possible_insertions_callback was not initialized")
            })),
            remove_activity_from_schedule_callback: Rc::new(Box::new(|_| {
                panic!("remove_activity_from_schedule_callback was not initialized")
            })),
        };

        activity_insertion.enable_click_events();
        activity_insertion.connect_draw();
        activity_insertion.connect_schedule_window_scroll();

        activity_insertion
    }

    fn enable_click_events(&self) {
        fetch_from!(self, header_drawing, schedules_drawing);
        header_drawing.add_events(gdk::EventMask::BUTTON_PRESS_MASK);
        schedules_drawing.add_events(gdk::EventMask::BUTTON_PRESS_MASK);
    }

    pub fn show_possible_activity_insertions(
        &self,
        concerned_entities_and_possible_insertion_times: EntitiesAndInsertionTimes,
    ) {
        let mut schedules = self.schedules_to_show.borrow_mut();
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
    pub(super) fn get_activity_under_cursor(&self) -> Option<ActivityToShow> {
        self.last_activity_under_cursor.borrow().clone()
    }

    #[must_use]
    pub(super) fn shown_entities(&self) -> Vec<String> {
        self.schedules_to_show
            .borrow()
            .entities_to_show
            .iter()
            .map(|entity| entity.name().clone())
            .collect()
    }

    pub(super) fn show_entities_schedule(&self, entities_to_show: Vec<EntityToShow>) {
        let mut schedules_to_show = self.schedules_to_show.borrow_mut();
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
        drop(schedules_to_show); // Avoid multiple borrow crash
        self.draw_schedules_sorted();
    }

    pub(super) fn remove_entity_schedule(&self, name_of_entity_to_remove: &str) {
        let mut schedules_to_show = self.schedules_to_show.borrow_mut();
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
        let mut schedules_to_show = self.schedules_to_show.borrow_mut();

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

    pub(super) fn connect_scroll_event(
        &self,
        set_activity_duration_callback: Rc<dyn Fn(ActivityId, bool)>,
        shift_held: Rc<RefCell<bool>>,
    ) {
        fetch_from!(
            self,
            schedule_scrolled_window,
            hours_scrolled_window,
            header_scrolled_window
        );

        schedule_scrolled_window.connect_scroll_event(
            clone!(@strong self as this => move |_scrolled_window, event|
            {
                let (x, y) = event.get_position();
                let (x_scrollbar_offset, y_scrollbar_offset) = (
                    header_scrolled_window.get_hadjustment().unwrap().get_value(),
                    hours_scrolled_window.get_vadjustment().unwrap().get_value(),
                );

                this.update_activity_under_cursor(x + x_scrollbar_offset, y + y_scrollbar_offset);

                let scroll_captured = if !*shift_held.borrow() {
                    // Shift is not held, pretend nothing happened
                    false
                } else {
                    // Check first if the mouse is on an activity
                    if let Some(activity) = &*this.last_activity_under_cursor.borrow() {
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
            }),
        );
    }

    pub(super) fn update_activity_under_cursor(&self, x: f64, y: f64) {
        let new_activity =
            get_activity_under_cursor(x as i32, y as i32, &self.schedules_to_show.borrow());

        self.set_activity_under_cursor(new_activity);
    }

    /// Sets the activity under cursor and enables/disable drag if the activity_under_cursor is
    /// some.
    fn set_activity_under_cursor(&self, new_activity: Option<ActivityToShow>) {
        let mut current_activity = self.last_activity_under_cursor.borrow_mut();

        // Make sure we will not drag without an activity under cursor
        // Drag is enabled automatically on left click release or enter event
        if new_activity.is_none() {
            self.disable_drag_from_schedules_drawing();
        }

        *current_activity = new_activity;
    }

    /// Returns the name of the entity to delete if the cursor is over the button to remove its
    /// schedule.
    pub(super) fn get_entity_to_remove_under_cursor(&self, x: f64, y: f64) -> Option<String> {
        get_entity_to_remove_under_cursor(x, y, &self.schedules_to_show.borrow())
    }
}
