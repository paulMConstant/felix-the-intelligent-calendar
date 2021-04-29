//! Operations on activities which depend on entities.
//! Checks interactions with entities.
//!
//! Includes:
//! - Addition of entities to the activity
//! - Deletion of entities from the activity
//! - Changing the duration of the activity (makes sure all entities have enough time)

use felix_backend::data::{Time, TimeInterval};
use test_utils::{Activity, DataBuilder};

// *** Add entities ***
#[test]
fn simple_add_entity() {
    let name = "Entity";
    test_ok!(
        data,
        DataBuilder::new()
            .with_work_interval_of_duration(4)
            .with_activity(Activity::default())
            .with_entity(name),
        {
            let id = data.activities_sorted()[0].id();
            data.add_entity_to_activity(id, name)
                .expect("Could not add entity to activity");
            let entities = data.activity(id).entities_sorted();
            assert_eq!(entities.len(), 1, "Participant was not added");
            assert_eq!(entities[0], name, "Participant was added with wrong name");
        }
    );
}

#[test]
fn add_entities_check_sorting() {
    let (name1, name2, name3) = ("Name1", "Name2", "Name3");
    test_ok!(
        data,
        DataBuilder::new()
            .with_work_interval_of_duration(4)
            .with_entities(vec![name1, name3, name2])
            .with_activity(Activity {
                entities: vec![name1, name3, name2],
                ..Default::default()
            }),
        {
            let id = data.activities_sorted()[0].id();
            let entities = data.activity(id).entities_sorted();
            assert_eq!(entities.len(), 3, "Participants were not added");
            assert_eq!(entities[0], name1, "Participants are not sorted");
            assert_eq!(entities[1], name2, "Participants are not sorted");
            assert_eq!(entities[2], name3, "Participants are not sorted");
        }
    );
}

#[test]
fn add_entity_not_enough_time() {
    let name = "Entity";
    test_err!(
        data,
        DataBuilder::new()
            .with_activity(Activity::default())
            .with_entity(name),
        {
            let id = data.activities_sorted()[0].id();
            data.add_entity_to_activity(id, name)
        },
        "Entity will not have enough time if they are added to 'Activity'.",
        "Could add entity with not enough time"
    );
}

#[test]
fn add_entity_already_participating() {
    let (entity_name, activity_name) = ("Entity", "Activity");
    // Add an insertion time to make sure that the activity is not detected as overlapping
    let beginning = Time::new(8, 0);
    let end = Time::new(10, 0);
    let time_interval = TimeInterval::new(beginning, end);
    test_err!(
        data,
        DataBuilder::new()
            .with_work_interval(time_interval)
            .with_entity(entity_name)
            .with_activity(Activity {
                name: activity_name,
                entities: vec![entity_name],
                insertion_time: Some(beginning),
                ..Default::default()
            }),
        {
            let id = data.activities_sorted()[0].id();
            data.add_entity_to_activity(id, entity_name)
        },
        "Entity is already in the activity 'Activity'.",
        "Could add the same entity twice"
    );
}

#[test]
fn add_entity_wrong_id() {
    let entity_name = "Entity";
    std::panic::catch_unwind(|| {
        let mut data = DataBuilder::new()
            .with_work_interval_of_duration(4)
            .with_entity(entity_name)
            .with_activity(Activity::default())
            .into_data();
        data.add_entity_to_activity(4, entity_name).unwrap();
    })
    .expect_err("Could add entity to activity with wrong id");
}

#[test]
fn add_entity_does_not_exist() {
    test_err!(
        data,
        DataBuilder::new()
            .with_work_interval_of_duration(4)
            .with_activity(Activity::default()),
        {
            let id = data.activities_sorted()[0].id();
            data.add_entity_to_activity(id, "Does not exist")
        },
        "Does Not Exist does not exist.",
        "Could add nonexistent entity to activity"
    );
}

#[test]
fn add_entity_to_inserted_activity_invalid_spot() {
    let (entity1, entity2) = ("Entity1", "Entity2");
    let (activity1, activity2) = ("Activity1", "Activity2");
    let beginning = Time::new(9, 0);
    let end = Time::new(13, 0);
    let time_interval = TimeInterval::new(beginning, end);

    test_err!(
        data,
        DataBuilder::new()
            .with_work_interval(time_interval)
            .with_entities(vec![entity1, entity2])
            .with_activities(vec![
                Activity {
                    name: activity1,
                    entities: vec![entity1],
                    duration: Time::new(1, 0),
                    groups: Vec::new(),
                    insertion_time: Some(beginning),
                },
                Activity {
                    name: activity2,
                    entities: vec![entity2],
                    duration: Time::new(1, 0),
                    groups: Vec::new(),
                    insertion_time: Some(beginning),
                }
            ]),
        {
            let id1 = data.activities_sorted()[0].id();

            // Try to add entity2 to activity1 which is inserted in the same spot as activity2
            data.add_entity_to_activity(id1, entity2)
        },
        "Entity2 cannot be added to 'Activity1' because it would overlap with 'Activity2'.",
        "Could add entity to activity which is inserted in an unavailable spot"
    );
}

