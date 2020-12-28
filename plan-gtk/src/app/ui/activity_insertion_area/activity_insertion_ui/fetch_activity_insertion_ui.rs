use gtk::prelude::*;

use super::ActivityInsertionArea;

impl ActivityInsertionArea {
    #[must_use]
    pub fn insertion_area_box(&self) -> gtk::Box {
        fetch_ui_from_builder!(self, "InsertionAreaBox")
    }

    #[must_use]
    pub fn hours_drawing(&self) -> gtk::DrawingArea {
        fetch_ui_from_builder!(self, "HoursDrawing")
    }

    #[must_use]
    pub fn schedules_drawing(&self) -> gtk::DrawingArea {
        fetch_ui_from_builder!(self, "SchedulesDrawing")
    }

    #[must_use]
    pub fn header_drawing(&self) -> gtk::DrawingArea {
        fetch_ui_from_builder!(self, "HeaderDrawing")
    }

    #[must_use]
    pub fn corner_drawing(&self) -> gtk::DrawingArea {
        fetch_ui_from_builder!(self, "CornerDrawing")
    }
}
