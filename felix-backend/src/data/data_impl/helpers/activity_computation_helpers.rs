use crate::data::{
    computation_structs::WorkHoursAndActivityDurationsSorted, Activity, ActivityId, Data,
};
use crate::errors::Result;

use std::collections::HashSet;

/// Helper functions to trigger & update activity insertion computation
impl Data {
    /// Queues up every entity to compute the possible beginnings of their entities.
    /// Must be called on startup if data is not created from stratch (i.e. instantiated without
    /// new(), with serde for example).
    pub fn queue_every_activity_for_beginning_computation(&mut self) {
        let entity_names = self
            .entities_sorted()
            .iter()
            .map(|entity| entity.name())
            .collect::<Vec<_>>();

        self.queue_entities(entity_names)
            .expect("Could not queue existing entities for computation");
    }

    /// Starts the computation of the possible beginnings of activities of entities whose work
    /// hours were modified.
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
    pub(crate) fn queue_activity_participants(&mut self, activity: &Activity) -> Result<()> {
        self.queue_entities(activity.entities_sorted())
    }

    /// Starts the computation of the possible beginnings of the activities of the given entities.
    pub(crate) fn queue_entities(&mut self, entities: Vec<String>) -> Result<()> {
        let work_hours_and_activity_durations =
            self.work_hours_and_activity_durations_from_entities(&entities)?;
        let activities_to_invalidate = self.activity_ids_of_entities(&entities)?;
        self.activities.trigger_update_possible_activity_beginnings(
            &work_hours_and_activity_durations,
            activities_to_invalidate,
        );
        Ok(())
    }

    /// Given a vector of entities, outputs their work hours and activity durations.
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
    fn activity_ids_of_entities(&self, entities: &[String]) -> Result<HashSet<ActivityId>> {
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
