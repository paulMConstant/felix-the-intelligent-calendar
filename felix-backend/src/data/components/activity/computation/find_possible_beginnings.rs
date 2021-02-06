use crate::data::{Time, TimeInterval, MIN_TIME_DISCRETIZATION_MINUTES};

use felix_computation_api::{
    find_possible_beginnings,
    find_possible_beginnings::{ActivityBeginnignsGivenDuration, WorkHourInMinutes},
};

/// Given the activities of an entity, computes the possible beginnings for a set duration.
///
/// This is a pre-computation: it takes into account entities separately, without conflicts.
fn find_possible_beginnings(
    mut work_hours: Vec<TimeInterval>,
    activity_durations: &[Time],
) -> ActivityBeginnignsGivenDuration {
    // Turn time structs into minutes
    let mut activity_durations = activity_durations
        .iter()
        .map(|time| time.total_minutes())
        .collect::<Vec<_>>();
    activity_durations.sort();

    work_hours.sort_by(|a, b| a.duration().cmp(&b.duration()));
    let work_hours_in_minutes = work_hours
        .iter()
        .map(|&time_interval| {
            WorkHourInMinutes::new(
                time_interval.beginning().total_minutes(),
                time_interval.end().total_minutes(),
            )
        })
        .collect::<Vec<_>>();

    find_possible_beginnings::find_possible_beginnings(
        &work_hours_in_minutes,
        &activity_durations,
        MIN_TIME_DISCRETIZATION_MINUTES.into(),
    )
}
