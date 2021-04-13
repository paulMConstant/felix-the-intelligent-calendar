use crate::data::{ActivityId, InsertionCost, Time, TimeInterval, MIN_TIME_DISCRETIZATION};

use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, Mutex};

pub type ActivityInsertionCosts = Option<BTreeSet<InsertionCost>>;

/// Holds computation-related data : duration, insertion interval if inserted,
/// incompatible activities, possible insertion times.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityComputationData {
    duration: Time,
    insertion_interval: Option<TimeInterval>,
    /// Kept in a arc because updated when necessary in separate threads.
    /// None means not computed yet (invalidated).
    /// Some means up to date.
    #[serde(skip)]
    insertion_costs: Arc<Mutex<ActivityInsertionCosts>>,
    incompatible_activity_ids: Vec<ActivityId>,
}

impl ActivityComputationData {
    /// Creates new computation data.
    pub fn new() -> ActivityComputationData {
        ActivityComputationData {
            duration: MIN_TIME_DISCRETIZATION,
            insertion_interval: None,
            insertion_costs: Arc::new(Mutex::new(None)),
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

    /// Simple getter for insertion costs.
    #[must_use]
    pub fn insertion_costs(&self) -> Arc<Mutex<ActivityInsertionCosts>> {
        self.insertion_costs.clone()
    }

    /// Simple getter for incompatible activities.
    #[must_use]
    pub fn incompatible_activity_ids(&self) -> Vec<ActivityId> {
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
        self.duration = duration;
    }

    /// Simple setter for incompatible activity ids.
    ///
    /// Does not perform any checks, the activities collection does it.
    pub fn set_incompatible_activity_ids(&mut self, incompatible_ids: Vec<ActivityId>) {
        self.incompatible_activity_ids = incompatible_ids;
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

impl Hash for ActivityComputationData {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Don't hash possible activity insertions because they are asynchronously calculated
        self.duration.hash(state);
        self.insertion_interval.hash(state);
        self.incompatible_activity_ids.hash(state);
    }
}
