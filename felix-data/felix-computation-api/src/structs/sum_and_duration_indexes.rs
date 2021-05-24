use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SumAndDurationIndexes {
    pub sum_minutes: u16,
    pub indexes: HashSet<u16>,
}

impl SumAndDurationIndexes {
    pub fn new() -> SumAndDurationIndexes {
        SumAndDurationIndexes {
            sum_minutes: 0,
            indexes: HashSet::new(),
        }
    }
}

impl Default for SumAndDurationIndexes {
    fn default() -> Self {
        SumAndDurationIndexes::new()
    }
}

impl Ord for SumAndDurationIndexes {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.sum_minutes.cmp(&other.sum_minutes)
    }
}

impl PartialOrd for SumAndDurationIndexes {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.sum_minutes.cmp(&other.sum_minutes))
    }
}
