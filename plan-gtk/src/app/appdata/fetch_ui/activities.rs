use gtk::prelude::*;

use crate::app::appdata::AppData;

impl AppData {
    #[must_use]
    pub fn activity_specific_pane(&self) -> gtk::Paned {
        fetch_ui_from_builder!(self, "ActivitySpecificPane")
    }

    #[must_use]
    pub fn activity_add_entry(&self) -> gtk::Entry {
        fetch_ui_from_builder!(self, "ActivityAddEntry")
    }

    #[must_use]
    pub fn activity_name_entry(&self) -> gtk::Entry {
        fetch_ui_from_builder!(self, "ActivityNameEntry")
    }

    #[must_use]
    pub fn activity_add_button(&self) -> gtk::Button {
        fetch_ui_from_builder!(self, "ActivityAddButton")
    }

    #[must_use]
    pub fn activity_remove_button(&self) -> gtk::Button {
        fetch_ui_from_builder!(self, "ActivityRemoveButton")
    }

    #[must_use]
    pub fn activity_add_to_entry(&self) -> gtk::Entry {
        fetch_ui_from_builder!(self, "ActivityAddToEntry")
    }

    #[must_use]
    pub fn activities_tree_view(&self) -> gtk::TreeView {
        fetch_ui_from_builder!(self, "ActivitiesTreeView")
    }

    #[must_use]
    pub fn activities_list_store(&self) -> gtk::ListStore {
        fetch_ui_from_builder!(self, "ActivitiesListStore")
    }
}
