use super::Schedules;

use super::drawing::{get_center_of_remove_button, REMOVE_BUTTON_RADIUS};
use crate::app::ui::ActivityToShow;
use felix_data::Time;

#[must_use]
pub(super) fn get_name_of_entity_from_x(x: i32, schedules: &Schedules) -> Option<String> {
    let index_of_entity = (x / schedules.width_per_schedule as i32) as usize;

    if index_of_entity < schedules.entities_to_show.len() {
        Some(schedules.entities_to_show[index_of_entity].name().clone())
    } else {
        None
    }
}

#[must_use]
pub(super) fn get_time_on_y(y: i32, schedules: &Schedules) -> Time {
    let n_times_min_discretization = (y as f64 / schedules.height_per_min_discretization) as i32;
    Time::from_n_times_min_discretization(n_times_min_discretization)
}

#[must_use]
pub(super) fn get_activity_under_cursor(
    x: i32,
    y: i32,
    schedules: &Schedules,
) -> Option<ActivityToShow> {
    if let Some(entity) = get_name_of_entity_from_x(x, schedules) {
        let time = get_time_on_y(y, schedules);
        // Check if an activity has the given insertion time
        // If yes, return its id
        schedules
            .entities_to_show
            .iter()
            .find(|other_entity| other_entity.name() == &entity)
            .map(|entity| {
                entity
                    .activities()
                    .iter()
                    .find(|activity| {
                        activity.insertion_interval().is_some()
                            && activity.insertion_interval().unwrap().contains(time)
                    })
                    .cloned()
            })
            .unwrap_or(None)
    } else {
        None
    }
}

/// Returns the name of the entity to delete if the cursor is over the button to remove its
/// schedule.
#[must_use]
pub(super) fn get_entity_to_remove_under_cursor(
    x: f64,
    y: f64,
    schedules: &Schedules,
) -> Option<String> {
    // Check if the cursor is inside any of the "remove" icons
    schedules
        .entities_to_show
        .iter()
        .enumerate()
        .find(|(index, _entity_to_show)| {
            let (x_button, y_button) = get_center_of_remove_button(
                &schedules,
                *index as f64 * schedules.width_per_schedule,
            );
            // Use euclidian distance to the center of the remove button which is a circle
            // No sqrt because it is useless for the sake of comparison
            (x - x_button).powi(2) + (y - y_button).powi(2) <= REMOVE_BUTTON_RADIUS.powi(2)
        })
        .map(|(_index, entity_to_show)| entity_to_show.name().clone())
}
