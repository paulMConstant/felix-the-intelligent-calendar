use crate::data::{Activity, ActivityID, Time, MIN_TIME_DISCRETIZATION_MINUTES};

use felix_computation_api::find_possible_beginnings::find_possible_beginnings;

use super::activity_beginnings_given_duration::{
    new_activity_beginnings_given_duration, ActivityBeginningsGivenDuration,
};
use super::work_hours_and_activity_durations_sorted::WorkHoursAndActivityDurationsSorted;

use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::sync::{Arc, Mutex};

type WorkHoursAndActivityDurationsSortedCache =
    HashMap<WorkHoursAndActivityDurationsSorted, ActivityBeginningsGivenDuration>;

/// Keeps track of which activity possible beginnings are out of date
/// and handles the computation.
///
/// This class is supposed to run on a single thread.
#[derive(Debug)]
pub struct PossibleBeginningsUpdater {
    possible_beginnings_up_to_date: HashMap<ActivityID, bool>,
    // Cache is mutable hence Cell
    computation_cache: Arc<Mutex<WorkHoursAndActivityDurationsSortedCache>>,
    thread_pool: Rc<rayon::ThreadPool>,
}

impl PossibleBeginningsUpdater {
    pub fn new(thread_pool: Rc<rayon::ThreadPool>) -> PossibleBeginningsUpdater {
        PossibleBeginningsUpdater {
            possible_beginnings_up_to_date: HashMap::new(),
            computation_cache: Arc::new(
                Mutex::new(WorkHoursAndActivityDurationsSortedCache::new()),
            ),
            thread_pool,
        }
    }

    /// Returns true if the activity possible beginnings are up to date.
    #[must_use]
    pub fn activity_beginnings_are_up_to_date(&self, id: &ActivityID) -> bool {
        *self
            .possible_beginnings_up_to_date
            .get(id)
            .expect("Querying invalid activity ID !")
    }

    /// For the given work hours and activity durations, computes the possible activity beginnings.
    /// Invalidates the concerned activities.
    pub fn queue_work_hours_and_activity_durations(
        &mut self,
        work_hours_and_activity_durations: WorkHoursAndActivityDurationsSorted,
        out_of_date_activities: &[ActivityID],
    ) {
        for &id in out_of_date_activities {
            self.possible_beginnings_up_to_date.insert(id, false);
        }

        if self
            .computation_cache
            .lock()
            .unwrap()
            .contains_key(&work_hours_and_activity_durations)
            == false
        {
            let computation_cache = &self.computation_cache;

            // Launch the computation in a separate thread
            self.thread_pool.install(|| {
                let result = new_activity_beginnings_given_duration(find_possible_beginnings(
                    &work_hours_and_activity_durations.work_hours_in_minutes(),
                    &work_hours_and_activity_durations.activity_durations_in_minutes(),
                    MIN_TIME_DISCRETIZATION_MINUTES.into(),
                ));

                computation_cache
                    .lock()
                    .unwrap()
                    .insert(work_hours_and_activity_durations.clone(), result);
            });
        }
    }

    /// Fuses the possible beginnings given every work\_hour\_and\_activity\_duration key.
    /// If the data is not yet available, returns None.
    #[must_use]
    pub fn poll_and_fuse_possible_beginnings(
        &mut self,
        work_hours_and_activity_durations: &[WorkHoursAndActivityDurationsSorted],
        activity: &Activity,
    ) -> Option<HashSet<Time>> {
        let computation_cache = self.computation_cache.lock().unwrap();
        let activity_duration = activity.duration();

        // Fetch possible beginnings
        let maybe_all_possible_beginnings: Option<Vec<_>> = work_hours_and_activity_durations
            .iter()
            .map(|key| {
                if let Some(beginnings_given_duration) = computation_cache.get(key) {
                    // Computation result is there.
                    // Fetch only the possible beginnings for the specified duration.
                    Some(
                        beginnings_given_duration
                            .get(&activity_duration)
                            .expect("Mismatch between computed beginnings and activity duration"),
                    )
                } else {
                    // Computation result is missing
                    None
                }
            })
            .collect();

        // Intersect all possible beginnings
        if let Some(mut all_possible_beginnings) = maybe_all_possible_beginnings {
            // Sort sets by ascending size so that fewer checks are done for intersections
            all_possible_beginnings.sort_by(|a, b| a.len().cmp(&b.len()));

            let first_set = all_possible_beginnings[0];
            let mut others = (&all_possible_beginnings[1..]).iter();
            let intersection = first_set.iter().filter(|k| others.all(|s| s.contains(k)));
            Some(intersection.copied().collect())
        } else {
            // At least one computation result was missing
            None
        }
    }
}
