mod error_checks;
mod inner;
mod queue_for_computation;

use super::helpers::clean_string;

use crate::{
    components::activity::{
        activities_into_computation_data, activities_sorted_filtered_for_computation,
    },
    errors::{invalid_insertion::InvalidInsertion, Result},
    Activity, ActivityBeginningMinutes, ActivityId, Data, Rgba, Time,
};

use felix_computation_api::{autoinsert, structs::AutoinsertionThreadHandle};

/// Operations on activities.
impl Data {
    /// Returns the activities, sorted by name.
    #[must_use]
    pub fn activities_sorted(&self) -> Vec<Activity> {
        self.activities.get_sorted_by_name()
    }

    /// Returns the activities, not sorted.
    #[must_use]
    pub fn activities_not_sorted(&self) -> Vec<Activity> {
        self.activities.get_not_sorted()
    }

    /// Returns a copy of the activity with given id.
    ///
    /// # Panics
    ///
    /// Panics if the activity is not found.
    pub fn activity(&self, id: ActivityId) -> Activity {
        self.activities.get_by_id(id)
    }

    /// Returns the activities in which the given entity participates.
    ///
    /// # Errors
    ///
    /// Returns err if the entity name is empty after sanitization.
    pub fn activities_of<S>(&self, entity_name: S) -> Result<Vec<Activity>>
    where
        S: Into<String>,
    {
        let entity_name = clean_string(entity_name)?;
        Ok(self
            .activities_not_sorted()
            .into_iter()
            .filter(|activity| activity.entities_sorted().contains(&entity_name))
            .collect())
    }

    /// Waits until the insertion costs of an activity have been computed.
    pub fn wait_for_possible_insertion_costs_computation(&self, id: ActivityId) {
        while self.activity(id).insertion_costs().is_none() {
            // Active wait
        }
    }

    /// Adds an activity with the formatted given name.
    ///
    /// Automatically assigns a unique id.
    /// Returns a copy of the created activity.
    ///
    /// # Errors
    ///
    /// Returns Err if the formatted name is empty.
    pub fn add_activity<S>(&mut self, name: S) -> Result<Activity>
    where
        S: Into<String>,
    {
        let activity_id = self.activities.add(clean_string(name)?).id();
        let activity = self.activity(activity_id);
        self.events()
            .borrow_mut()
            .emit_activity_added(self, &activity);
        // No update of possible beginnings necessary
        Ok(activity)
    }

    /// Removes the activity with the given id.
    /// Returns the position of the removed activity in the array sorted by name.
    ///
    /// # Panics
    ///
    /// Panics if the activity with given ID does not exist.
    pub fn remove_activity(&mut self, id: ActivityId) {
        let activities = self.activities_sorted();
        let position_of_removed_activity = activities
            .into_iter()
            .position(|activity| activity.id() == id)
            .expect("The activity with given id does not exist");

        let impacted_entities = self.activity(id).entities_sorted();
        self.activities.remove(id);

        self.queue_entities(impacted_entities);

        self.events()
            .borrow_mut()
            .emit_activity_removed(self, position_of_removed_activity);
    }

    /// Adds the entity with given name to the activity with given id.
    ///
    /// # Errors
    ///
    /// Returns Err if the entity is not found,
    /// if the entity does not have enough time left
    /// or the entity is already taking part in the activity.
    ///
    /// # Panics
    ///
    /// Panics if the activity with given ID does not exist.
    pub fn add_entity_to_activity<S>(&mut self, id: ActivityId, entity_name: S) -> Result<()>
    where
        S: Into<String>,
    {
        let entity_name = clean_string(entity_name)?;
        self.check_has_enough_time_for_activity(id, &entity_name)?;
        self.check_no_activity_of_the_entity_is_overlapping(id, &entity_name)?;
        self.check_activity_inside_of_work_hours(id, &entity_name)?;

        self.activities.add_entity(id, entity_name.clone())?;
        self.queue_entities(vec![entity_name]);

        self.events()
            .borrow_mut()
            .emit_entity_added_to_activity(self, &self.activity(id));
        Ok(())
    }

    /// Removes the entity with given name from the activity with given id.
    ///
    /// # Errors
    ///
    /// Returns Err if the entity is not found,
    /// if the name is empty
    /// or the entity is not taking part in the activity.
    pub fn remove_entity_from_activity<S>(&mut self, id: ActivityId, entity_name: S) -> Result<()>
    where
        S: Into<String>,
    {
        // Check that the entity exists and get it formatted
        let entity_name = self.entity(entity_name)?.name();
        // Remove the entity from the activity
        self.activities.remove_entity(id, &entity_name)?;

        // Queue the entity which was just removed
        // TODO add test here: entity schedule changes when entity removed from one activity
        // After adding tests, comment self.queue_entities line and see if tests fail
        self.queue_entities(vec![entity_name]);

        if self.activity(id).can_be_inserted() {
            // Queue the activity because it has one less participant
            self.queue_activity_participants(self.activity(id));
        } else {
            // Remove activity from schedule because it cannot be inserted anymore
            self.insert_activity(id, None)?;
        }

        self.events()
            .borrow_mut()
            .emit_entity_removed_from_activity(self, &self.activity(id));
        Ok(())
    }

