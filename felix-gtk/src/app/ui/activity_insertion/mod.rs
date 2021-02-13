pub mod activity_insertion_ui;
pub mod entity_to_show;

use crate::app::ui::Ui;
use entity_to_show::EntityToShow;

use felix_backend::data::{Activity, ActivityID, Data, Entity, Time};

use std::collections::HashSet;
use std::sync::Arc;

use gtk::prelude::*;

impl Ui {
    pub(super) fn on_init_activity_insertion(&self) {
        fetch_from!(self, insertion_box);
        insertion_box.pack_end(
            &self.activity_insertion.lock().unwrap().get_insertion_box(),
            true,
            true,
            0,
        );
    }

    pub fn set_activity_get_possible_insertions_callback(
        &mut self,
        callback: Arc<dyn Fn(ActivityID) -> (Option<HashSet<Time>>, Vec<String>)>,
    ) {
        self.get_possible_insertions_callback = callback;
    }

    pub fn set_activity_try_insert_callback(
        &mut self,
        callback: Arc<dyn Fn(String, ActivityID, Time)>,
    ) {
        self.activity_insertion
            .lock()
            .unwrap()
            .set_activity_try_insert_callback(callback);
    }

    pub fn on_show_entity_schedule(&mut self, entity_to_show: EntityToShow) {
        self.activity_insertion
            .lock()
            .unwrap()
            .show_entities_schedule(vec![entity_to_show]);
    }

    pub fn on_activities_changed_update_schedules(&mut self, data: &Data, _: &Activity) {
        self.update_schedules(data);
    }

    pub fn on_activity_inserted_update_schedules(&mut self, data: &Data, _: &Activity) {
        self.update_schedules(data);
    }

    pub fn on_work_hours_changed_update_schedules(&mut self, data: &Data) {
        self.update_schedules(data);
    }

    fn update_schedules(&mut self, data: &Data) {
        let activity_insertion = self.activity_insertion.lock().unwrap();
        let entities_to_show: Vec<_> = activity_insertion
            .shown_entities()
            .iter()
            .map(|entity_name| EntityToShow::new(entity_name.clone(), data))
            .collect();

        activity_insertion.show_entities_schedule(entities_to_show);
    }

    pub fn on_entity_renamed_update_schedules(
        &mut self,
        data: &Data,
        entity: &Entity,
        old_name: &String,
    ) {
        let activity_insertion = self.activity_insertion.lock().unwrap();
        if activity_insertion.shown_entities().contains(old_name) {
            activity_insertion.remove_entity_schedule(old_name);
            let new_entity = EntityToShow::new(entity.name(), data);
            activity_insertion.show_entities_schedule(vec![new_entity]);
        }
    }

    pub fn on_entity_removed_update_schedules(
        &mut self,
        _data: &Data,
        _position: usize,
        old_name: &String,
    ) {
        self.activity_insertion
            .lock()
            .unwrap()
            .remove_entity_schedule(old_name);
    }
}
