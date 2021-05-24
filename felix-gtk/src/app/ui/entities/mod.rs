use crate::app::ui::helpers::collections::get_next_element;
use crate::app::ui::Ui;

use felix_data::{Data, Entity, TimeInterval};

mod update;

impl Ui {
    pub(super) fn on_init_entities(&mut self) {
        self.update_current_entity_without_ui(None);
        self.hide_current_entity_view();
    }

    pub fn on_entity_added(&mut self, data: &Data, entity: &Entity) {
        self.update_current_entity(Some(entity.clone()), data);
        self.update_entities_treeview(&data.entities_sorted());
    }

    pub fn on_entity_selected(&mut self, data: &Data, entity: Entity) {
        self.update_current_entity(Some(entity), data);
    }

    pub fn on_entity_removed(&mut self, data: &Data, position_of_removed_entity: usize) {
        let entities = data.entities_sorted();
        let (new_current_entity, _) = get_next_element(position_of_removed_entity, &entities);
        self.update_current_entity(new_current_entity.cloned(), data);
        self.update_entities_treeview(&entities);
    }

    pub fn on_entity_renamed(&mut self, data: &Data, entity: &Entity) {
        self.update_current_entity_without_ui(Some(entity.clone()));
        self.update_entities_treeview(&data.entities_sorted());
    }

    pub fn on_add_custom_work_hour(&self, current_work_hours: Vec<TimeInterval>) {
        self.custom_work_hours_builder
            .on_add_work_hour(current_work_hours);
    }

    pub fn on_custom_work_hours_changed(&mut self, data: &Data) {
        let current_entity_name = self
            .current_entity
            .as_ref()
            .expect("Current entity should be set before custom work hours change")
            .name();

        let new_current_entity = data
            .entity(current_entity_name)
            .expect("Current entity should exist when custom work hours change");

        self.update_current_entity(Some(new_current_entity), data);
    }
}
