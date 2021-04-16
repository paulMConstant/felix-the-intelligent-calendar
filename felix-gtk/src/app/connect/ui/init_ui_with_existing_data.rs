use crate::app::App;

impl App {
    pub(in super::super::super) fn init_ui_with_existing_data(&self) {
        self.emit_existing_activities();
        self.emit_existing_entities();
        self.emit_existing_groups();
        self.emit_existing_work_hours();
    }

    fn emit_existing_activities(&self) {
        let data = self.data.borrow();
        let activities = data.activities_not_sorted();
        if !activities.is_empty() {
            // Refresh the view
            data.events()
                .borrow_mut()
                .emit_activity_added(&data, &activities[0]);
        }
    }

    fn emit_existing_entities(&self) {
        let data = self.data.borrow();
        let entities = data.entities_sorted();
        if !entities.is_empty() {
            // Refresh the view
            data.events()
                .borrow_mut()
                .emit_entity_added(&data, entities[0]);
        }
    }

    fn emit_existing_groups(&self) {
        let data = self.data.borrow();
        let groups = data.groups_sorted();
        if !groups.is_empty() {
            // Refresh the view
            data.events()
                .borrow_mut()
                .emit_group_added(&data, groups[0]);
        }
    }

    fn emit_existing_work_hours(&self) {
        let data = self.data.borrow();
        // Refresh the view
        data.events().borrow_mut().emit_work_hours_changed(&data);
    }
}
