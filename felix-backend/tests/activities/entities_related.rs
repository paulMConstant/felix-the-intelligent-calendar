//! Operations on activities which depend on entities.
//! Checks interactions with entities.
//!
//! Includes:
//! - Addition of entities to the activity
//! - Deletion of entities from the activity
//! - Changing the duration of the activity (makes sure all entities have enough time)

use felix_backend::data::Time;
use test_utils::data_builder::{Activity, DataBuilder};

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
            let entities = data
                .activity(id)
                .expect("Could not get activity by id")
                .entities_sorted();
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
            let entities = data
                .activity(id)
                .expect("Could not get activity by id")
                .entities_sorted();
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
    test_err!(
        data,
        DataBuilder::new()
            .with_work_interval_of_duration(4)
            .with_entity(entity_name)
            .with_activity(Activity {
                name: activity_name,
                entities: vec![entity_name],
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
    test_err!(
        data,
        DataBuilder::new()
            .with_work_interval_of_duration(4)
            .with_entity(entity_name)
            .with_activity(Activity::default()),
        data.add_entity_to_activity(4, entity_name),
        "The activity with id '4' does not exist.",
        "Could add entity to activity with wrong id"
    );
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
            let entities = data
                .activity(id)
                .expect("Could not get activity by id")
                .entities_sorted();

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
    test_err!(
        data,
        DataBuilder::new()
            .with_activity(Activity::default())
            .with_entity(entity_name),
        data.remove_entity_from_activity(15, entity_name),
        "The activity with id '15' does not exist.",
        "Could remove entity from nonexsistent activity"
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
