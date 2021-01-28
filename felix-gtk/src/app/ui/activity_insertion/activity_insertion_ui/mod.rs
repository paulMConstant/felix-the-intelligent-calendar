mod drawing;
mod drop;
mod fetch_activity_insertion_ui;
mod schedules;

use crate::app::ui::EntityToShow;
use schedules::Schedules;

use glib::clone;
use gtk::prelude::*;

use std::sync::{Arc, Mutex};

const NUM_HOURS_IN_DAY: i32 = 24;

pub struct ActivityInsertionUi {
    builder: gtk::Builder,
    schedules_to_show: Arc<Mutex<Schedules>>,
    // TODO maybe not pixbuf but window ?time_tooltip: gtk::Pixbuf,
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
        };

        activity_insertion.connect_draw();
        activity_insertion.connect_schedule_window_scroll();
        activity_insertion.enable_drop();

        activity_insertion
    }

    #[must_use]
    pub(super) fn get_insertion_box(&self) -> gtk::Box {
        fetch_from!(self, insertion_box);
        insertion_box
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

    pub(super) fn show_entities_schedule(&mut self, entities_to_show: Vec<EntityToShow>) {
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
        drop(schedules_to_show); // Borrow checker
        self.draw_schedules_sorted();
    }

    pub(super) fn remove_entity_schedule(&mut self, name_of_entity_to_remove: &String) {
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

    fn draw_schedules_sorted(&mut self) {
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
}
