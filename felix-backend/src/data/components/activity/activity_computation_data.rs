use crate::data::{ActivityID, Time, TimeInterval, MIN_TIME_DISCRETIZATION};
use std::collections::HashSet;

/// Holds computation-related data : duration, insertion interval if inserted,
/// incompatible activities, possible insertion times.
#[derive(Debug, Clone)]
pub struct ActivityComputationData {
    duration: Time,
    insertion_interval: Option<TimeInterval>,
    possible_insertion_times_if_no_conflict: HashSet<Time>,
    incompatible_activity_ids: Vec<ActivityID>,
}

impl ActivityComputationData {
    /// Creates new computation data.
    pub fn new() -> ActivityComputationData {
        ActivityComputationData {
            duration: MIN_TIME_DISCRETIZATION,
            insertion_interval: None,
            possible_insertion_times_if_no_conflict: HashSet::new(),
            incompatible_activity_ids: Vec::new(),
        }
    }

    // *** Getters ***

    /// Simple getter for the duration.
    #[must_use]
    pub fn duration(&self) -> Time {
        self.duration
    }

    /// Simple getter for the insertion interval.
    #[must_use]
    pub fn insertion_interval(&self) -> Option<TimeInterval> {
        self.insertion_interval
    }

    /// Simple getter for possible insertion times.
    #[must_use]
    pub fn possible_insertion_times_if_no_conflict(&self) -> &HashSet<Time> {
        &self.possible_insertion_times_if_no_conflict
    }

    /// Simple getter for incompatible activities.
    #[must_use]
    pub fn incompatible_activity_ids(&self) -> Vec<ActivityID> {
        self.incompatible_activity_ids.clone()
    }

    // *** Setters ***

    /// Simple setter for the duration.
    ///
    /// If the duration is shorter than the current one, updates the current insertion time.
    /// If the duration is greater, we don't know where the activity will fit. It is the
    /// responsibility of higher level collections to deal with it.
    pub fn set_duration(&mut self, duration: Time) {
        if duration < self.duration {
            if let Some(insertion_interval) = self.insertion_interval {
                self.insertion_interval = Some(TimeInterval::new(
                    insertion_interval.beginning(),
                    insertion_interval.beginning() + duration,
                ));
            }
        }
        println!("Set duration {} to {}", self.duration, duration);
        self.duration = duration;
    }

    /// Simple setter for incompatible activity ids.
    ///
    /// Does not perform any checks, the activities collection does it.
    pub fn set_incompatible_activity_ids(&mut self, incompatible_ids: Vec<ActivityID>) {
        self.incompatible_activity_ids = incompatible_ids;
    }

    /// Simple setter for possible beginnings if no conflicts.
    ///
    /// Does not perform any check, the activities collection does it.
    pub fn set_possible_insertion_times_if_no_conflict(
        &mut self,
        possible_insertion_times_if_no_conflict: HashSet<Time>,
    ) {
        self.possible_insertion_times_if_no_conflict = possible_insertion_times_if_no_conflict;
    }

    /// Inserts the activity at given time.
    /// If None is given, the activity is removed from the schedule.
    ///
    /// Does not perform any checks, data should be sanitized above.
    ///
    /// # Panics
    ///
    /// Panics if the insertion time + duration Time is invalid.
    pub fn insert(&mut self, insertion_time: Option<Time>) {
        if let Some(insertion_time) = insertion_time {
            self.insertion_interval = Some(TimeInterval::new(
                insertion_time,
                insertion_time + self.duration,
            ));
        } else {
            self.insertion_interval = None;
        }
    }
}

// No tests, functions are tested in tests directory
impl Eq for ActivityComputationData {}
impl PartialEq for ActivityComputationData {
    fn eq(&self, other: &Self) -> bool {
        // Don't check for possible activity insertions because they are asynchronously calculated
        self.duration == other.duration
            && self.insertion_interval == other.insertion_interval
            && self.incompatible_activity_ids == other.incompatible_activity_ids
    }
}
