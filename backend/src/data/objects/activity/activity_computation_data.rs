use super::super::time::{time_interval::TimeInterval, Time, MIN_TIME_DISCRETIZATION};
use std::collections::HashSet;

/// Holds computation-related data : duration, insertion interval if inserted,
/// incompatible activities, possible insertion times.
#[derive(Debug, Clone)]
pub struct ActivityComputationData {
    duration: Time,
    insertion_interval: Option<TimeInterval>,
    possible_insertion_beginnings_if_no_conflict: HashSet<Time>,
    incompatible_activity_ids: Vec<u16>,
}

impl ActivityComputationData {
    /// Creates new computation data.
    pub fn new() -> ActivityComputationData {
        ActivityComputationData {
            duration: MIN_TIME_DISCRETIZATION,
            insertion_interval: None,
            possible_insertion_beginnings_if_no_conflict: HashSet::new(),
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
    pub fn possible_insertion_beginnings(&self) -> &HashSet<Time> {
        // TODO take conflicts into account
        &self.possible_insertion_beginnings_if_no_conflict
    }

    /// Getter for incompatible activities, used for testing. Should not go out of this module.
    #[cfg(test)]
    #[must_use]
    pub fn incompatible_activity_ids(&self) -> &Vec<u16> {
        &self.incompatible_activity_ids
    }

    // *** Setters ***

    /// Simple setter for the duration.
    ///
    /// # Errors
    ///
    /// Returns Err if the duration is too short (< MIN\_TIME\_DISCRETIZATION).
    pub fn set_duration(&mut self, duration: Time) -> Result<(), String> {
        if duration < MIN_TIME_DISCRETIZATION {
            Err("The given duration is too short !".to_owned())
        } else {
            self.duration = duration;
            Ok(())
        }
    }

    /// Simple setter for incompatible activity ids.
    ///
    /// Does not perform any checks, the activities collection does it.
    pub fn set_incompatible_activity_ids(&mut self, incompatible_ids: Vec<u16>) {
        self.incompatible_activity_ids = incompatible_ids;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_activity_duration() {
        let mut computation_data = ActivityComputationData::new();

        let invalid_duration = Time::new(0, 0);
        assert!(computation_data.set_duration(invalid_duration).is_err());
        assert_eq!(computation_data.duration(), MIN_TIME_DISCRETIZATION);

        let valid_duration = Time::new(0, 10);
        assert!(computation_data.set_duration(valid_duration).is_ok());
        assert_eq!(computation_data.duration(), valid_duration);
    }
}
