use crate::data::Data;
use crate::errors::{
    change_work_hours_while_activity_inserted::ChangeWorkHoursWhileActivityInserted, Result,
};
use crate::Time;

impl Data {
    pub(super) fn notify_work_hours_changed(&mut self) {
        self.events().borrow_mut().emit_work_hours_changed(self);
        self.queue_entities_on_global_work_hour_change();
    }

    pub(super) fn check_no_activity_inserted(&self) -> Result<()> {
        if self
            .activities_not_sorted()
            .iter()
            .any(|activity| activity.insertion_interval().is_some())
        {
            Err(ChangeWorkHoursWhileActivityInserted::new())
        } else {
            Ok(())
        }
    }

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
