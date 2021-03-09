pub mod activity_insertion_ui;
pub mod entity_to_show;

use crate::app::ui::{EntitiesAndInsertionTimes, Ui};
use entity_to_show::EntityToShow;

use felix_backend::data::{Activity, ActivityId, Data, Entity, Time};

use std::sync::Arc;

use gtk::prelude::*;

impl Ui {
    pub(super) fn on_init_activity_insertion(&self) {
        fetch_from!(self, insertion_box);
        // Put the activity insertion widget inside of the box
        insertion_box.pack_end(
            &self
                .activity_insertion
                .lock()
                .unwrap()
                .get_activity_insertion_box(),
            true,
            true,
            0,
        );
    }

    pub fn set_activity_get_possible_insertions_callback(
        &mut self,
        callback: Arc<dyn Fn(ActivityId) -> EntitiesAndInsertionTimes>,
    ) {
        self.get_possible_insertions_callback = callback;
    }

    pub fn set_activity_try_insert_callback(
        &mut self,
        callback: Arc<dyn Fn(String, ActivityId, Time)>,
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

    pub fn on_activity_removed_update_schedules(&mut self, data: &Data, _position: usize) {
        self.update_schedules(data);
    }

    pub fn on_activity_changed_update_schedules(&mut self, data: &Data, _activity: &Activity) {
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
        old_name: &str,
    ) {
        let activity_insertion = self.activity_insertion.lock().unwrap();
        if activity_insertion
            .shown_entities()
            .contains(&old_name.into())
        {
            activity_insertion.remove_entity_schedule(old_name);
            let new_entity = EntityToShow::new(entity.name(), data);
            activity_insertion.show_entities_schedule(vec![new_entity]);
        }
    }

    pub fn on_entity_removed_update_schedules(
        &mut self,
        _data: &Data,
        _position: usize,
        old_name: &str,
    ) {
        self.activity_insertion
            .lock()
            .unwrap()
            .remove_entity_schedule(old_name);
    }

    pub fn on_left_click(&mut self, data: &Data) {
        let maybe_id = self
            .activity_insertion
            .lock()
            .unwrap()
            .get_id_of_activity_under_cursor();
        if let Some(id) = maybe_id {
            let activity = data
                .activity(id)
                .expect("User clicked on activity which does not exist");
            self.update_current_activity(&data.groups_sorted(), Some(activity));
        }
    }

    pub fn on_right_click(&mut self, data: &Data) {
        //println!("Got right click {:?}", self.activity_insertion
        //.lock()
        //.unwrap()
        //.get_id_of_activity_under_cursor());
        // TODO Lock activity in place
    }
}
