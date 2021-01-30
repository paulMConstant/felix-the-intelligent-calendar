use felix_backend::data::{Activity, Data, Time, TimeInterval};

/// Simple struct holding an activity's name and insertion interval.
pub struct ActivityNameAndInsertionTime {
    name: String,
    insertion_interval: Option<TimeInterval>,
}

impl ActivityNameAndInsertionTime {
    #[must_use]
    pub fn new(activity: &Activity) -> ActivityNameAndInsertionTime {
        ActivityNameAndInsertionTime {
            name: activity.name(),
            insertion_interval: activity.insertion_interval(),
        }
    }

    #[must_use]
    pub fn name(&self) -> &String {
        &self.name
    }

    #[must_use]
    pub fn insertion_interval(&self) -> &Option<TimeInterval> {
        &self.insertion_interval
    }
}

/// Simple struct holding an entity's name, its activities and work hours.
pub struct EntityToShow {
    name: String,
    activities: Vec<ActivityNameAndInsertionTime>,
    work_hours: Vec<TimeInterval>,
    free_time: Time,
}

impl EntityToShow {
    #[must_use]
    pub fn new(entity_name: String, data: &Data) -> EntityToShow {
        EntityToShow {
            activities: data
                .activities_of(&entity_name)
                .expect("Every entity to show should exist in Data when created")
                .iter()
                .map(|activity| ActivityNameAndInsertionTime::new(activity))
                .collect(),
            work_hours: data
                .work_hours_of(&entity_name)
                .expect("Every entity to show should exist in Data when created"),
            free_time: data
                .free_time_of(&entity_name)
                .expect("Every entity to show should exist in Data when created"),
            name: entity_name,
        }
    }

    #[must_use]
    pub fn name(&self) -> &String {
        &self.name
    }

    #[must_use]
    pub fn activities(&self) -> &Vec<ActivityNameAndInsertionTime> {
        &self.activities
    }

    #[must_use]
    pub fn work_hours(&self) -> &Vec<TimeInterval> {
        &self.work_hours
    }

    #[must_use]
    pub fn free_time(&self) -> Time {
        self.free_time
    }
}

impl Eq for EntityToShow {}
impl PartialEq for EntityToShow {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
