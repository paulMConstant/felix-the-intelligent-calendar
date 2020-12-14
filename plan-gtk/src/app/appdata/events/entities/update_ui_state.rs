use crate::app::appdata::{events::helpers::tree_path_from_selection_index, AppData};
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
    /// If the given row is None, keeps the original row.
    pub(super) fn update_entities_treeview(&self, selection_row: Option<i32>) {
        self.update_entities_list_store();
        self.update_entities_treeview_selection(selection_row);
    }

    fn update_entities_list_store(&self) {
        fetch_from!(self, entities_list_store, entities_tree_view);

        with_blocked_signals!(
            self,
            {
                entities_list_store.clear();
                for entity_name in self
                    .data
                    .entities_sorted()
                    .into_iter()
                    .map(|entity| entity.name())
                {
                    entities_list_store.insert_with_values(None, &[0], &[&entity_name]);
                }
            },
            entities_tree_view
        );
    }

    /// Selects the row with given index.
    fn update_entities_treeview_selection(&self, selection_index: Option<i32>) {
        fetch_from!(self, entities_tree_view, entities_list_store);

        let selection_tree_path = tree_path_from_selection_index(
            selection_index,
            entities_list_store,
            self.state.current_entity.as_ref(),
        );
        let focus_column = None::<&gtk::TreeViewColumn>;
        with_blocked_signals!(
            self,
            entities_tree_view.set_cursor(&selection_tree_path, focus_column, false),
            entities_tree_view
        );
    }

    fn update_current_entity_view(&self, entity: &Entity) {
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

    fn hide_current_entity_view(&self) {
        fetch_from!(self, entity_specific_box);
        entity_specific_box.hide();
    }
}
