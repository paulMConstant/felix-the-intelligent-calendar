use crate::app::ui::helpers::tree::get_selection_from_treeview;
use crate::app::ui::{
    activities_treeview_config::*, drag_config::*, EntitiesAndInsertionTimes, Ui,
};

use felix_backend::data::ActivityId;

use gdk::prelude::GdkContextExt;
use gtk::prelude::*;

use byteorder::ByteOrder;
use std::sync::Arc;

impl Ui {
    pub(in super::super) fn enable_drag_from_activities_treeview(
        &self,
        possible_insertions_callback: Arc<dyn Fn(ActivityId) -> EntitiesAndInsertionTimes>,
        remove_activity_from_schedule_callback: Arc<dyn Fn(ActivityId)>,
    ) {
        self.drag_source_set();
        self.connect_drag_begin(
            possible_insertions_callback,
            remove_activity_from_schedule_callback,
        );
        self.connect_drag_data_get();
        self.connect_drag_end();
    }

    fn drag_source_set(&self) {
        fetch_from!(self, activities_tree_view);
        let targets = vec![gtk::TargetEntry::new(
            DRAG_TYPE,
            gtk::TargetFlags::SAME_APP,
            0,
        )];
        activities_tree_view.drag_source_set(
            gdk::ModifierType::MODIFIER_MASK,
            &targets,
            gdk::DragAction::COPY,
        );
    }

    fn connect_drag_begin(
        &self,
        get_possible_insertions_callback: Arc<dyn Fn(ActivityId) -> EntitiesAndInsertionTimes>,
        remove_activity_from_schedule_callback: Arc<dyn Fn(ActivityId)>,
    ) {
        fetch_from!(self, activities_tree_view);
        let activity_insertion = self.activity_insertion.clone();

        activities_tree_view.connect_drag_begin(move |treeview, drag_context| {
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
            let selected_activity_name =
                get_selection_from_treeview(treeview, ACTIVITY_NAME_COLUMN)
                    .expect("Dragging an activity when no activity is selected");

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

            drag_context.set_hotspot(DRAG_WIDTH / 2, 0); // TODO does not work
            treeview.drag_source_set_icon_pixbuf(&pixbuf);

            // 2. Remove activity from schedule
            let selected_activity_id = get_selection_from_treeview(&treeview, ACTIVITY_ID_COLUMN)
                .expect("Dragging an activity when no activity is selected")
                .parse::<ActivityId>()
                .expect("Error when parsing activity ID from activities model");

            remove_activity_from_schedule_callback(selected_activity_id);

            // 3. Draw possible activity beginnings
            let concerned_entities_and_possible_insertion_times =
                get_possible_insertions_callback(selected_activity_id);
            activity_insertion
                .lock()
                .unwrap()
                .show_possible_activity_insertions(concerned_entities_and_possible_insertion_times);
        });
    }

    fn connect_drag_data_get(&self) {
        fetch_from!(self, activities_tree_view);
        activities_tree_view.connect_drag_data_get(
            move |treeview, _drag_context, selection_data, _info, _timestamp| {
                // Fetch the selected activity ID and send it.
                let selected_activity_id =
                    get_selection_from_treeview(treeview, ACTIVITY_ID_COLUMN)
                        .expect("Dragging an activity when no activity is selected")
                        .parse::<ActivityId>()
                        .expect("Error when parsing activity ID from activities model");

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
        fetch_from!(self, activities_tree_view);
        let activity_insertion = self.activity_insertion.clone();

        activities_tree_view.connect_drag_end(move |_drawing_area, _drag_context| {
            // Clear possible insertions
            activity_insertion
                .lock()
                .unwrap()
                .show_possible_activity_insertions(EntitiesAndInsertionTimes {
                    entities: Vec::new(),
                    insertion_times: None,
                });
        });
    }
}
