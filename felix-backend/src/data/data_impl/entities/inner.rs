//! Helper functions for entities implementation of data.

use crate::data::{Data, Time};
use crate::errors::Result;

impl Data {
    /// Returns the time taken by the activities of an entity.
    ///
    /// If the entity does not exist, returns Time(0, 0).
    #[must_use]
    pub(super) fn time_taken_by_activities(&self, entity_name: &str) -> Time {
        self.activities_sorted()
            .iter()
            .filter_map(|activity| {
                if activity.entities_sorted().contains(&entity_name.into()) {
                    Some(activity.duration())
                } else {
                    None
                }
            })
            .sum()
    }

    /// Returns the total time available for an entity.
    ///
    /// # Errors
    ///
    /// Returns Err if the entity does not exist.
    pub(super) fn total_available_time(&self, entity_name: &str) -> Result<Time> {
        Ok(self
            .work_hours_of(entity_name)?
            .iter()
            .map(|interval| interval.duration())
            .sum())
    }
}
