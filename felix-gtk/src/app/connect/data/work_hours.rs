use crate::app::App;

use glib::clone;

impl App {
    pub(in super::super) fn connect_work_hour_events(&self) {
        let events = self.data.borrow().events();
        let mut events = events.borrow_mut();

        events.connect_work_hours_changed(Box::new(clone!(@strong self.ui as ui => move |data| {
            let mut ui = ui.borrow_mut();
            ui.on_work_hours_changed(data);

            // No need to update custom work hours if no entity is selected
            if ui.current_entity().is_some() {
                ui.on_custom_work_hours_changed(data);
            }

            ui.update_schedules(data);
        })));
    }
}
