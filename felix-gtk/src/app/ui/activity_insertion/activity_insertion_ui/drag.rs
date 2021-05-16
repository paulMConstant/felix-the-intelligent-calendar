use felix_backend::data::ActivityId;

use super::ActivityInsertionUi;
use crate::app::ui::{drag_config::*, EntitiesAndInsertionTimes};

use gdk::prelude::GdkContextExt;
use glib::clone;
use gtk::prelude::*;

use byteorder::ByteOrder;
use std::rc::Rc;

impl ActivityInsertionUi {
    pub(in super::super::super) fn setup_drag_from_schedules_drawing(
        &mut self,
        possible_insertions_callback: Rc<dyn Fn(ActivityId) -> EntitiesAndInsertionTimes>,
        remove_activity_from_schedule_callback: Rc<dyn Fn(ActivityId)>,
    ) {
        self.possible_insertions_callback = possible_insertions_callback;
        self.remove_activity_from_schedule_callback = remove_activity_from_schedule_callback;
    }

    // Public so that connect module can access it
    pub fn enable_drag_from_schedules_drawing(&self) {
        fetch_from!(self, schedules_drawing);
        let targets = vec![gtk::TargetEntry::new(
            DRAG_TYPE,
            gtk::TargetFlags::SAME_APP,
            0,
        )];
        schedules_drawing.drag_source_set(
            gdk::ModifierType::MODIFIER_MASK,
            &targets,
            gdk::DragAction::COPY,
        );
        self.connect_drag_begin(
            self.possible_insertions_callback.clone(),
            self.remove_activity_from_schedule_callback.clone(),
        );
        self.connect_drag_data_get();
        self.connect_drag_end();
    }

    pub(super) fn disable_drag_from_schedules_drawing(&self) {
        fetch_from!(self, schedules_drawing);

        schedules_drawing.drag_source_unset();
    }

    fn connect_drag_begin(
        &self,
        get_possible_insertions_callback: Rc<dyn Fn(ActivityId) -> EntitiesAndInsertionTimes>,
        remove_activity_from_schedule_callback: Rc<dyn Fn(ActivityId)>,
    ) {
        fetch_from!(self, schedules_drawing);

        schedules_drawing.connect_drag_begin(
            clone!(@strong self as this => move |treeview, _drag_context| {
            let activity_under_cursor = this.get_activity_under_cursor()
                .expect("Dragging without an activity under cursor !");

            // 0. Remove the activity from the schedule
            (remove_activity_from_schedule_callback)(activity_under_cursor.id());

            // 1. Initialize drag item
            // Create pixbuf
            let color = gdk_pixbuf::Colorspace::Rgb;
            let pixbuf = gdk_pixbuf::Pixbuf::new(color, false, 8, DRAG_WIDTH, DRAG_HEIGHT)
                .expect("Not enough memory to create pixbuf");

            // Fill pixbuf with cairo
            let surface =
                cairo::ImageSurface::create(cairo::Format::ARgb32, DRAG_WIDTH, DRAG_HEIGHT)
                    .expect("Could not create surface");
            let context = cairo::Context::new(&surface);
            context.set_source_pixbuf(&pixbuf, DRAG_WIDTH as f64, DRAG_HEIGHT as f64);
            context.set_source_rgb(
                DRAG_BACKGROUND_RGB,
                DRAG_BACKGROUND_RGB,
                DRAG_BACKGROUND_RGB,
            );
            context.paint();

            // Get the name of the activity
            let selected_activity_name = activity_under_cursor.name();

            // Draw activity name with cairo
            // Center the text
            let size_of_text = context.text_extents(&selected_activity_name).width;
            let x_offset = (DRAG_WIDTH as f64 - size_of_text) / 2.0;
            context.move_to(x_offset, DRAG_TEXT_Y_OFFSET);
            context.set_font_size(DRAG_FONT_SIZE as f64);
            context.set_source_rgb(DRAG_FONT_RGB, DRAG_FONT_RGB, DRAG_FONT_RGB);
            context.show_text(&selected_activity_name);

            // Assign pixbuf to drag
            let surface = context.get_target();
            let pixbuf = gdk::pixbuf_get_from_surface(&surface, 0, 0, DRAG_WIDTH, DRAG_HEIGHT)
                .expect("Could not get pixbuf from surface");

            treeview.drag_source_set_icon_pixbuf(&pixbuf);

            // 2. Draw possible activity beginnings
            let selected_activity_id = activity_under_cursor.id();

            let concerned_entities_and_possible_insertion_times =
                (get_possible_insertions_callback)(selected_activity_id);
            this.show_possible_activity_insertions(concerned_entities_and_possible_insertion_times);
        }));
    }

    fn connect_drag_data_get(&self) {
        fetch_from!(self, schedules_drawing);

        let this = self.clone();
        schedules_drawing.connect_drag_data_get(
            move |_drawing, _drag_context, selection_data, _info, _timestamp| {
                // Fetch the selected activity ID and send it.
                let selected_activity_id = this
                    .get_activity_under_cursor()
                    .expect("Dragging without an activity under cursor !")
                    .id();

                let buffer: &mut [u8; DRAG_DATA_FORMAT] = &mut [0; DRAG_DATA_FORMAT];
                byteorder::NativeEndian::write_u32(
                    &mut buffer[0..DRAG_DATA_FORMAT],
                    selected_activity_id as u32,
                );
                selection_data.set(
                    &gdk::Atom::intern(DRAG_TYPE),
                    DRAG_DATA_FORMAT as i32,
                    buffer,
                );
            },
        );
    }

    fn connect_drag_end(&self) {
        fetch_from!(self, schedules_drawing);

        fn clear_possible_insertions(activity_insertion: &ActivityInsertionUi) {
            activity_insertion.show_possible_activity_insertions(EntitiesAndInsertionTimes {
                entities: Vec::new(),
                insertion_times: None,
            });
        }

        schedules_drawing.connect_drag_end(clone!(@strong self as this =>
                                                  move |_drawing_area, _drag_context| {
            clear_possible_insertions(&this);
        }));

        schedules_drawing.connect_drag_failed(clone!(@strong self as this =>
            move |_drawing_area, _drag_context, _drag_result| {
            clear_possible_insertions(&this);
            gtk::Inhibit(false)
        }));
    }
}
