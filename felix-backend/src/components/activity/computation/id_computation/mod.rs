//! Contains helper functions specific to impl Activities.

#[cfg(test)]
mod tests;

use super::super::activity_metadata::ActivityMetadata;
use crate::ActivityId;

/// Generates the smallest unused id.
pub fn generate_next_id(mut used_ids: Vec<ActivityId>) -> ActivityId {
    // Fetch the ids in ascending order.
    used_ids.sort_unstable();

    // If 0 is unused, assign it.
    if used_ids.is_empty() || used_ids[0] != 0 {
        0
    } else {
        // Compute the difference between neighbours to check for the first hole
        // Example : [0, 1, 2, 4, 5] -> [1, 1, 2, 1] -> tab[2] > 1 : 3 is the hole to fill
        if let Some(index) = used_ids.windows(2).map(|w| w[1] - w[0]).position(|i| i > 1) {
            // Found a hole ! Return the index + 1 (cf example : index 2 means next number is 3)
            index + 1
        } else {
            // Hole not found : all numbers from 0 to (len - 1) are taken.
            used_ids.len()
        }
    }
}

/// Returns the list of incompatible activities for a given activity metadata
/// given all other metadata.
pub fn compute_incompatible_ids(
    metadata: &ActivityMetadata,
    metadata_vec: &[ActivityMetadata],
) -> Vec<ActivityId> {
    metadata_vec
        .iter()
        // The entities have one element in common
        .filter(|other_metadata| {
            metadata.id() != other_metadata.id()
                && metadata
                    .entities_as_set()
                    .intersection(other_metadata.entities_as_set())
                    .next()
                    != None
        })
        .map(|other_metadata| other_metadata.id())
        .collect()
}
