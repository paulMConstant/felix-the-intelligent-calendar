use crate::data::{computation_structs::WorkHoursAndActivityDurationsSorted, Time};

use felix_computation_api::find_possible_beginnings;

use super::{
    activity_beginnings_given_duration::{
        new_activity_beginnings_given_duration, ActivityBeginningsGivenDuration,
    },
    thread_pool::ThreadPool,
};

use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use std_semaphore::Semaphore;

pub type PossibleBeginningsComputationPool =
    HashMap<WorkHoursAndActivityDurationsSorted, ActivityBeginningsGivenDuration>;

/// Keeps track of which activity possible beginnings are out of date
/// and handles the computation.
///
/// This class is NOT thread-safe, it only runs the computations in a separate thread pool.
#[derive(Debug)]
pub struct PossibleBeginningsPool {
    // Prototype design pattern : all results are stored in RAM and computed only once
    computation_pool: Arc<Mutex<PossibleBeginningsComputationPool>>,
    thread_pool: Rc<ThreadPool>,
}

impl PossibleBeginningsPool {
    pub(crate) fn new(
        computation_pool: Arc<Mutex<PossibleBeginningsComputationPool>>,
        thread_pool: Rc<ThreadPool>,
    ) -> PossibleBeginningsPool {
        PossibleBeginningsPool {
            computation_pool,
            thread_pool,
        }
    }

    /// For the given work hours and activity durations, computes the possible activity beginnings.
    pub fn queue_work_hours_and_activity_durations(
        &mut self,
        work_hours_and_activity_durations: Vec<WorkHoursAndActivityDurationsSorted>,
        computation_done_semaphore: Arc<Semaphore>,
    ) {
        for key in work_hours_and_activity_durations {
            if self.computation_pool.lock().unwrap().contains_key(&key) {
                // Result already in - Notify that the computation is already done
                computation_done_semaphore.release();
            } else {
                let pool = self.computation_pool.clone();

                let computation_done_semaphore = computation_done_semaphore.clone();
                // Launch the computation in a separate thread
                self.thread_pool.spawn(move || {
                    let activity_beginnings_given_duration_minutes = find_possible_beginnings(
                        &key.work_hours_in_minutes(),
                        &key.activity_durations_in_minutes(),
                    );

                    let result = new_activity_beginnings_given_duration(
                        activity_beginnings_given_duration_minutes,
                    );

                    pool.lock().unwrap().insert(key.clone(), result);
                    // One result was done
                    computation_done_semaphore.release();
                });
            }
        }
    }

    /// Fuses the possible beginnings given every work\_hour\_and\_activity\_duration key.
    #[must_use]
    pub fn poll_and_fuse_possible_beginnings(
        &mut self,
        schedules_of_participants: &[WorkHoursAndActivityDurationsSorted],
        duration: Time,
        // TODO should return nothing or just a bool
    ) -> Option<HashSet<Time>> {
        let pool = self.computation_pool.lock().unwrap();

        // Fetch possible beginnings
        let maybe_all_possible_beginnings: Option<Vec<_>> = schedules_of_participants
            .iter()
            .map(|work_hours_and_activity_durations| {
                pool.get(work_hours_and_activity_durations)
                    .map(|beginnings_given_duration| {
                        // If Some, then computation result is there.
                        // Fetch only the possible beginnings for the specified duration.
                        beginnings_given_duration.get(&duration).expect(
                            "Activity duration not in durations calculated for participants",
                        )
                    })
            })
            .collect();

        // Intersect all possible beginnings
        if let Some(mut all_possible_beginnings) = maybe_all_possible_beginnings {
            // Sort sets by ascending size so that fewer checks are done for intersections
            all_possible_beginnings.sort_by_key(|a| a.len());

            let first_set = all_possible_beginnings.first();

            if let Some(first_set) = first_set {
                let intersection = first_set
                    .iter()
                    .filter(|k| all_possible_beginnings[1..].iter().all(|s| s.contains(k)));

                Some(intersection.copied().collect())
            } else {
                // No possible beginnings
                Some(HashSet::new())
            }
        } else {
            // At least one computation result was missing
            None
        }
    }
}
