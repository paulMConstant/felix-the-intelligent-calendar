use crate::app::appdata::AppData;

use gtk::prelude::*;

impl AppData {
    #[must_use]
    pub fn group_add_button(&self) -> gtk::Button {
        fetch_ui_from_builder!(self, "AddGroupButton")
    }

    #[must_use]
    pub fn group_remove_button(&self) -> gtk::Button {
        fetch_ui_from_builder!(self, "RemoveGroupButton")
    }

    #[must_use]
    pub fn group_add_entry(&self) -> gtk::Entry {
        fetch_ui_from_builder!(self, "AddGroupEntry")
    }

    #[must_use]
    pub fn groups_list_store(&self) -> gtk::ListStore {
        fetch_ui_from_builder!(self, "GroupsListStore")
    }

    #[must_use]
    pub fn group_members_list_store(&self) -> gtk::ListStore {
        fetch_ui_from_builder!(self, "GroupMembersListStore")
    }

    #[must_use]
    pub fn group_specific_box(&self) -> gtk::Box {
        fetch_ui_from_builder!(self, "GroupSpecificBox")
    }

    #[must_use]
    pub fn groups_tree_view(&self) -> gtk::TreeView {
        fetch_ui_from_builder!(self, "GroupsTreeView")
    }

    #[must_use]
    pub fn group_name_entry(&self) -> gtk::Entry {
        fetch_ui_from_builder!(self, "GroupNameEntry")
    }

    #[must_use]
    pub fn entity_into_group_name_entry(&self) -> gtk::Entry {
        fetch_ui_from_builder!(self, "EntityIntoGroupNameEntry")
    }

    #[must_use]
    pub fn add_to_group_button(&self) -> gtk::Button {
        fetch_ui_from_builder!(self, "AddToGroupButton")
    }

    #[must_use]
    pub fn group_members_tree_view(&self) -> gtk::TreeView {
        fetch_ui_from_builder!(self, "GroupMembersTreeView")
    }

    #[must_use]
    pub fn entity_into_group_completion(&self) -> gtk::EntryCompletion {
        fetch_ui_from_builder!(self, "EntityIntoGroupCompletion")
    }

    #[must_use]
    pub fn create_entity_before_adding_to_group_switch(&self) -> gtk::Switch {
        fetch_ui_from_builder!(self, "CreateEntityBeforeAddingToGroupSwitch")
    }
}
