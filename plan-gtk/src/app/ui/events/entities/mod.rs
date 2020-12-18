use crate::app::ui::Ui;

use plan_backend::data::Entity;

mod update;

impl Ui {
    pub(super) fn on_init_entities(&mut self) {
        self.update_current_entity(None);
    }

    pub fn on_entity_added(&mut self, entity: Entity, position_of_entity: i32) {
        self.update_current_entity(Some(entity));
        //self.update_entities_treeview(Some(position_of_new_entity));
        //self.update_activities_completion_list_store();
    }

    pub fn on_entity_selected(&mut self, entity: Entity) {
        self.update_current_entity(Some(entity));
    }

    pub fn on_entity_removed(&mut self, new_current_entity: Entity) {
        //let position_of_removed_entity = self
        //.data
        //.entities_sorted()
        //.iter()
        //.position(|other_entity| other_entity.name() == *entity_to_remove);
        //return_if_err!(self.data.remove_entity(entity_to_remove));

        //let position_of_removed_entity = position_of_removed_entity
        //.expect("Entity existed because it was removed, therefore it had a position");

        //let (new_current_entity, position_of_new_current_entity) =
        //get_next_element(position_of_removed_entity, self.data.entities_sorted());
        self.update_current_entity(Some(new_current_entity));
        //self.update_entities_treeview(position_of_new_current_entity);
        //self.update_current_group_members();
        //self.update_current_activity_entities();
        //self.update_activities_completion_list_store();
    }

    pub fn on_entity_renamed(&self, old_name: &String, new_name: &String) {
        //self.update_current_entity_without_ui(&Some(new_name);
        //self.update_entities_treeview(None);
        //self.update_current_group_members();
        //self.update_current_activity_entities();
        //self.update_activities_completion_list_store();
    }
}
