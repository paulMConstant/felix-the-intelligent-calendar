mod work_hours_builder;

use crate::app::ui::Ui;
pub use work_hours_builder::WorkHoursBuilder;

use felix_backend::{Data, TimeInterval};

impl Ui {
    pub fn on_add_work_hour(&self, current_work_hours: Vec<TimeInterval>) {
        self.work_hours_builder.on_add_work_hour(current_work_hours);
    }

    pub fn on_work_hours_changed(&self, data: &Data) {
        self.work_hours_builder
            .on_work_hours_changed(data.work_hours());
    }
}
