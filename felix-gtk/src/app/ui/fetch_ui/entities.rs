use gtk::prelude::*;

use crate::app::ui::Ui;

impl Ui {
    #[must_use]
    pub fn entities_tree_view(&self) -> gtk::TreeView {
        fetch_ui_from_builder!(self, "EntitiesTreeView")
    }

    #[must_use]
    pub fn entities_list_store(&self) -> gtk::ListStore {
        fetch_ui_from_builder!(self, "EntitiesListStore")
    }

    #[must_use]
    pub fn entity_name_entry(&self) -> gtk::Entry {
        fetch_ui_from_builder!(self, "EntityNameEntry")
    }

    #[must_use]
    pub fn entity_add_button(&self) -> gtk::Button {
        fetch_ui_from_builder!(self, "AddEntityButton")
    }

    #[must_use]
    pub fn entity_add_entry(&self) -> gtk::Entry {
        fetch_ui_from_builder!(self, "AddEntityEntry")
    }

    #[must_use]
    pub fn entity_remove_button(&self) -> gtk::Button {
        fetch_ui_from_builder!(self, "EntityRemoveButton")
    }

    #[must_use]
    pub fn entity_specific_box(&self) -> gtk::Box {
        fetch_ui_from_builder!(self, "EntitySpecificBox")
    }

    #[must_use]
    pub fn custom_work_hours_scrolled_window(&self) -> gtk::ScrolledWindow {
        fetch_ui_from_builder!(self, "CustomWorkHoursScrolledWindow")
    }

    #[must_use]
    pub fn custom_work_hours_add_button(&self) -> gtk::Button {
        fetch_ui_from_builder!(self, "CustomWorkHoursAddButton")
    }
}
