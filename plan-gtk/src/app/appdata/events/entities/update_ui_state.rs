use crate::app::appdata::AppData;
use plan_backend::data::Entity;

use gtk::prelude::*;

impl AppData {
    /// Updates the state of AppData and Entity-specific UI.
    pub(super) fn update_current_entity(&mut self, entity: &Option<Entity>) {
        match entity {
            Some(entity) => {
                self.update_current_entity_view(entity);
                self.state.current_entity = Some(entity.name());
            }
            None => {
                self.hide_current_entity_view();
                self.state.current_entity = None;
            }
        };
    }

    /// Updates the treeview of entities and selects the given row if not None.
    pub(super) fn update_entities_treeview(&self, selection_row: Option<i32>) {
        self.update_entities_list_store();
        if let Some(row) = selection_row {
            self.update_treeview_selection(row);
        }
    }

    fn update_entities_list_store(&self) {
        fetch_from!(self, entities_list_store);

        entities_list_store.clear();
        for entity_name in self
            .data
            .entities_sorted()
            .into_iter()
            .map(|entity| entity.name())
        {
            entities_list_store.insert_with_values(None, &[0], &[&entity_name]);
        }
    }

    /// Selects the row with given index.
    fn update_treeview_selection(&self, selected_row: i32) {
        // Select the corresponding entity in tree view
        fetch_from!(self, entities_tree_view);
        let tree_path = gtk::TreePath::from_indicesv(&[selected_row]);
        let focus_column = None::<&gtk::TreeViewColumn>;
        with_blocked_signals!(
            self,
            entities_tree_view.set_cursor(&tree_path, focus_column, false),
            entities_tree_view
        );
    }

    fn update_current_entity_view(&mut self, entity: &Entity) {
        fetch_from!(
            self,
            entity_specific_box,
            entity_name_entry,
            entity_send_mail_switch,
            entity_mail_entry,
            entity_custom_work_hours_switch
        );

        entity_specific_box.show();

        with_blocked_signals!(
            self,
            {
                entity_name_entry.set_text(&entity.name());
                entity_mail_entry.set_text(&entity.mail());
                entity_custom_work_hours_switch
                    .set_active(entity.custom_work_hours().is_empty() == false);
                entity_send_mail_switch.set_active(entity.send_me_a_mail());
            },
            entity_name_entry,
            entity_mail_entry,
            entity_custom_work_hours_switch,
            entity_send_mail_switch
        );
    }

    fn hide_current_entity_view(&mut self) {
        fetch_from!(self, entity_specific_box);
        entity_specific_box.hide();
    }
}
