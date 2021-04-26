use felix_backend::data::{Data, Time, TimeInterval};

use crate::{Activity, Group};

pub struct DataBuilder {
    data: Data,
}

impl DataBuilder {
    #[must_use]
    pub fn new() -> DataBuilder {
        DataBuilder { data: Data::new() }
    }

    #[must_use]
    pub fn with_entity<S>(mut self, entity: S) -> DataBuilder
    where
        S: Into<String>,
    {
        self.data.add_entity(entity).expect("Could not add entity");
        self
    }

    #[must_use]
    pub fn with_entities(mut self, entities: Vec<&'static str>) -> DataBuilder {
        for entity in entities {
            self = self.with_entity(entity);
        }
        self
    }

    #[must_use]
    pub fn with_custom_work_interval_for<S>(
        mut self,
        entity: S,
        interval: TimeInterval,
    ) -> DataBuilder
    where
        S: Into<String>,
    {
        self.data
            .add_custom_work_interval_for(entity, interval)
            .expect("Could not add custom work interval");
        self
    }

    #[must_use]
    pub fn with_custom_work_intervals_for<S>(
        mut self,
        entity: S,
        intervals: Vec<TimeInterval>,
    ) -> DataBuilder
    where
        S: Into<String> + Clone,
    {
        for interval in intervals {
            self = self.with_custom_work_interval_for(entity.clone(), interval);
        }
        self
    }

    #[must_use]
    pub fn with_group(mut self, group: Group) -> DataBuilder {
        let group_name = self
            .data
            .add_group(group.name)
            .expect("Could not add group");
        for entity in group.entities {
            self.data
                .add_entity_to_group(&group_name, entity)
                .expect("Could not add entity to group");
        }
        self
    }

    #[must_use]
    pub fn with_groups(mut self, groups: Vec<Group>) -> DataBuilder {
        for group in groups {
            self = self.with_group(group);
        }
        self
    }

    #[must_use]
    pub fn with_work_interval(mut self, work_interval: TimeInterval) -> DataBuilder {
        self.data
            .add_work_interval(work_interval)
            .expect("Could not add work interval");
        self
    }

    #[must_use]
    pub fn with_work_intervals(mut self, work_intervals: Vec<TimeInterval>) -> DataBuilder {
        for work_interval in work_intervals {
            self = self.with_work_interval(work_interval);
        }
        self
    }

    /// Convenience functions which adds an interval from [00:00 to hours:00].
    #[must_use]
    pub fn with_work_interval_of_duration(self, hours: i8) -> DataBuilder {
        let interval = TimeInterval::new(Time::new(0, 0), Time::new(hours, 0));
        self.with_work_interval(interval)
    }

    #[must_use]
    pub fn with_activity(mut self, activity: Activity) -> DataBuilder {
        let id = self
            .data
            .add_activity(activity.name)
            .expect("Could not add activity")
            .id();

        for entity in activity.entities {
            self.data
                .add_entity_to_activity(id, entity)
                .expect("Could not add entity to activity");
        }

        for group in activity.groups {
            self.data
                .add_group_to_activity(id, group)
                .expect("Could not add group to activity");
        }
        println!("BEFORE SET DURATION");
        self.data
            .set_activity_duration(id, activity.duration)
            .expect("Could not set activity duration");

        println!("BEFORE WAIT");
        while self
            .data
            .possible_insertion_times_of_activity_with_associated_cost(id)
            .is_none()
        {
            // Wait for possible insertion times to be asynchronously calculated
        }
        println!("BEFORE INSERT");
        self.data
            .insert_activity(id, activity.insertion_time)
            .expect("Could not insert activity");
        self
    }

    #[must_use]
    pub fn with_activities(mut self, activities: Vec<Activity>) -> DataBuilder {
        for activity in activities {
            self = self.with_activity(activity);
        }
        self
    }

    /// Consumes the data builder and returns the built Data object.
    #[must_use]
    pub fn into_data(self) -> Data {
        self.data
    }
}

impl Default for DataBuilder {
    fn default() -> Self {
        Self::new()
    }
}
