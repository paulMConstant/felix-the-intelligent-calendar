mod activity_insertion_ui;

pub use activity_insertion_ui::ActivityInsertionUi;

use gtk::prelude::*;

use crate::app::ui::Ui;

impl Ui {
    pub(super) fn on_init_activity_insertion(&self) {
        fetch_from!(self, insertion_box);
        insertion_box.pack_end(&self.activity_insertion.get_insertion_box(), true, true, 0);
    }

    pub fn on_show_entity_schedule(&mut self, entity_to_show: String) {
        self.activity_insertion.show_entity_schedule(entity_to_show);
    }
}
