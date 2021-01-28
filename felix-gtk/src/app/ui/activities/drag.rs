use crate::app::ui::helpers::tree::get_selection_from_treeview;
use crate::app::ui::{activities_treeview_config::*, drag_config::*, Ui};

use felix_backend::data::ActivityID;

use gdk::prelude::GdkContextExt;
use glib::clone;
use gtk::prelude::*;

use cairo;

use byteorder::ByteOrder;

impl Ui {
    pub(super) fn enable_drag_from_activities_treeview(&self) {
        self.drag_source_set();
        self.connect_drag_begin();
        self.connect_drag_data_get();
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

    fn connect_drag_begin(&self) {
        fetch_from!(self, activities_tree_view);
        activities_tree_view.connect_drag_begin(
            clone!(@strong activities_tree_view => move |treeview, drag_context| {
            // Create pixbuf
            let color = gdk_pixbuf::Colorspace::Rgb;
            let pixbuf = gdk_pixbuf::Pixbuf::new(color, false, 8, DRAG_WIDTH, DRAG_HEIGHT)
                .expect("Not enough memory");

            // Fill pixbuf with cairo
            let surface = cairo::ImageSurface::create(cairo::Format::ARgb32, DRAG_WIDTH, DRAG_HEIGHT)
                .expect("Could not create surface");
            let context = cairo::Context::new(&surface);
            context.set_source_pixbuf(&pixbuf, DRAG_WIDTH as f64, DRAG_HEIGHT as f64);
            context.set_source_rgb(DRAG_BACKGROUND_RGB, DRAG_BACKGROUND_RGB, DRAG_BACKGROUND_RGB);
            context.paint();

            // Draw activity name with cairo
            context.set_font_size(DRAG_FONT_SIZE as f64);
            context.set_source_rgb(DRAG_FONT_RGB, DRAG_FONT_RGB, DRAG_FONT_RGB);

            // Get the name of the activity
            let selected_activity_name = get_selection_from_treeview(&activities_tree_view,
                                                                     ACTIVITY_NAME_COLUMN)
                .expect("Dragging an activity when no activity is selected");

            let size_of_text = context.text_extents(&selected_activity_name).width;
            // Center the text
            let x_offset = (DRAG_WIDTH as f64 - size_of_text) / 2.0;
            context.move_to(x_offset, DRAG_TEXT_Y_OFFSET);
            context.show_text(&selected_activity_name);

            // Assign pixbuf to drag
            let surface = context.get_target();
            let pixbuf = gdk::pixbuf_get_from_surface(&surface, 0, 0, DRAG_WIDTH, DRAG_HEIGHT)
                .expect("Could not get pixbuf from surface");

            drag_context.set_hotspot(DRAG_WIDTH / 2, 0); // TODO does not work
            treeview.drag_source_set_icon_pixbuf(&pixbuf);

        }));
    }

    fn connect_drag_data_get(&self) {
        fetch_from!(self, activities_tree_view);
        activities_tree_view.connect_drag_data_get(
            clone!(@strong activities_tree_view => move |_treeview, _drag_context, selection_data, _info, _timestamp| {
            // Fetch the selected activity ID and send it.
            let selected_activity_id = get_selection_from_treeview(&activities_tree_view,
                                                                   ACTIVITY_ID_COLUMN)
                .expect("Dragging an activity when no activity is selected")
                .parse::<ActivityID>()
                .expect("Error when parsing activity ID from activities model");

            let buffer: &mut [u8; DRAG_DATA_FORMAT] = &mut [0; DRAG_DATA_FORMAT];
            byteorder::NativeEndian::write_u32(&mut buffer[0..DRAG_DATA_FORMAT], selected_activity_id);
            selection_data.set(&gdk::Atom::intern(DRAG_TYPE), DRAG_DATA_FORMAT as i32, buffer);
        }));
    }
}
