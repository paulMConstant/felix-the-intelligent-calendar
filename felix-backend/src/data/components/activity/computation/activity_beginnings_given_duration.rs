use crate::data::Time;

use felix_computation_api::find_possible_beginnings::ActivityBeginningsGivenDurationMinutes;

use std::collections::{HashMap, HashSet};

pub type ActivityBeginningsGivenDuration = HashMap<Time, HashSet<Time>>;

/// Translates ActivityBeginningsGivenDurationMinutes to ActivityBeginningsGivenDuration (hours AND
/// minutes.
pub fn new_activity_beginnings_given_duration(
    activity_beginnings_given_duration_minutes: ActivityBeginningsGivenDurationMinutes,
) -> ActivityBeginningsGivenDuration {
    let mut res = ActivityBeginningsGivenDuration::new();
    for (activity_duration, possible_beginnings) in activity_beginnings_given_duration_minutes {
        res.insert(
            Time::from_total_minutes(activity_duration),
            possible_beginnings
                .iter()
                .map(|&possible_beginning_minutes| {
                    Time::from_total_minutes(possible_beginning_minutes)
                })
                .collect(),
        );
    }
    res
}
