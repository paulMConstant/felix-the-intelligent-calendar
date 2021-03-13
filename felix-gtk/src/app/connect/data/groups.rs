use crate::app::App;

use glib::clone;

impl App {
    pub(super) fn connect_group_events(&self) {
        let events = self.data.lock().unwrap().events();
        let mut events = events.borrow_mut();

        events.connect_group_added(Box::new(
            clone!(@strong self.ui as ui => move |data, group| {
                let mut ui = ui.lock().unwrap();
                ui.on_group_added(data, group);
                ui.on_entities_or_groups_changed(data);
            }),
        ));

        events.connect_group_renamed(Box::new(
            clone!(@strong self.ui as ui => move |data, group| {
                let mut ui = ui.lock().unwrap();
                ui.on_group_renamed(data, group);
                ui.on_entities_or_groups_changed(data);
            }),
        ));

        events.connect_group_removed(Box::new(
            clone!(@strong self.ui as ui => move |data, position| {
                let mut ui = ui.lock().unwrap();
                ui.on_group_removed(data, position);
                ui.on_entities_or_groups_changed(data);
            }),
        ));

        events.connect_entity_added_to_group(Box::new(
            clone!(@strong self.ui as ui => move |data, _group| {
                let mut ui = ui.lock().unwrap();
                ui.on_group_members_changed(data);
                ui.on_group_members_changed_update_activity(data);
            }),
        ));

        events.connect_entity_removed_from_group(Box::new(
            clone!(@strong self.ui as ui => move |data, _group| {
                let mut ui = ui.lock().unwrap();
                ui.on_group_members_changed(data);
                ui.on_group_members_changed_update_activity(data);
            }),
        ));
    }
}
