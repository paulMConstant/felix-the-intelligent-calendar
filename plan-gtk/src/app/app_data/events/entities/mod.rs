mod update_ui_state;

use super::helpers::{get_next_element, get_selection_from_treeview};
use crate::app::app_data::AppData;
use plan_backend::data::clean_string;
use plan_backend::errors::Result;

use gtk::prelude::*;
use std::convert::TryFrom;

impl AppData {
    pub(super) fn event_init_entities(&mut self) {
        self.update_current_entity(&None);
    }

    pub fn event_add_entity(&mut self) {
        fetch_from!(self, entity_add_entry);
        let entity_name = entity_add_entry.get_text();
        with_blocked_signals!(self, entity_add_entry.set_text(""), entity_add_entry);
        no_notify_assign_or_return!(entity_name, clean_string(entity_name));
        return_if_err!(self.add_entity(&entity_name));
    }

    pub(in super::super) fn add_entity(&mut self, entity_name: &String) -> Result<()> {
        let entity_name = self.data.add_entity(entity_name)?;
        let entity = self.data.entity(&entity_name)?;

        // Fetch where the entity was added.
        let position_of_new_entity = self
            .data
            .entities_sorted()
            .into_iter()
            .position(|entity| entity.name() == entity_name)
            .expect("The entity was added succesfully so this should be valid");

        let position_of_new_entity = i32::try_from(position_of_new_entity)
            .expect("There should not be 2 billion entities, we should be safe");

        let entity = entity.clone();
        self.update_current_entity(&Some(entity));
        self.update_entities_treeview(Some(position_of_new_entity));
        self.update_activities_completion_list_store();
        Ok(())
    }

    pub fn event_entity_selected(&mut self) {
        fetch_from!(self, entities_tree_view);
        let selected_entity = get_selection_from_treeview(entities_tree_view);
        if let Some(entity_name) = selected_entity {
            assign_or_return!(entity, self.data.entity(entity_name));
            let entity = entity.clone();
            self.update_current_entity(&Some(entity));
        }
    }

    pub fn event_remove_entity(&mut self) {
        let entity_to_remove =
            self.state.current_entity.as_ref().expect(
                "Current entity should be selected before accessing any entity-related filed",
            );

        let position_of_removed_entity = self
            .data
            .entities_sorted()
            .iter()
            .position(|other_entity| other_entity.name() == *entity_to_remove);
        return_if_err!(self.data.remove_entity(entity_to_remove));

        let position_of_removed_entity = position_of_removed_entity
            .expect("Entity existed because it was removed, therefore it had a position");

        let (new_current_entity, position_of_new_current_entity) =
            get_next_element(position_of_removed_entity, self.data.entities_sorted());
        self.update_current_entity(&new_current_entity);
        self.update_entities_treeview(position_of_new_current_entity);
        self.update_current_group_members();
        self.update_current_activity_entities();
        self.update_activities_completion_list_store();
    }

    pub fn event_rename_entity(&mut self) {
        fetch_from!(self, entity_name_entry);
        let entity_to_rename =
            self.state.current_entity.as_ref().expect(
                "Current entity should be selected before accessing any entity-related field",
            );
        let new_name = entity_name_entry.get_text();
        no_notify_assign_or_return!(
            new_name, // TODO 
            self.data.set_entity_name(entity_to_rename, new_name)
        );
        self.update_current_entity_without_ui(Some(new_name));
        self.update_entities_treeview(None);
        self.update_current_group_members();
        self.update_current_activity_entities();
        self.update_activities_completion_list_store();
    }

    pub fn on_entity_renamed(&mut self) {
        // TODO CQRS to avoid deadlock
        //self.update_current_entity_without_ui(&Some(new_name);
        //self.update_entities_treeview(None);
        //self.update_current_group_members();
        //self.update_current_activity_entities();
        //self.update_activities_completion_list_store();

    }

    pub fn event_set_entity_mail(&mut self) {
        fetch_from!(self, entity_mail_entry);

        let mail = entity_mail_entry.get_text();
        let entity =
            self.state.current_entity.as_ref().expect(
                "Current entity should be selected before accessing any entity-related field",
            );
        return_if_err!(self.data.set_entity_mail(entity, mail));
    }

    pub fn event_set_send_mail(&mut self) {
        fetch_from!(self, entity_send_mail_switch);

        let send = entity_send_mail_switch.get_active();
        let entity =
            self.state.current_entity.as_ref().expect(
                "Current entity should be selected before accessing any entity-related field",
            );
        return_if_err!(self.data.set_send_mail_to(entity, send));
    }
}
