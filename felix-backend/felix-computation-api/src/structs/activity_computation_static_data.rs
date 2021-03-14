use std::collections::BTreeSet;

pub struct ActivityComputationStaticData {
    pub possible_insertion_beginnings_minutes_sorted: BTreeSet<u16>,
    pub indexes_of_incompatible_activities: Vec<usize>,
    pub duration_minutes: u16,
}