    /// Adds the group with the formatted given name to the activity with the given id.
    ///
    /// Any activity currently in the group will be added to the activity.
    ///
    /// # Panics
    ///
    /// Panics if the activity with given ID does not exist.
    ///
    /// # Errors
    ///
    /// Returns Err if the group is not found,
    /// if the name is empty
    /// or if the group is already taking part in the activity.
    pub fn add_group_to_activity<S>(&mut self, id: ActivityId, group_name: S) -> Result<()>
    where
        S: Into<String>,
    {
        // Check that the group exists and get name formatted
        let group = self.group(group_name)?;
        // Fetch group and entities here as copies (dropping group reference for borrow checker)
        let entities = group.entities_sorted();
        let group_name = group.name();

        for entity_name in entities.iter() {
            self.check_has_enough_time_for_activity(id, entity_name)?;
        }

        // Add each entity in the group to the activity.
        // We do not care about the result: if the entity is already in the activity, it is fine.
        for entity_name in entities {
            let _ = self.add_entity_to_activity(id, entity_name);
        }

        // Add the group to the activity
        self.activities.add_group(id, clean_string(group_name)?)?;

        if self.activity(id).can_be_inserted() {
            self.queue_activity_participants(self.activity(id));
        }

        self.events()
            .borrow_mut()
            .emit_group_added_to_activity(self, &self.activity(id));
        Ok(())
    }

    /// Removes the group with the formatted given name from the activity with the given id.
    ///
    /// The group will be removed from the activities.
    /// Any entity participating in activities only through this group will be removed from the
    /// activities.
    ///
    /// # Errors
    ///
    /// Returns Err if the group is not found,
    /// if the name is empty
    /// or if the group is not taking part in the activity.
    ///
    /// # Panics
    ///
    /// Panics if the activity is not found.
    pub fn remove_group_from_activity<S>(&mut self, id: ActivityId, group_name: S) -> Result<()>
    where
        S: Into<String>,
    {
        // Check that the group exists and get name formatted
        let group_name = self.group(group_name)?.name();

        self.activities.remove_group(id, &group_name)?;

        let entities_to_remove =
            self.entities_participating_through_this_group_only(id, &group_name)?;

        for entity_name in &entities_to_remove {
            // The entity may already be out of the activity if excluded from group.
            // Therefore, don't check for errors.
            let _ = self.remove_entity_from_activity(id, entity_name);
        }

        if self.activity(id).can_be_inserted() {
            self.queue_activity_participants(self.activity(id));
        }

        self.events()
            .borrow_mut()
            .emit_group_removed_from_activity(self, &self.activity(id));
        Ok(())
    }

    /// Sets the name of the activity with given id with the formatted given name.
    ///
    /// Returns the formatted version of the given name.
    ///
    /// # Errors
    ///
    /// Returns Err if the formatted name is empty.
    ///
    /// # Panics
    ///
    /// Panics if the activity does not exist.
    pub fn set_activity_name<S>(&mut self, id: ActivityId, name: S) -> Result<String>
    where
        S: Into<String>,
    {
        let name = clean_string(name)?;
        self.activities.set_name(id, name.clone());
        self.events()
            .borrow_mut()
            .emit_activity_renamed(self, &self.activity(id));
        Ok(name)
    }

    /// Sets the duration of the activity with given id.
    ///
    /// # Panics
    ///
    /// Panics if the activity with given ID does not exist.
    ///
    /// # Errors
    ///
    /// Returns Err if an entity does not have enough time left.
    pub fn set_activity_duration(&mut self, id: ActivityId, new_duration: Time) -> Result<()> {
        // If the duration is longer than the previous one, check for conflicts
        let activity = self.activity(id);

        if new_duration > activity.duration() {
            self.check_entity_without_enough_time_to_set_duration(id, new_duration)?;
            // Remove the activity from the schedule if its duration is greater.
            // Because we may not be sure that it will fit there again, we have to perform the
            // computation in another thread before we can insert it again.
            if activity.insertion_interval().is_some() {
                // Remember that the activity was inserted because we will remove it from the
                // schedule.
                // Once we compute its possible beginnings, we will be able to put it back in the
                // schedule.
                self.activities.store_activity_was_inserted(id);
                self.insert_activity(id, None)
                    .expect("Could not remove activity from schedule. This is a bug.");
            }
        } else if new_duration == Time::new(0, 0) && activity.insertion_interval().is_some() {
            // Activity with empty duration cannot be inserted
            self.insert_activity(id, None)
                .expect("Could not remove activity from schedule. This is a bug.");
        }

        self.activities.set_duration(id, new_duration);

        // Don't queue activity with no duration or participants
        if self.activity(id).can_be_inserted() {
            self.queue_activity_participants(self.activity(id));
        }

        self.events()
            .borrow_mut()
            .emit_activity_duration_changed(self, &self.activity(id));

        Ok(())
    }

