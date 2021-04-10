use crate::data::{ActivityId, ActivityInsertionCosts};

use super::{possible_beginnings_pool::PossibleBeginningsComputationPool, thread_pool::ThreadPool};

use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

/// Invalidates, computes the insertion costs of activities and revalidates them.
/// Computation and revalidation are done in a separate thread.
#[derive(Debug)]
pub struct InsertionCostsUpdater {
    pub activities_insertion_costs: HashMap<ActivityId, Arc<Mutex<ActivityInsertionCosts>>>,
    possible_beginnings_pool: Arc<Mutex<PossibleBeginningsComputationPool>>,
    thread_pool: Rc<ThreadPool>,
}

impl InsertionCostsUpdater {
    pub(crate) fn new(
        possible_beginnings_pool: Arc<Mutex<PossibleBeginningsComputationPool>>,
        thread_pool: Rc<ThreadPool>,
    ) -> Self {
        InsertionCostsUpdater {
            activities_insertion_costs: HashMap::new(),
            possible_beginnings_pool,
            thread_pool,
        }
    }
}
