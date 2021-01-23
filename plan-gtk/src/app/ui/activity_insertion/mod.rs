pub mod activity_insertion_ui;
pub mod entity_to_show;

use crate::app::ui::Ui;
use entity_to_show::EntityToShow;

use plan_backend::data::Data;

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
        let entities_to_show: Vec<_> = self
            .activity_insertion
            .shown_entities()
            .iter()
            .map(|entity_name| EntityToShow::new(entity_name.clone(), data))
            .collect();

        self.activity_insertion
            .show_entities_schedule(entities_to_show);
    }
}
