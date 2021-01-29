use super::drawing::get_height_for_one_hour;
use crate::app::ui::EntityToShow;

use felix_backend::data::{ActivityID, Time, MIN_TIME_DISCRETIZATION};

use std::sync::Arc;

const MIN_SCHEDULE_WIDTH: f64 = 200.0;
const MAX_SCHEDULE_WIDTH: f64 = 450.0;

pub struct TimeTooltipToDraw {
    pub x_cursor: f64,
    pub y_cursor: f64,
    pub time: Time,
}

/// Holds data required to interact with the schedules drawing area.
pub struct Schedules {
    pub entities_to_show: Vec<EntityToShow>,
    pub width_per_schedule: f64,
    pub height_per_min_discretization: f64,
    pub try_insert_activity_callback: Arc<dyn Fn(String, ActivityID, Time)>,
    pub time_tooltip_to_draw: Option<TimeTooltipToDraw>,
}

impl Schedules {
    #[must_use]
    pub fn new() -> Schedules {
        Schedules {
            entities_to_show: Vec::new(),
            width_per_schedule: 0.0,
            height_per_min_discretization: 0.0,
            try_insert_activity_callback: Arc::new(Box::new(|_, _, _| {
                panic!("Insert activity callback has not been initialized !")
            })),
            time_tooltip_to_draw: None,
        }
    }

    pub fn compute_schedule_width(&mut self, visible_width: f64) {
        self.width_per_schedule = visible_width / self.entities_to_show.len() as f64;

        self.width_per_schedule = self.width_per_schedule.max(MIN_SCHEDULE_WIDTH);
        self.width_per_schedule = self.width_per_schedule.min(MAX_SCHEDULE_WIDTH);
    }

    pub fn compute_height_for_min_discretization(&mut self, visible_height: f64) {
        let num_min_discretization_in_hour = 60 / MIN_TIME_DISCRETIZATION.minutes();
        self.height_per_min_discretization =
            get_height_for_one_hour(visible_height) / num_min_discretization_in_hour as f64;
    }
}
