use crate::app::App;

use glib::clone;

impl App {
    pub(super) fn connect_entity_events(&self) {
        let events = self.data.lock().unwrap().events();
        let mut events = events.borrow_mut();

        events.connect_entity_added(Box::new(
            clone!(@strong self.ui as ui => move |data, entity| {
                let mut ui = ui.lock().unwrap();
                ui.on_entity_added(data, entity);
                ui.on_entities_or_groups_changed(data);
            }),
        ));

        events.connect_entity_renamed(Box::new(
            clone!(@strong self.ui as ui => move |data, entity, _old_name| {
                let mut ui = ui.lock().unwrap();
                ui.on_entity_renamed(data, entity);
                ui.on_group_members_changed(data);
                ui.on_entities_or_groups_changed(data);
            }),
        ));

        events.connect_entity_removed(Box::new(
            clone!(@strong self.ui as ui => move |data, position, name| {
                let mut ui = ui.lock().unwrap();
                ui.on_entity_removed(data, position);
                ui.on_group_members_changed(data);
                ui.on_entities_or_groups_changed(data);
                ui.on_entity_removed_update_schedules(name);
            }),
        ));
    }
}
