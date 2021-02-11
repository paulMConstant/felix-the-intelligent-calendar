use super::{schedules::TimeTooltipToDraw, ActivityInsertionUi, Schedules};

use crate::app::ui::drag_config::*;

use felix_backend::data::{ActivityID, Time};

use glib::clone;
use gtk::prelude::*;

use byteorder::ByteOrder;

impl ActivityInsertionUi {
    pub(super) fn enable_drop(&self) {
        self.drag_dest_set();
        self.connect_drag_motion();
        self.connect_drag_data_received();
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
        let schedules = &self.schedules_to_show;

        schedules_drawing.connect_drag_motion(
            clone!(@strong schedules, @strong schedules_drawing => move |_drawing_area, _drag_context, x, y, _timestamp| {
                let mut schedules = schedules.lock().unwrap();
                let time = get_time_on_y(y, &schedules);
                let tooltip = TimeTooltipToDraw { x_cursor: x as f64, y_cursor: y as f64, time };
                schedules.time_tooltip_to_draw = Some(tooltip);
                schedules_drawing.queue_draw();
                glib::signal::Inhibit(false)
        }));
    }

    fn connect_drag_data_received(&self) {
        fetch_from!(self, schedules_drawing);
        let schedules = &self.schedules_to_show;

        schedules_drawing.connect_drag_data_received(
            clone!(@strong schedules, @strong schedules_drawing => move |_drawing_area,
                   _drag_context, x, y, selection_data, _info, _timestamp| {
            if selection_data.get_data_type().name() != DRAG_TYPE {
                return;
            }
            let schedules = schedules.lock().unwrap();
            if let Some(entity_name) = get_name_of_entity_from_x(x, &schedules) {
                let activity_id: ActivityID = byteorder::NativeEndian::read_u32(
                    &selection_data.get_data()) as ActivityID;
                let insertion_time = get_time_on_y(y, &schedules);
                (schedules.try_insert_activity_callback)(entity_name, activity_id, insertion_time);
            }
        }));
    }
}

#[must_use]
fn get_name_of_entity_from_x(x: i32, schedules: &Schedules) -> Option<String> {
    let index_of_entity = (x / schedules.width_per_schedule as i32) as usize;

    if index_of_entity < schedules.entities_to_show.len() {
        Some(schedules.entities_to_show[index_of_entity].name().clone())
    } else {
        None
    }
}

#[must_use]
fn get_time_on_y(y: i32, schedules: &Schedules) -> Time {
    let n_times_min_discretization = (y as f64 / schedules.height_per_min_discretization) as i32;
    Time::from_n_times_min_discretization(n_times_min_discretization)
}
