pub mod insertion_cost_minutes;

use crate::{Time, InsertionCostsMinutes};

use std::cmp::Ordering;

/// Simple struct holding an insertion time and its cost.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct InsertionCost {
    pub beginning: Time,
    pub cost: usize,
}

impl InsertionCost {
    #[must_use]
    pub fn new(beginning: Time, cost: usize) -> InsertionCost {
        InsertionCost { beginning, cost }
    }

    #[must_use]
    pub fn from_insertion_cost_minutes(
        insertion_cost_minutes: InsertionCostsMinutes,
    ) -> InsertionCost {
        InsertionCost {
            beginning: Time::from_total_minutes(insertion_cost_minutes.beginning_minutes),
            cost: insertion_cost_minutes.cost,
        }
    }
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
