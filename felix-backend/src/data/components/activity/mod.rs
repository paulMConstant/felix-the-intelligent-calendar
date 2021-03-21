pub mod activities;
mod activity_computation_data;
mod activity_metadata;
mod computation;

use crate::data::{Time, TimeInterval};
use activity_computation_data::ActivityComputationData;
use activity_metadata::ActivityMetadata;
pub use activity_metadata::Rgba;

use serde::{Deserialize, Serialize};

pub type ActivityId = usize;

/// An activity represents a group of entities which must meet during a defined period of time.
///
/// This structure is read-only. To modify an activity, use the Data structure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Activity {
    metadata: ActivityMetadata,
    computation_data: ActivityComputationData,
}

impl Activity {
    /// Creates a new activity with the given name.
    ///
    /// An activity is split into
    /// * metadata (id, name, entities),
    /// * computation data (duration, time interval if inserted,
    /// incompatible activities, possible insertion times)
    #[must_use]
    fn new(id: ActivityId, name: String) -> Activity {
        Activity {
            metadata: ActivityMetadata::new(id, name),
            computation_data: ActivityComputationData::new(),
        }
    }

    // *** Getters *** - only public API. All operations go through the Activities collection.

    /// Simple getter for the unique id.
    #[must_use]
    pub fn id(&self) -> ActivityId {
        self.metadata.id()
    }

    /// Simple getter for the name.
    #[must_use]
    pub fn name(&self) -> String {
        self.metadata.name().clone()
    }

    /// Simple getter for the entities. The entities are sorted by name.
    #[must_use]
    pub fn entities_sorted(&self) -> Vec<String> {
        self.metadata.entities_sorted()
    }

    /// Simple getter for the groups. The groups are sorted by name.
    #[must_use]
    pub fn groups_sorted(&self) -> Vec<String> {
        self.metadata.groups_sorted()
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

    /// Simple getter for the color.
    #[must_use]
    pub fn color(&self) -> Rgba {
        self.metadata.color()
    }

    /// Simple getter for incompatible activities.
    #[must_use]
    pub(crate) fn incompatible_activity_ids(&self) -> Vec<ActivityId> {
        self.computation_data.incompatible_activity_ids()
    }
}
