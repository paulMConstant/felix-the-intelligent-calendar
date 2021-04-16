//! Utilities used by the activities collections.

use super::{
    Activities, 
    super::{
        ActivityMetadata,
        computation::id_computation::compute_incompatible_ids,
    },
};
use crate::data::{ActivityId, Activity};

impl Activities {
    /// Simple private mutable getter for an activity.
    ///
    /// # Panics
    ///
    /// Panics if the activity with given ID does not exist.
    pub(super) fn get_mut_by_id(&mut self, id: ActivityId) -> &mut Activity {
        self.activities
            .iter_mut()
            .find(|activity| activity.id() == id)
            .expect("Asking for activity which does not exist")
    }

    /// Updates the incompatible activity ids of each activity.
    ///
    /// Used for internal computation only.
    pub(super) fn update_incompatible_activities(&mut self) {
        // 1. Create a copy of the metadata
        let metadata_vec: Vec<ActivityMetadata> = self
            .activities
            .iter()
            .map(|activity| activity.metadata.clone())
            .collect();

        // 2. Iterate over the copied metadata to fill incompatible ids (activities which
        // have at least one entity in common are incompatible).
        // If the activity has the same id, it is the same activity, don't add it
        for metadata in &metadata_vec {
            self.get_mut_by_id(metadata.id())
                .computation_data
                .set_incompatible_activity_ids(compute_incompatible_ids(&metadata, &metadata_vec));
        }
    }
}