    /// Sets the color of the activity with given id.
    ///
    /// # Panics
    ///
    /// Panics if the activity is not found.
    pub fn set_activity_color(&mut self, id: ActivityId, color: Rgba) -> Result<()> {
        self.activities.set_color(id, color);
        let activity = self.activity(id);
        self.events()
            .borrow_mut()
            .emit_activity_color_changed(self, &activity);
        Ok(())
    }

    /// Tries to insert the activity with given id with the given beginning.
    /// If None is given, the activity is removed from the schedule.
    ///
    /// # Errors
    ///
    /// Returns Err if the insertion time is not available.
    ///
    /// # Panics
    ///
    /// Panics if the activity does not exist.
    pub fn insert_activity(&mut self, id: ActivityId, insertion_time: Option<Time>) -> Result<()> {
        if let Some(insertion_time) = insertion_time {
            // We want to insert the activity
            if let Some(possible_insertion_costs) = self.activity(id).insertion_costs() {
                if possible_insertion_costs
                    .iter()
                    .any(|insertion_cost| insertion_cost.beginning == insertion_time)
                {
                    self.activities.insert_activity(id, Some(insertion_time));
                    self.events()
                        .borrow_mut()
                        .emit_activity_inserted(self, &self.activity(id));
                    self.queue_activity_participants(self.activity(id));
                    Ok(())
                } else {
                    // We cannot insert the activity - find out why
                    let activity = self.activity(id);
                    if let Some(blocking_activity) =
                        self.incompatible_activity_inserted_at_time(&activity, insertion_time)
                    {
                        Err(InvalidInsertion::would_overlap_with_activity(
                            activity.name(),
                            insertion_time,
                            blocking_activity.name(),
                        ))
                    } else {
                        Err(
                            InvalidInsertion::cannot_fit_or_would_block_other_activities(
                                activity.name(),
                                insertion_time,
                            ),
                        )
                    }
                }
            } else {
                // Computation is not finished
                Err(InvalidInsertion::insertions_not_computed_yet(
                    self.activity(id).name(),
                ))
            }
        } else {
            // TODO split function
            // Remove activity from schedule
            self.activities.insert_activity(id, None);
            self.events()
                .borrow_mut()
                .emit_activity_inserted(self, &self.activity(id));

            // TODO remove condition and always queue (check done it queue)
            // If the activity has no entities or no duration, it is useless to queue it.
            // This also makes sure that its insertion costs are not invalidated here.
            if self.activity(id).can_be_inserted() {
                self.queue_activity_participants(self.activity(id));
            }
            Ok(())
        }
    }

    /// If activities were removed from the schedule because their duration was increased, insert
    /// them back into the schedule in the closest spot we find.
    pub fn insert_activities_removed_because_duration_increased_in_closest_spot(&mut self) {
        let activity_ids_and_old_beginnings = self
            .activities
            .get_activities_removed_because_duration_increased();

        for (id, old_beginning) in activity_ids_and_old_beginnings {
            if let Some(possible_insertion_times) = self.activity(id).insertion_costs() {
                // Possible insertion times have been computed
                if let Some(closest_spot) = self.activities.get_closest_spot_to_insert_activity(
                    id,
                    old_beginning,
                    possible_insertion_times,
                ) {
                    // Activity can be inserted
                    self.insert_activity(id, Some(closest_spot))
                        .expect("Activity could not be inserted, but we computed that it could");
                }
            }
        }
    }

    /// Starts autoinsertion in a separate thread and returns a mpsc::receiver handle for the
    /// result.
    ///
    /// # Errors
    ///
    /// Returns Err if the insertions have not been computed yet.
    pub fn start_autoinsertion(&mut self) -> Result<AutoinsertionThreadHandle> {
        // Poll insertion data
        let activities = activities_sorted_filtered_for_computation(&self.activities_not_sorted());

        if let Some(activity_not_computed_yet) = activities
            .iter()
            .find(|activity| activity.insertion_costs().is_none())
        {
            Err(InvalidInsertion::insertions_not_computed_yet(
                activity_not_computed_yet.name(),
            ))
        } else {
            let (static_data, insertion_data) = activities_into_computation_data(&activities);

            Ok(autoinsert(&static_data, &insertion_data))
        }
    }

    /// Applies the result of autoinsertion to the activities.
    pub fn apply_autoinsertion_result(&mut self, insertion_data: Vec<ActivityBeginningMinutes>) {
        self.activities.overwrite_insertion_data(insertion_data);
        self.events().borrow_mut().emit_autoinsertion_done(self);
    }

    /// Clears the list of activities which were removed because their duration increased.
    /// This means that we give up on reinserting them automatically (for instance, it's been too
    /// long since the activity was removed).
    pub fn clear_list_activities_removed_because_duration_increased(&mut self) {
        self.activities
            .clear_activities_removed_because_duration_increased();
    }

    /// If activities were removed because their duration increased, returns true.
    #[must_use]
    pub fn activities_were_uninserted_and_can_maybe_be_inserted_back(&self) -> bool {
        !self
            .activities
            .get_activities_removed_because_duration_increased()
            .is_empty()
    }
}
