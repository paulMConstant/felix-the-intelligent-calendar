use crate::app::ui::Ui;
use gtk::prelude::*;

impl Ui {
    #[must_use]
    pub fn pdf_export_button(&self) -> gtk::Button {
        fetch_ui_from_builder!(self, "PdfExportButton")
    }

    #[must_use]
    pub fn export_popover(&self) -> gtk::Popover {
        fetch_ui_from_builder!(self, "ExportPopover")
    }
}
