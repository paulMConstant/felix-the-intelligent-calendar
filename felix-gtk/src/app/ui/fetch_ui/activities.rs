use gtk::prelude::*;

use crate::app::ui::Ui;

impl Ui {
    #[must_use]
    pub fn activity_specific_box(&self) -> gtk::Box {
        fetch_ui_from_builder!(self, "ActivitySpecificBox")
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

    #[must_use]
    pub fn activity_duration_hour_spin(&self) -> gtk::SpinButton {
        fetch_ui_from_builder!(self, "ActivityDurationHourSpin")
    }

    #[must_use]
    pub fn activity_duration_minute_spin(&self) -> gtk::SpinButton {
        fetch_ui_from_builder!(self, "ActivityDurationMinuteSpin")
    }

    #[must_use]
    pub fn activity_inserted_switch(&self) -> gtk::Switch {
        fetch_ui_from_builder!(self, "ActivityInsertedSwitch")
    }

    #[must_use]
    pub fn activity_beginning_hour_spin(&self) -> gtk::SpinButton {
        fetch_ui_from_builder!(self, "ActivityBeginningHourSpin")
    }

    #[must_use]
    pub fn activity_beginning_minute_spin(&self) -> gtk::SpinButton {
        fetch_ui_from_builder!(self, "ActivityBeginningMinuteSpin")
    }

    #[must_use]
    pub fn activity_end_hour_spin(&self) -> gtk::SpinButton {
        fetch_ui_from_builder!(self, "ActivityEndHourSpin")
    }

    #[must_use]
    pub fn activity_end_minute_spin(&self) -> gtk::SpinButton {
        fetch_ui_from_builder!(self, "ActivityEndMinuteSpin")
    }

    #[must_use]
    pub fn activity_color_button(&self) -> gtk::ColorButton {
        fetch_ui_from_builder!(self, "ActivityColorButton")
    }

    #[must_use]
    pub fn activity_insertion_time_box(&self) -> gtk::Box {
        fetch_ui_from_builder!(self, "ActivityInsertionTimeBox")
    }

    #[must_use]
    pub fn activity_add_to_button(&self) -> gtk::Button {
        fetch_ui_from_builder!(self, "ActivityAddToButton")
    }

    #[must_use]
    pub fn activity_groups_list_store(&self) -> gtk::ListStore {
        fetch_ui_from_builder!(self, "ActivityGroupsListStore")
    }

    #[must_use]
    pub fn activity_entities_list_store(&self) -> gtk::ListStore {
        fetch_ui_from_builder!(self, "ActivityEntitiesListStore")
    }

    #[must_use]
    pub fn activity_groups_tree_view(&self) -> gtk::TreeView {
        fetch_ui_from_builder!(self, "ActivityGroupsTreeView")
    }

    #[must_use]
    pub fn activity_entities_tree_view(&self) -> gtk::TreeView {
        fetch_ui_from_builder!(self, "ActivityEntitiesTreeView")
    }

    #[must_use]
    pub fn autoinsert_button(&self) -> gtk::Button {
        fetch_ui_from_builder!(self, "AutoInsertButton")
    }
}
