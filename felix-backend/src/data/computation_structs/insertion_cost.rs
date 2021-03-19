use crate::data::Time;

use std::cmp::Ordering;

/// Simple struct holding an insertion time and its cost.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct InsertionCost {
    pub beginning: Time,
    pub cost: usize,
}

impl Ord for InsertionCost {
    fn cmp(&self, other: &Self) -> Ordering {
        self.beginning.cmp(&other.beginning)
    }
}

impl PartialOrd for InsertionCost {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.beginning.cmp(&other.beginning))
    }
}
