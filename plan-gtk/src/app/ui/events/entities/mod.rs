use crate::app::ui::helpers::get_next_element;
use crate::app::ui::Ui;

use plan_backend::data::{Data, Entity};

mod update;

impl Ui {
    pub(super) fn on_init_entities(&mut self) {
        self.update_current_entity(None);
    }

    pub fn on_entity_added(&mut self, data: &Data, entity: &Entity) {
        self.update_current_entity(Some(entity.clone()));
        self.update_entities_treeview(&data.entities_sorted());
    }

    pub fn on_entity_selected(&mut self, entity: Entity) {
        self.update_current_entity(Some(entity));
    }

    pub fn on_entity_removed(&mut self, data: &Data, position_of_removed_entity: usize) {
        let entities = &data.entities_sorted();
        let (new_current_entity, _) = get_next_element(position_of_removed_entity, entities);
        self.update_current_entity(new_current_entity);
        self.update_entities_treeview(entities);
    }

    pub fn on_entity_renamed(&mut self, data: &Data, entity: &Entity) {
        self.update_current_entity(Some(entity.clone()));
        self.update_entities_treeview(&data.entities_sorted());
    }
}
