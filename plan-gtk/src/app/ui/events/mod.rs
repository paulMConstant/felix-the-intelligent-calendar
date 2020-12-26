use crate::app::ui::Ui;

mod activities;
mod entities;
mod groups;
mod work_hours;

impl Ui {
    pub(in super::super) fn init_ui_state(&mut self) {
        self.on_init_activities();
        self.on_init_entities();
        self.on_init_groups();
    }
}
