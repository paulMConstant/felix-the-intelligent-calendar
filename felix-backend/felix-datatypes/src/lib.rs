use std::collections::{HashMap, HashSet};

mod time;
mod insertion_cost;

pub use time::{Time, TimeInterval, MIN_TIME_DISCRETIZATION};
pub use insertion_cost::{insertion_cost_minutes::InsertionCostsMinutes, InsertionCost};

/// Each entity has a set of possible insertion times for every activity duration it has.
/// Times are represented in total minutes.
pub type ActivityBeginningsGivenDurationMinutes =
    HashMap<ActivityDurationMinutes, HashSet<ActivityBeginningMinutes>>;

pub type ActivityDurationMinutes = u16;
pub type Cost = usize;
pub type ActivityBeginningMinutes = u16;
