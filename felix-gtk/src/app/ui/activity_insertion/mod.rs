pub mod activity_insertion_ui;
pub mod entity_to_show;

use crate::app::ui::Ui;
use entity_to_show::EntityToShow;

use felix_backend::data::{Data, Entity};

use gtk::prelude::*;

impl Ui {
    pub(super) fn on_init_activity_insertion(&self) {
        fetch_from!(self, insertion_box);
        insertion_box.pack_end(&self.activity_insertion.get_insertion_box(), true, true, 0);
    }

    pub fn on_show_entity_schedule(&mut self, entity_to_show: EntityToShow) {
        self.activity_insertion
            .show_entities_schedule(vec![entity_to_show]);
    }

    pub fn on_work_hours_changed_update_schedules(&mut self, data: &Data) {
        // Update data
        let entities_to_show: Vec<_> = self
            .activity_insertion
            .shown_entities()
            .iter()
            .map(|entity_name| EntityToShow::new(entity_name.clone(), data))
            .collect();

        self.activity_insertion
            .show_entities_schedule(entities_to_show);
    }

    pub fn on_entity_renamed_update_schedules(
        &mut self,
        data: &Data,
        entity: &Entity,
        old_name: &String,
    ) {
        if self.activity_insertion.shown_entities().contains(old_name) {
            self.activity_insertion.remove_entity_schedule(old_name);
            let new_entity = EntityToShow::new(entity.name(), data);
            self.activity_insertion
                .show_entities_schedule(vec![new_entity]);
        }
    }

    pub fn on_entity_removed_update_schedules(
        &mut self,
        _data: &Data,
        _position: usize,
        old_name: &String,
    ) {
        self.activity_insertion.remove_entity_schedule(old_name);
    }
}
