use crate::app::{
    config,
    ui::{EntityToShow, UiState},
    App,
};

impl App {
    pub fn recreate_last_ui_state(&mut self) {
        // Fetch ui from serialized file
        let ui_state_file_contents = std::fs::read_to_string(config::UI_STATE_FILE);

        let ui_state = if let Ok(contents) = ui_state_file_contents {
            let ui_state_value: serde_json::Result<UiState> = serde_json::from_str(&contents);
            if let Ok(ui_state) = ui_state_value {
                ui_state
            } else {
                UiState::default()
            }
        } else {
            UiState::default()
        };

        for entity_name in ui_state.entities_whose_schedules_are_shown {
            self.ui
                .borrow_mut()
                .on_show_entity_schedule(EntityToShow::new(entity_name, &self.data.borrow()));
        }
    }
}
