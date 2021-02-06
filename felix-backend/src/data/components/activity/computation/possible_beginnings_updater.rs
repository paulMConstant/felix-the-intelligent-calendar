use crate::data::{Time, TimeInterval, MIN_TIME_DISCRETIZATION_MINUTES};

use felix_computation_api::find_possible_beginnings::find_possible_beginnings;

use super::activity_beginnings_given_duration::{
    new_activity_beginnings_given_duration, ActivityBeginningsGivenDuration,
};
use super::work_hours_and_activity_durations_sorted::WorkHoursAndActivityDurationsSorted;

use std::collections::HashMap;

type WorkHoursAndActivityDurationsSortedCache =
    HashMap<WorkHoursAndActivityDurationsSorted, ActivityBeginningsGivenDuration>;

struct PossibleBeginningsUpdater {
    cache: WorkHoursAndActivityDurationsSortedCache,
}

impl PossibleBeginningsUpdater {
    /// Given the activities of an entity, computes the possible beginnings for a set duration.
    ///
    /// This is a pre-computation: it takes into account entities separately, without conflicts.
    /// This function translates Time into u16 so that it can be read by felix-computation-api
    /// and translates it back to Time values.
    fn update_possible_beginnings(
        &mut self,
        work_hours: Vec<TimeInterval>,
        activity_durations: &[Time],
    ) {
        let work_hours_and_activity_durations_sorted =
            WorkHoursAndActivityDurationsSorted::new(work_hours, activity_durations.to_vec());

        // Check for contains_key then get it for borrow checker
        if self
            .cache
            .contains_key(&work_hours_and_activity_durations_sorted)
            == false
        {
            // TODO Threads
            // TODO maybe enqueue the function if the result is not found in the cache ?
            let result = new_activity_beginnings_given_duration(find_possible_beginnings(
                &work_hours_and_activity_durations_sorted.work_hours_in_minutes(),
                &work_hours_and_activity_durations_sorted.activity_durations_in_minutes(),
                MIN_TIME_DISCRETIZATION_MINUTES.into(),
            ));
            self.cache
                .insert(work_hours_and_activity_durations_sorted.clone(), result);
        }
    }
}
