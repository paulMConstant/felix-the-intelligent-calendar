use serde::{Deserialize, Serialize};

/// Contains the UI state to restore when the app opens again.
#[derive(Serialize, Deserialize, Debug)]
pub struct UiState {
    pub entities_whose_schedules_are_shown: Vec<String>,
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            entities_whose_schedules_are_shown: Vec::new(),
        }
    }
}
