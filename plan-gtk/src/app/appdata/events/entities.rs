use crate::app::appdata::AppData;

use gtk::prelude::*;

impl AppData {
    pub fn add_entity_event(&mut self) {
        fetch_from!(self, add_entity_entry);
        let entity_name = add_entity_entry.get_text();
        add_entity_entry.set_text("");

        if entity_name.trim().is_empty() == false {
            notify_if_err!(self.data.add_entity(entity_name));
            self.update_entities_list();
        }
    }

    fn update_entities_list(&self) {
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

    pub fn entity_selected_event(&mut self, path: &gtk::TreePath) {
        fetch_from!(
            self,
            entities_list_store,
            entity_name_entry,
            entity_send_mail_switch,
            entity_mail_entry,
            entity_custom_work_hours_switch
        );

        let iter = entities_list_store
            .get_iter(path)
            .expect("Invalid selection path");
        let value = entities_list_store.get_value(&iter, 0);
        let selected_entity: &str = value
            .get()
            .expect("Value in list store should be gchararray")
            .expect("Value in list store should be gchararray");

        assign_or_return!(entity, self.data.entity(selected_entity));

        self.state.current_entity = Some(entity.name());

        with_blocked_signals!(self, {
        entity_name_entry.set_text(&entity.name());
        entity_mail_entry.set_text(&entity.mail());
        entity_custom_work_hours_switch.set_active(entity.custom_work_hours().is_empty() == false);
        entity_send_mail_switch.set_active(entity.send_me_a_mail());
        },
        entity_name_entry, entity_mail_entry, entity_custom_work_hours_switch, entity_send_mail_switch);
    }

    pub fn set_entity_mail_event(&mut self) {
        fetch_from!(self, entity_mail_entry);

        let mail = entity_mail_entry.get_text();
        let entity = self.state.current_entity.as_ref().expect("Current entity should be selected before accessing any entity-related field");
        notify_if_err!(self.data.set_entity_mail(entity, mail));
    }

    pub fn set_send_mail_event(&mut self) {
        fetch_from!(self, entity_send_mail_switch);

        let send = entity_send_mail_switch.get_active();
        let entity = self.state.current_entity.as_ref().expect("Current entity should be selected before accessing any entity-related field");
        notify_if_err!(self.data.set_send_mail_to(entity, send));
    }
}
