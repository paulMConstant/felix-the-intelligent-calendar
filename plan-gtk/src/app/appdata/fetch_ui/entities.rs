use gtk::prelude::*;

use crate::app::appdata::AppData;

impl AppData {
    #[must_use]
    pub fn entities_tree_view(&self) -> gtk::TreeView {
        self.builder
            .get_object("EntitiesTreeView")
            .expect("Could not load EntitiesTreeView from ui file.")
    }

    #[must_use]
    pub fn entities_list_store(&self) -> gtk::ListStore {
        self.builder
            .get_object("EntitiesListStore")
            .expect("Could not load EntitiesListStore from ui file.")
    }

    #[must_use]
    pub fn entity_name_entry(&self) -> gtk::Entry {
        self.builder
            .get_object("EntityNameEntry")
            .expect("Could not load EntityNameEntry from ui file.")
    }

    #[must_use]
    pub fn add_entity_button(&self) -> gtk::Button {
        self.builder
            .get_object("AddEntityButton")
            .expect("Could not load AddEntityButton from ui file.")
    }

    #[must_use]
    pub fn add_entity_entry(&self) -> gtk::Entry {
        self.builder
            .get_object("AddEntityEntry")
            .expect("Could not load AddEntityEntry from ui file.")
    }

    #[must_use]
    pub fn entity_remove_button(&self) -> gtk::Button {
        self.builder
            .get_object("EntityRemoveButton")
            .expect("Could not load EntityRemoveButton from ui file.")
    }

    #[must_use]
    pub fn entity_send_mail_switch(&self) -> gtk::Switch {
        self.builder
            .get_object("EntitySendMailSwitch")
            .expect("Could not load EntitySendMailSwitch from ui file.")
    }

    #[must_use]
    pub fn entity_mail_entry(&self) -> gtk::Entry {
        self.builder
            .get_object("EntityMailEntry")
            .expect("Could not load EntityMailEntry from ui file.")
    }

    #[must_use]
    pub fn entity_custom_work_hours_switch(&self) -> gtk::Switch {
        self.builder
            .get_object("EntityCustomWorkHoursSwitch")
            .expect("Could not load EntityCustomWorkHoursSwitch from ui file.")
    }

    #[must_use]
    pub fn entity_specific_box(&self) -> gtk::Box {
        self.builder
            .get_object("EntitySpecificBox")
            .expect("Could not load EntitySpecificBox from ui file.")
    }
}
