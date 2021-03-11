mod drawing;
mod drop;
mod fetch_activity_insertion_ui;
mod fetch_data_from_cursor_position;
mod schedules;

use crate::app::ui::{EntitiesAndInsertionTimes, EntityToShow};
use fetch_data_from_cursor_position::get_id_of_activity_under_cursor;
use schedules::Schedules;

use felix_backend::data::ActivityId;

use glib::clone;
use gtk::prelude::*;

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

const NUM_HOURS_IN_DAY: i32 = 24;

#[derive(Debug, Clone)]
struct MousePosition {
    pub x: i32,
    pub y: i32,
}

pub struct ActivityInsertionUi {
    builder: gtk::Builder,
    schedules_to_show: Arc<Mutex<Schedules>>,
    mouse_position: Rc<RefCell<Option<MousePosition>>>,
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
            mouse_position: Rc::new(RefCell::new(None)),
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
    pub(super) fn get_id_of_activity_under_cursor(&self) -> Option<ActivityId> {
        let cursor_position = (*self.mouse_position.borrow()).clone();
        cursor_position.and_then(|pos| {
            get_id_of_activity_under_cursor(pos.x, pos.y, &self.schedules_to_show.lock().unwrap())
        })
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
    ) {
        fetch_from!(self, schedule_scrolled_window);
        let mouse_position = &self.mouse_position;
        schedule_scrolled_window.connect_leave_notify_event(
            clone!(@strong mouse_position => move |_window, _event| {
                *mouse_position.borrow_mut() = None;
                glib::signal::Inhibit(false)
            }),
        );

        macro_rules! update_mouse_position {
            ($window: ident, $event: ident, $mouse_position: ident) => {
                let (x_event, y_event) = $event.get_position();

                let (x_scrollbar_offset, y_scrollbar_offset) = (
                    $window.get_hadjustment().unwrap().get_value(),
                    $window.get_vadjustment().unwrap().get_value(),
                );
                let (x, y) = (x_event + x_scrollbar_offset, y_event + y_scrollbar_offset);

                *$mouse_position.borrow_mut() = Some(MousePosition {
                    x: x as i32,
                    y: y as i32,
                });
            };
        }

        schedule_scrolled_window.connect_motion_notify_event(
            clone!(@strong mouse_position => move |scrolled_window, event| {
                update_mouse_position!(scrolled_window, event, mouse_position);
                glib::signal::Inhibit(false)
            }),
        );

        let schedules = &self.schedules_to_show;
        schedule_scrolled_window.connect_scroll_event(
            clone!(@strong mouse_position, @strong schedules
                       => move |scrolled_window, event| {
                update_mouse_position!(scrolled_window, event, mouse_position);
                // Shift is not held, pretend nothing happened
                if !*shift_held.borrow() {
                    return glib::signal::Inhibit(false);
                }

                let mouse_position = mouse_position.borrow();
                let mouse_position = mouse_position.as_ref().unwrap();

                let maybe_id = get_id_of_activity_under_cursor(
                    mouse_position.x, mouse_position.y, &schedules.lock().unwrap());
                // Unlock
                let inhibit =
                    // Check first if the mouse is on an activity
                    if let Some(id) = maybe_id {
                    let increase_duration = match event.get_direction() {
                        // Check if the scroll direction is vertical
                        gdk::ScrollDirection::Up => Some(true),
                        gdk::ScrollDirection::Down => Some(false),
                        gdk::ScrollDirection::Smooth => {
                            // Use x and y to deduce scroll direction
                            let (dx, dy) = event.get_delta();
                            if dx.abs() > dy.abs() {
                                // Horizontal scroll
                                None
                            } else {
                                Some(dy < 0.0)
                            }
                        },
                        // Horizontal scroll
                        _ => None,
                    };
                    if let Some(increase) = increase_duration {
                        // Vertical scroll
                        (set_activity_duration_callback)(id, increase);
                        true
                    } else {
                        // Horizontal scroll
                        false
                    }
                } else {
                    // We are not on an activity
                    false
                };

                glib::signal::Inhibit(inhibit)
            }),
        );
    }
}
