#[macro_use]
pub mod fetch_ui;
pub mod helpers;
pub mod signals;

mod activities;
mod activity_insertion;
mod common;
mod entities;
mod groups;
mod work_hours;

use glib::signal::SignalHandlerId;
use gtk::prelude::*;
use std::collections::HashMap;

use activity_insertion::activity_insertion_ui::ActivityInsertionUi;
pub use activity_insertion::entity_to_show::EntityToShow;

use work_hours::WorkHoursBuilder;

use felix_backend::data::{Activity, Entity, Group};

pub struct Ui {
    builder: gtk::Builder,
    signals: HashMap<String, Vec<SignalHandlerId>>,
    current_entity: Option<Entity>,
    current_group: Option<Group>,
    current_activity: Option<Activity>,
    work_hours_builder: WorkHoursBuilder,
    custom_work_hours_builder: WorkHoursBuilder,
    activity_insertion: ActivityInsertionUi,
}

impl Ui {
    pub fn new(builder: gtk::Builder) -> Ui {
        Ui {
            builder,
            signals: HashMap::new(),
            current_entity: None,
            current_group: None,
            current_activity: None,
            work_hours_builder: WorkHoursBuilder::new(),
            custom_work_hours_builder: WorkHoursBuilder::new(),
            activity_insertion: ActivityInsertionUi::new(),
        }
    }

    pub(super) fn init_ui_state(&mut self) {
        self.on_init_activities();
        self.on_init_entities();
        self.on_init_groups();
        self.on_init_activity_insertion();
    }

    #[must_use]
    pub(super) fn work_hours_builder(&mut self) -> &mut WorkHoursBuilder {
        &mut self.work_hours_builder
    }

    #[must_use]
    pub(super) fn custom_work_hours_builder(&mut self) -> &mut WorkHoursBuilder {
        &mut self.custom_work_hours_builder
    }

    #[must_use]
    pub(super) fn current_entity(&self) -> Option<Entity> {
        self.current_entity.clone()
    }

    #[must_use]
    pub(super) fn current_group(&self) -> Option<Group> {
        self.current_group.clone()
    }

    #[must_use]
    pub(super) fn current_activity(&self) -> Option<Activity> {
        self.current_activity.clone()
    }

    pub(super) fn show_mainwindow(&mut self) {
        fetch_from!(self, main_window);
        main_window.show_all();
        self.init_ui_state();
    }
}
