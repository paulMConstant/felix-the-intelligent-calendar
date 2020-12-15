//! Contains helper functions specific to impl Activities.

use super::super::activity_metadata::ActivityMetadata;
use crate::data::ActivityID;
use std::convert::TryFrom;

/// Generates the smallest unused id.
///
/// # Panics
///
/// Panics if there is no id available under 65536.
pub fn generate_next_id(mut used_ids: Vec<&ActivityID>) -> ActivityID {
    // Fetch the ids in ascending order.
    used_ids.sort();

    // If 0 is unused, assign it.
    if used_ids.is_empty() || *used_ids[0] != 0 {
        0
    } else {
        // Compute the difference between neighbours to check for the first hole
        // Example : [0, 1, 2, 4, 5] -> [1, 1, 2, 1] -> tab[2] > 1 : 3 is the hole to fill
        if let Some(index) = used_ids.windows(2).map(|w| w[1] - w[0]).position(|i| i > 1) {
            // Found a hole ! Return its index + 1.
            match ActivityID::try_from(index + 1) {
                Ok(i) => i,
                Err(_) => panic!("All 65536 ids have been used !"),
            }
        } else {
            // Hole not found : return the length of the used ids.
            match ActivityID::try_from(used_ids.len()) {
                Ok(i) => i,
                Err(_) => panic!("All 65536 ids have been used !"),
            }
        }
    }
}

/// Returns the list of incompatible activities for a given activity metadata
/// given all other metadata.
pub fn compute_incompatible_ids(
    metadata: &ActivityMetadata,
    metadata_vec: &Vec<ActivityMetadata>,
) -> Vec<ActivityID> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_next_id() {
        let used_ids: Vec<&ActivityID> = vec![&1, &3, &2, &4, &6, &0];
        let expected_next_id = 5;
        assert_eq!(generate_next_id(used_ids), expected_next_id);

        let used_ids: Vec<&ActivityID> = vec![&1, &3, &2, &4, &6];
        let expected_next_id = 0;
        assert_eq!(generate_next_id(used_ids), expected_next_id);

        let used_ids: Vec<&ActivityID> = vec![&1, &3, &0, &4, &6];
        let expected_next_id = 2;
        assert_eq!(generate_next_id(used_ids), expected_next_id);
    }
    // compute_incompatible_ids is tested in super::activities.rs
}
