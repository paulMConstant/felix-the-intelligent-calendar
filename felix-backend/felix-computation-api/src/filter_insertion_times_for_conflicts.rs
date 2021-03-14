use crate::structs::{
    ActivityComputationStaticData,
    ActivityInsertionBeginningMinutes,
};

use std::collections::BTreeSet;

pub fn filter_insertion_times_for_conflicts(static_data: &[ActivityComputationStaticData],
                                            insertion_data: &[ActivityInsertionBeginningMinutes],
                                            index_of_activity_to_check: usize)
-> BTreeSet<u16> {
    unsafe {
        let activity_data = static_data
            .get_unchecked(index_of_activity_to_check);

        let mut possible_beginnings = activity_data
            .possible_insertion_beginnings_minutes_sorted
            .clone();

        for (incompatible_beginning, incompatible_end) in activity_data
            .indexes_of_incompatible_activities
            .iter()
            .copied()
            .filter_map(|index| {
                if let Some(incompatible_beginning) = insertion_data.get_unchecked(index) {
                    Some((incompatible_beginning, 
                     incompatible_beginning + static_data.get_unchecked(index).duration_minutes))
                } else {
                    None
                }
            }) {
                // Need to collect for borrow checker
                let values_to_remove = possible_beginnings.range(
                    incompatible_beginning..&incompatible_end
                ).copied().collect::<Vec<_>>();
        
                for value_to_remove in values_to_remove {
                    possible_beginnings.remove(&value_to_remove);
                }
        }
        possible_beginnings
    }
}
