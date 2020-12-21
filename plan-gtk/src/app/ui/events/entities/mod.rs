use crate::app::ui::helpers::get_next_element;
use crate::app::ui::Ui;

use plan_backend::data::Entity;

mod update;

impl Ui {
    pub(super) fn on_init_entities(&mut self) {
        self.update_current_entity(None);
    }

    pub fn on_entity_added(&mut self, entity: &Entity, entities: &Vec<&Entity>) {
        self.update_current_entity(Some(entity.clone()));
        self.update_entities_treeview(&entities);
        self.update_activities_completion_list_store();
    }

    pub fn on_entity_selected(&mut self, entity: Entity) {
        self.update_current_entity(Some(entity));
    }

    pub fn on_entity_removed(
        &mut self,
        position_of_removed_entity: usize,
        entities: &Vec<&Entity>,
    ) {
        let (new_current_entity, position_of_new_current_entity) =
            get_next_element(position_of_removed_entity, entities);
        self.update_current_entity(new_current_entity);
        self.update_entities_treeview(entities);
        self.update_current_group_members();
        self.update_current_activity_entities();
        self.update_activities_completion_list_store();
    }

    pub fn on_entity_renamed(&mut self, entity: &Entity, entities: &Vec<&Entity>) {
        self.update_current_entity_name_only(Some(entity.clone()));
        self.update_entities_treeview(entities);
        self.update_current_group_members();
        self.update_current_activity_entities();
        self.update_activities_completion_list_store();
    }
}
