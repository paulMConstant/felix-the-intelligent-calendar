use crate::data::{Time, TimeInterval, MIN_TIME_DISCRETIZATION_MINUTES};

use felix_computation_api::find_possible_beginnings;

/// Given the activities of an entity, computes the possible beginnings for a set duration.
///
/// This is a pre-computation: it takes into account entities separately, without conflicts.
fn find_possible_beginnings(
    work_hours: &[TimeInterval],
    activity_durations: &[Time],
) -> find_possible_beginnings::ActivityBeginnignsGivenDuration {
    // Turn time structs into minutes
    let activity_durations = activity_durations
        .iter()
        .map(|time| time.total_minutes())
        .collect::<Vec<_>>();

    let work_hour_beginnings = work_hours
        .iter()
        .map(|&time_interval| time_interval.beginning().total_minutes())
        .collect::<Vec<_>>();

    let work_hour_ends = work_hours
        .iter()
        .map(|&time_interval| time_interval.end().total_minutes())
        .collect::<Vec<_>>();

    let work_hour_durations = work_hours
        .iter()
        .map(|&time_interval| time_interval.duration().total_minutes())
        .collect::<Vec<_>>();

    // TODO import struct {beginning, end} for work hours from felix_computation_api
    find_possible_beginnings::find_possible_beginnings(
        work_hour_beginnings,
        work_hour_ends,
        work_hour_durations,
        activity_durations,
        MIN_TIME_DISCRETIZATION_MINUTES.into(),
    )
}
