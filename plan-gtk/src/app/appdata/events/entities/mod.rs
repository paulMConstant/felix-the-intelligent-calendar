pub mod update_ui_state;

use crate::app::appdata::AppData;
use plan_backend::data::{clean_string, Entity};

use gtk::prelude::*;
use std::convert::TryFrom;

impl AppData {
    pub(super) fn init_entity_events(&mut self) {
        self.update_current_entity(&None);
    }

    pub fn event_add_entity(&mut self) {
        fetch_from!(self, add_entity_entry);
        let entity_name = add_entity_entry.get_text();
        add_entity_entry.set_text("");

        no_notify_assign_or_return!(entity_name, clean_string(entity_name));
        assign_or_return!(entity_name, self.data.add_entity(&entity_name));
        assign_or_return!(entity, self.data.entity(&entity_name));

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
    }

    pub fn event_entity_selected(&mut self) {
        fetch_from!(self, entities_tree_view);

        let selection = entities_tree_view.get_selection().get_selected();
        if selection.is_none() {
            return;
        }

        let (model, iter) = selection.expect("We treated the None case above");
        let value = model.get_value(&iter, 0);
        let selected_entity: &str = value
            .get()
            .expect("Value in list store should be gchararray")
            .expect("Value in list store should be gchararray");

        assign_or_return!(entity, self.data.entity(selected_entity));
        let entity = entity.clone();
        self.update_current_entity(&Some(entity));
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
            .expect("Entity existed because it was removed, therefore this is valid");
        let entities = self.data.entities_sorted();
        let len = entities.len();

        let (new_current_entity, position_of_new_current_entity) = if len == 0 {
            // No entities left
            (None::<Entity>, None::<i32>)
        } else {
            let position_of_new_current_entity = if len <= position_of_removed_entity {
                // The removed entity was the last. Show the previous entity.
                position_of_removed_entity - 1
            } else {
                // Show the next entity
                position_of_removed_entity
            };

            let new_current_entity = Some(entities[position_of_new_current_entity].clone());
            let position_of_next_entity = i32::try_from(position_of_new_current_entity)
                .expect("There should not be 2 billion entities, we should be safe");

            (new_current_entity, Some(position_of_next_entity))
        };

        self.update_current_entity(&new_current_entity);
        self.update_entities_treeview(position_of_new_current_entity);
    }

    pub fn event_rename_entity(&mut self) {
        fetch_from!(self, entity_name_entry);
        let entity_to_rename =
            self.state.current_entity.as_ref().expect(
                "Current entity should be selected before accessing any entity-related field",
            );
        let new_name = entity_name_entry.get_text();
        no_notify_assign_or_return!(new_name, clean_string(new_name));
        no_notify_assign_or_return!(
            new_name,
            self.data.set_entity_name(entity_to_rename, new_name)
        );
        assign_or_return!(new_entity, self.data.entity(new_name));

        let new_entity = new_entity.clone();
        self.update_current_entity(&Some(new_entity));
        self.update_entities_treeview(None);
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
