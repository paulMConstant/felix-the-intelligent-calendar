mod drawing;
mod drop;
mod fetch_activity_insertion_ui;
mod fetch_data_from_cursor_position;
mod schedules;

use crate::app::ui::EntityToShow;
use fetch_data_from_cursor_position::get_id_of_activity_under_cursor;
use schedules::Schedules;

use felix_backend::data::{ActivityID, Time};

use glib::clone;
use gtk::prelude::*;

use std::cell::RefCell;
use std::collections::HashSet;
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
    try_insert_activity_callback: Arc<dyn Fn(String, ActivityID, Time)>,
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
            try_insert_activity_callback: Arc::new(Box::new(|_, _, _| {
                panic!("Insert activity callback has not been initialized !")
            })),
            mouse_position: Rc::new(RefCell::new(None)),
        };

        activity_insertion.connect_draw();
        activity_insertion.connect_schedule_window_scroll();
        activity_insertion.connect_mouse_position_events();

        activity_insertion
    }

    pub fn set_activity_try_insert_callback(
        &mut self,
        callback: Arc<dyn Fn(String, ActivityID, Time)>,
    ) {
        self.try_insert_activity_callback = callback;
        // Enable drop only after the callback has been set
        self.enable_drop();
    }

    pub fn show_possible_activity_insertions(
        &self,
        possible_insertion_times: Option<HashSet<Time>>,
        concerned_entities: Vec<String>,
    ) {
        let mut schedules = self.schedules_to_show.lock().unwrap();
        schedules.possible_activity_insertion_times = possible_insertion_times;
        schedules.activity_insertion_concerned_entities = concerned_entities;
        drop(schedules);

        self.draw_schedules_sorted();
    }

    #[must_use]
    pub(super) fn get_activity_insertion_box(&self) -> gtk::Box {
        fetch_from!(self, activity_insertion_box);
        activity_insertion_box
    }

    #[must_use]
    pub(super) fn get_id_of_activity_under_cursor(&self) -> Option<ActivityID> {
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

    pub(super) fn remove_entity_schedule(&self, name_of_entity_to_remove: &String) {
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

    fn connect_mouse_position_events(&self) {
        fetch_from!(self, schedule_scrolled_window);
        let mouse_position = self.mouse_position.clone();
        schedule_scrolled_window.connect_leave_notify_event(move |_window, _event| {
            *mouse_position.borrow_mut() = None;
            glib::signal::Inhibit(false)
        });

        let mouse_position = self.mouse_position.clone();
        schedule_scrolled_window.connect_motion_notify_event(move |scrolled_window, event| {
            let (x_event, y_event) = event.get_position();
            let (x_scrollbar_offset, y_scrollbar_offset) = (
                scrolled_window.get_hadjustment().unwrap().get_value(),
                scrolled_window.get_vadjustment().unwrap().get_value(),
            );
            let (x, y) = (x_event + x_scrollbar_offset, y_event + y_scrollbar_offset);

            *mouse_position.borrow_mut() = Some(MousePosition {
                x: x as i32,
                y: y as i32,
            });
            glib::signal::Inhibit(false)
        });
    }
}
