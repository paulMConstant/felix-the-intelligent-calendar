use crate::data::{computation_structs::WorkHoursAndActivityDurationsSorted};

use super::computation_done_semaphore::Semaphore;

use super::{
    thread_pool::ThreadPool,
};

use std::rc::Rc;
use std::sync::{Arc, Mutex};


#[derive(Debug)]
pub struct PossibleBeginningsPool {
    // Prototype design pattern : all results are computed only once then stored
    computation_pool: Arc<Mutex<PossibleBeginningsComputationPool>>,
    thread_pool: Rc<ThreadPool>,
    computation_done_semaphore: Arc<Semaphore>,
}

impl PossibleBeginningsPool {
    pub(crate) fn new(
        computation_pool: Arc<Mutex<PossibleBeginningsComputationPool>>,
        thread_pool: Rc<ThreadPool>,
        computation_done_semaphore: Arc<Semaphore>,
    ) -> PossibleBeginningsPool {
        PossibleBeginningsPool {
            computation_pool,
            thread_pool,
            computation_done_semaphore,
        }
    }

}