#[test]
fn add_entity_to_inserted_activity_spot_not_in_work_hours() {
    let (entity1, entity2) = ("Entity1", "Entity2");
    let activity1 = "Activity1";
    let beginning1 = Time::new(9, 0);
    let end1 = Time::new(13, 0);
    let time_interval1 = TimeInterval::new(beginning1, end1);

    let beginning2 = Time::new(11, 0);
    let end2 = Time::new(13, 0);
    let time_interval2 = TimeInterval::new(beginning2, end2);

    test_err!(
        data,
        DataBuilder::new()
            .with_work_interval(time_interval1)
            .with_entities(vec![entity1, entity2])
            .with_custom_work_interval_for(entity2, time_interval2)
            .with_activity(Activity {
                name: activity1,
                entities: vec![entity1],
                duration: Time::new(1, 0),
                groups: Vec::new(),
                insertion_time: Some(beginning1),
            },),
        {
            let id1 = data.activities_sorted()[0].id();

            // Try to add entity2 to activity1 which is inserted in the same spot as activity2
            data.add_entity_to_activity(id1, entity2)
        },
        "Entity2 cannot be added to 'Activity1' because it would be outside of their work hours.",
        "Could add entity to activity which is inserted in an unavailable spot"
    );
}

// *** Remove entities ***
#[test]
fn simple_remove_entity() {
    let (entity1, entity2) = ("Entity1", "Entity2");
    test_ok!(
        data,
        DataBuilder::new()
            .with_work_interval_of_duration(4)
            .with_entities(vec![entity1, entity2])
            .with_activity(Activity {
                entities: vec![entity1, entity2],
                ..Default::default()
            }),
        {
            let id = data.activities_sorted()[0].id();
            data.remove_entity_from_activity(id, entity1)
                .expect("Could not remove entity");
            let entities = data.activity(id).entities_sorted();

            assert_eq!(entities.len(), 1, "Participant was not removed");
            assert_eq!(entities[0], entity2, "The wrong entity was removed");
        }
    );
}

#[test]
fn remove_entity_does_not_exist() {
    test_err!(
        data,
        DataBuilder::new().with_activity(Activity::default()),
        {
            let id = data.activities_sorted()[0].id();
            data.remove_entity_from_activity(id, "Does not exist")
        },
        "Does Not Exist does not exist.",
        "Could remove nonexistent entity"
    );
}

#[test]
fn remove_entity_not_participating() {
    let entity_name = "Entity";
    let activity_name = "Activity";
    test_err!(
        data,
        DataBuilder::new()
            .with_activity(Activity {
                name: activity_name,
                ..Default::default()
            })
            .with_entity(entity_name),
        {
            let id = data.activities_sorted()[0].id();
            data.remove_entity_from_activity(id, entity_name)
        },
        "Entity is not in the activity 'Activity'.",
        "Could remove entity not taking part in the activity"
    );
}

#[test]
fn remove_entity_wrong_activity_id() {
    let entity_name = "Entity";
    std::panic::catch_unwind(|| {
        let mut data = DataBuilder::new()
            .with_activity(Activity::default())
            .with_entity(entity_name)
            .into_data();
        data.remove_entity_from_activity(15, entity_name).unwrap();
    })
    .expect_err("Could remove entity from nonexsistent activity");
}

#[test]
fn remove_last_entity_check_activity_uninserted() {
    let (entity1, entity2) = ("Entity1", "Entity2");
    let (beginning, end) = (Time::new(8, 0), Time::new(12, 0));
    let duration = Time::new(1, 0);
    test_ok!(
        data,
        DataBuilder::new()
            .with_work_interval(TimeInterval::new(beginning, end))
            .with_entities(vec![entity1, entity2])
            .with_activity(Activity {
                entities: vec![entity1, entity2],
                insertion_time: Some(beginning),
                duration,
                ..Default::default()
            }),
        {
            let id = data.activities_sorted()[0].id();
            data.remove_entity_from_activity(id, entity1)
                .expect("Could not remove entity");

            // Check that the activity is still inserted
            let activity = data.activity(id);
            let expected_insertion_interval =
                TimeInterval::new(beginning, beginning + activity.duration());
            assert_eq!(
                activity.insertion_interval(),
                Some(expected_insertion_interval),
                "Activity was uninserted even though it still has one participant"
            );

            // Remove the last entity from the activity and check that the activity was uninserted
            data.remove_entity_from_activity(id, entity2)
                .expect("Could not remove entity");
            let activity = data.activity(id);
            assert_eq!(
                activity.insertion_interval(),
                None,
                "Activity is still inserted even though it has no participant anymore"
            );
        }
    );
}

// *** Set duration ***
#[test]
fn set_activity_duration_not_enough_free_time() {
    let entity_name = "Entity";
    test_err!(
        data,
        DataBuilder::new()
            .with_entity(entity_name)
            .with_work_interval_of_duration(1)
            .with_activity(Activity {
                duration: Time::new(1, 0),
                entities: vec![entity_name],
                ..Default::default()
            }),
        {
            let id = data.activities_sorted()[0].id();

            data.set_activity_duration(id, Time::new(2, 0))
        },
        "Entity will not have enough time if the duration of 'Activity' is increased.",
        "Could set duration where an entity has not enough free time"
    );
}
