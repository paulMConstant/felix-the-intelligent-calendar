mod drawing;
mod fetch_activity_insertion_ui;

use gtk::prelude::*;

use std::sync::{Arc, Mutex};

pub struct ActivityInsertionUi {
    builder: gtk::Builder,
    schedules_to_show: Arc<Mutex<Vec<String>>>,
}

impl ActivityInsertionUi {
    #[must_use]
    pub fn new() -> ActivityInsertionUi {
        let builder = gtk::Builder::new();
        builder
            .add_from_resource("/com/github/paulmconstant/plan/ui/activity_insertion.ui")
            .expect("Could not load ui file: activity_insertion.ui");

        let activity_insertion = ActivityInsertionUi {
            builder,
            schedules_to_show: Arc::new(Mutex::new(Vec::new())),
        };

        activity_insertion.connect_draw();

        activity_insertion
    }

    #[must_use]
    pub(super) fn get_insertion_box(&self) -> gtk::Box {
        fetch_from!(self, insertion_box);
        insertion_box
    }

    pub(super) fn show_entity_schedule(&mut self, entity_to_show: String) {
        self.schedules_to_show.lock().unwrap().push(entity_to_show);

        fetch_from!(self, header_drawing, hours_drawing);
        for drawing in &[header_drawing, hours_drawing] {
            drawing.queue_draw();
        }
    }
}
