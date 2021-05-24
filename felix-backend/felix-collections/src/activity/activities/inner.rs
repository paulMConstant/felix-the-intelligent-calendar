//! Utilities used by the activities collections.

use super::{
    super::{computation::id_computation::compute_incompatible_ids, ActivityMetadata},
    Activities,
};
use crate::Activity;

use felix_datatypes::{ActivityId};

impl Activities {
    /// Performs the given operation on the activity with given id.
    ///
    /// # Panics
    ///
    /// Panics if the activity with given ID does not exist.
    // Having this function accept a closure ensures that the mutex is always locked.
    // It is not possible to return a &mut Activity from inside the mutex because once the mutex
    // goes out of scope, it is unlocked.
    pub(super) fn mutate_activity<Res>(
        &self,
        id: ActivityId,
        operation: impl FnOnce(&mut Activity) -> Res,
    ) -> Res {
        operation(
            self.activities
                .lock()
                .unwrap()
                .iter_mut()
                .find(|activity| activity.id() == id)
                .expect("Asking for activity which does not exist"),
        )
    }

    /// Updates the incompatible activity ids of each activity.
    ///
    /// Used for internal computation only.
    pub(super) fn update_incompatible_activities(&self) {
        // 1. Create a copy of the metadata
        let metadata_vec: Vec<ActivityMetadata> = self
            .activities
            .lock()
            .unwrap()
            .iter()
            .map(|activity| activity.metadata.clone())
            .collect();

        // 2. Iterate over the copied metadata to fill incompatible ids (activities which
        // have at least one entity in common are incompatible).
        // If the activity has the same id, it is the same activity, don't add it
        for metadata in &metadata_vec {
            self.mutate_activity(metadata.id(), |activity| {
                activity
                    .computation_data
                    .set_incompatible_activity_ids(compute_incompatible_ids(
                        &metadata,
                        &metadata_vec,
                    ));
            });
        }
    }
}
