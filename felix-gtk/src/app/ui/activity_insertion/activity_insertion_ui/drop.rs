use super::{ActivityInsertionUi, Schedules};

use crate::app::ui::drag_config::*;

use felix_backend::data::ActivityID;

use glib::clone;
use gtk::prelude::*;

use std::sync::{Arc, Mutex};

use byteorder::ByteOrder;

impl ActivityInsertionUi {
    pub(super) fn enable_drop(&self) {
        self.drag_dest_set();
        self.connect_drag_data_received();
        self.connect_drag_motion();
    }

    fn drag_dest_set(&self) {
        fetch_from!(self, schedules_drawing);
        let targets = vec![gtk::TargetEntry::new(
            DRAG_TYPE,
            gtk::TargetFlags::SAME_APP,
            0,
        )];
        schedules_drawing.drag_dest_set(gtk::DestDefaults::ALL, &targets, gdk::DragAction::COPY);
    }

    fn connect_drag_motion(&self) {
        fetch_from!(self, schedules_drawing);
        schedules_drawing.connect_drag_motion(
            clone!(@strong schedules_drawing => move |_drawing_area, _drag_context, _x, _y, _timestamp| {
            glib::signal::Inhibit(false)
        }));
    }

    fn connect_drag_data_received(&self) {
        fetch_from!(self, schedules_drawing);
        let schedules = &self.schedules_to_show;

        schedules_drawing.connect_drag_data_received(
            clone!(@strong schedules => move |_drawing_area, _drag_context, x, _y, selection_data, _info, _timestamp| {
            if selection_data.get_data_type().name() != DRAG_TYPE {
                return;
            }
            if let Some(entity_name) = get_name_of_entity_dropped_on(x, schedules.clone()) {
                let activity_id: ActivityID = byteorder::NativeEndian::read_u32(&selection_data.get_data());
                // TODO find time
                println!("Insert activity ID {} for entity {}", activity_id, entity_name);
            }
        }));
    }
}

fn get_name_of_entity_dropped_on(x: i32, schedules: Arc<Mutex<Schedules>>) -> Option<String> {
    let schedules = schedules.lock().unwrap();
    let index_of_entity: usize = (x / schedules.width_per_schedule as i32) as usize;

    if index_of_entity < schedules.entities_to_show.len() {
        Some(schedules.entities_to_show[index_of_entity].name().clone())
    } else {
        None
    }
}
