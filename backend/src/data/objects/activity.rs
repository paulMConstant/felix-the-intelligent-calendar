pub mod activities;
mod activity_computation_data;
mod activity_metadata;

use super::super::{Time, TimeInterval};
use activity_computation_data::ActivityComputationData;
use activity_metadata::ActivityMetadata;
use std::collections::HashSet;

/// An activity represents a group of entities which must meet during a defined period of time.
#[derive(Debug)]
pub struct Activity {
    metadata: ActivityMetadata,
    computation_data: ActivityComputationData,
}

impl Activity {
    /// Creates a new activity with the given name.
    ///
    /// An activity is split into
    /// * metadata (id, name, participants),
    /// * computation data (duration, time interval if inserted,
    /// incompatible activities, possible insertion times)
    #[must_use]
    fn new<S>(id: u16, name: S) -> Activity
    where
        S: Into<String>,
    {
        Activity {
            metadata: ActivityMetadata::new(id, name),
            computation_data: ActivityComputationData::new(),
        }
    }

    // *** Getters *** - only public API. All operations go through the Activities collection.

    /// Simple getter for the unique id.
    #[must_use]
    pub fn id(&self) -> u16 {
        self.metadata.id()
    }

    /// Simple getter for the name.
    #[must_use]
    pub fn name(&self) -> String {
        self.metadata.name().clone()
    }

    /// Simple getter for the participants. The participants are sorted by name.
    #[must_use]
    pub fn participants_sorted(&self) -> Vec<String> {
        self.metadata.participants_sorted()
    }

    /// Simple getter for the duration.
    #[must_use]
    pub fn duration(&self) -> Time {
        self.computation_data.duration()
    }

    /// Simple getter for the insertion interval.
    /// Returns None if the activity is not inserted.
    #[must_use]
    pub fn insertion_interval(&self) -> Option<TimeInterval> {
        self.computation_data.insertion_interval()
    }

    /// Simple getter for possible insertion times.
    #[must_use]
    pub fn possible_insertion_beginnings(&self) -> HashSet<Time> {
        self.computation_data
            .possible_insertion_beginnings()
            .clone()
    }
}

impl Eq for Activity {}
impl PartialEq for Activity {
    fn eq(&self, other: &Self) -> bool {
        self.metadata.id() == other.metadata.id()
    }
}
