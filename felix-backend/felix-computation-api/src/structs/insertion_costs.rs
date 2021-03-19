use crate::structs::{ActivityBeginningMinutes, Cost};

/// A simple struct holding the beginning of an activity in minutes and its cost.
/// The higher the cost, the more the activtiy blocks other activities.
#[derive(Debug, Clone, Copy)]
pub struct InsertionCostsMinutes {
    pub beginning_minutes: ActivityBeginningMinutes,
    pub cost: Cost,
}
