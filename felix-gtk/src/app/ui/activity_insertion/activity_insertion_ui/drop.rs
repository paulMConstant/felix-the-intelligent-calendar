use super::{
    fetch_data_from_cursor_position::{get_name_of_entity_from_x, get_time_on_y},
    schedules::TimeTooltipToDraw,
    ActivityInsertionUi,
};

use crate::app::ui::drag_config::*;

use felix_backend::data::ActivityId;

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
        let schedules = self.schedules_to_show.clone();

        schedules_drawing.connect_drag_motion(
            move |drawing_area, _drag_context, x, y, _timestamp| {
                let mut schedules = schedules.lock().unwrap();
                let time = get_time_on_y(y, &schedules);
                let tooltip = TimeTooltipToDraw {
                    x_cursor: x as f64,
                    y_cursor: y as f64,
                    time,
                };
                schedules.time_tooltip_to_draw = Some(tooltip);
                drawing_area.queue_draw();
                glib::signal::Inhibit(false)
            },
        );
    }

    fn connect_drag_data_received(&self) {
        fetch_from!(self, schedules_drawing);
        let schedules = self.schedules_to_show.clone();
        let try_insert_activity_callback = self.try_insert_activity_callback.clone();

        schedules_drawing.connect_drag_data_received(
            move |_drawing_area, _drag_context, x, y, selection_data, _info, _timestamp| {
                if selection_data.get_data_type().name() != DRAG_TYPE {
                    return;
                }
                let schedules = schedules.lock().unwrap();
                if let Some(entity_name) = get_name_of_entity_from_x(x, &schedules) {
                    let activity_id: ActivityId =
                        byteorder::NativeEndian::read_u32(&selection_data.get_data()) as ActivityId;
                    let insertion_time = get_time_on_y(y, &schedules);
                    drop(schedules); // Unlock
                    (try_insert_activity_callback)(entity_name, activity_id, insertion_time);
                }
            },
        );
    }
}
