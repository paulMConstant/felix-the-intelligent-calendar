use crate::data::{
    computation_structs::work_hours_and_activity_durations_sorted::WorkHoursAndActivityDurationsSorted,
    Activity, ActivityID, Data,
};
use crate::errors::Result;

use std::collections::HashSet;

/// Helper functions to trigger & update activity insertion computation
impl Data {
    /// Starts the computation of the possible beginnings of activities of entities whose work
    /// hours were modified.
    #[must_use]
    pub(crate) fn queue_entities_on_global_work_hour_change(&mut self) -> Result<()> {
        let entities_to_queue = self
            .entities_sorted()
            .iter()
            .filter(|entity| entity.custom_work_hours().is_empty())
            .map(|entity| entity.name())
            .collect::<Vec<_>>();
        self.queue_entities(entities_to_queue)
    }

    /// Starts the computation of the possible beginnings of the given activity.
    #[must_use]
    pub(crate) fn queue_activity_participants(&mut self, activity: &Activity) -> Result<()> {
        self.queue_entities(activity.entities_sorted())
    }

    /// Starts the computation of the possible beginnings of the activities of the given entities.
    #[must_use]
    pub(crate) fn queue_entities(&mut self, entities: Vec<String>) -> Result<()> {
        let work_hours_and_activity_durations =
            self.work_hours_and_activity_durations_from_entities(&entities)?;
        let activities_to_invalidate = self.activity_ids_of_entities(&entities)?;
        self.activities.must_update_possible_activity_beginnings(
            work_hours_and_activity_durations,
            activities_to_invalidate,
        );
        Ok(())
    }

    /// Given a vector of entities, outputs their work hours and activity durations.
    #[must_use]
    pub(crate) fn work_hours_and_activity_durations_from_entities(
        &self,
        entities: &[String],
    ) -> Result<Vec<WorkHoursAndActivityDurationsSorted>> {
        entities
            .iter()
            .map(|entity| {
                let work_hours = self.work_hours_of(entity)?;
                let activity_durations = self
                    .activities_of(entity)?
                    .iter()
                    .map(|activity| activity.duration())
                    .collect::<Vec<_>>();
                Ok(WorkHoursAndActivityDurationsSorted::new(
                    work_hours,
                    activity_durations,
                ))
            })
            .collect()
    }

    /// Given a vector of entities, outputs the ids of all their activities.
    #[must_use]
    fn activity_ids_of_entities(&self, entities: &[String]) -> Result<HashSet<ActivityID>> {
        let activities_of_entities = entities
            .iter()
            .map(|entity| self.activities_of(entity))
            .collect::<Result<Vec<_>>>()?;

        Ok(activities_of_entities
            .iter()
            .flat_map(|activities| activities.iter().map(|activity| activity.id()))
            .collect())
    }
}
