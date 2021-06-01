#[macro_use]
pub mod macros;
mod activities;
mod activity_insertion;
mod entities;
mod groups;
mod work_hours;
mod notification;
mod export;

use crate::app::ui::Ui;
use gtk::prelude::*;

impl Ui {
    #[must_use]
    pub fn main_window(&self) -> gtk::Window {
        fetch_ui_from_builder!(self, "MainWindow")
    }

    #[must_use]
    pub fn data_window(&self) -> gtk::Window {
        fetch_ui_from_builder!(self, "DataWindow")
    }

    #[must_use]
    pub fn data_button(&self) -> gtk::Button {
        fetch_ui_from_builder!(self, "DataButton")
    }

    #[must_use]
    pub fn entity_and_group_completion_list_store(&self) -> gtk::ListStore {
        fetch_ui_from_builder!(self, "EntityAndGroupCompletionListStore")
    }
}
