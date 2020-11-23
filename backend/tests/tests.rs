extern crate backend;
use backend::data::{Data, Time, TimeInterval};

#[test]
fn entities_add_remove_modify() {
    let mut data = Data::new();
    let name = "Emma Carena";
    let name_aa = "Aa";
    let name_zz = "Zz";
    // Add entity
    assert!(data.add_entity(name).is_ok());
    // Add existing entity
    assert!(data.add_entity(name).is_err());
    assert!(data.add_entity("").is_err());

    // Get entity
    assert!(data.entity(name).is_ok());
    // Get non-existent entity
    assert!(data.entity("").is_err());
    assert!(data.entity("Nonexistent").is_err());

    // Add other entities - check sorting
    assert!(data.add_entity(name_aa).is_ok());
    assert!(data.add_entity(name_zz).is_ok());
    let entities = data.entities_sorted();
    assert_eq!(entities[0].name(), name_aa);
    assert_eq!(entities[1].name(), name);
    assert_eq!(entities[2].name(), name_zz);

    // Add entity to activity to check if it is removed later
    // First, add work hours
    let morning_shift = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    data.add_work_interval(morning_shift).unwrap();
    let activity_id = data.add_activity("test activity").unwrap().id();
    assert!(data
        .add_participant_to_activity(activity_id, name_aa.clone())
        .is_ok());

    // Remove entity
    assert!(data.remove_entity("Nonexistent").is_err());
    assert!(data.remove_entity(name_aa).is_ok());

    // Check that entity was removed in activity
    assert!(data
        .activity(activity_id)
        .unwrap()
        .participants_sorted()
        .is_empty());

    // Cannot remove non-existent entity
    assert!(data.remove_entity(name_aa).is_err());

    // Remove other entity
    assert!(data.remove_entity(name_zz).is_ok());
    let entities = data.entities_sorted();
    assert_eq!(entities.len(), 1);
    assert_eq!(entities[0].name(), name);

    // Set entity mail
    let mail = "macaroni@xyz.com";
    assert!(data.set_entity_mail(name, mail).is_ok());
    assert_eq!(data.entity(name).unwrap().mail(), mail);

    // Set 'send me a mail'
    assert_eq!(data.entity(name).unwrap().send_me_a_mail(), false);
    let send = true;
    data.set_send_mail_to(name, send).unwrap();
    assert!(data.entity(name).unwrap().send_me_a_mail());

    // Add entity to activity to check if it is renamed later
    assert!(data.add_participant_to_activity(activity_id, name).is_ok());

    // Rename entity
    let valid_name = "Emma Caroni";
    assert!(data.set_entity_name(name, valid_name).is_ok());
    assert_eq!(data.entities_sorted()[0].name(), valid_name);
    assert!(data.set_entity_name(name, valid_name).is_err());

    // Check that entity was renamed in the activity
    assert_eq!(
        data.activity(activity_id).unwrap().participants_sorted()[0],
        valid_name
    );

    // Custom work hours should be tested in work_hours test
}

#[test]
fn work_hours_add_remove_global_and_custom() {
    let mut data = Data::new();
    // Add work interval
    let interval_latest = TimeInterval::new(Time::new(14, 0), Time::new(17, 0));
    assert!(data.add_work_interval(interval_latest).is_ok());

    // Cannot add overlapping work interval
    let overlapping_interval = TimeInterval::new(Time::new(12, 0), Time::new(15, 0));
    assert!(data.add_work_interval(overlapping_interval).is_err());
    assert_eq!(data.work_hours().len(), 1);
    assert_eq!(data.work_hours()[0], interval_latest);

    // Add non overlapping work interval
    let interval_earliest = TimeInterval::new(Time::new(8, 0), Time::new(14, 0));
    assert!(data.add_work_interval(interval_earliest).is_ok());

    // Check that work hours are sorted
    assert_eq!(data.work_hours()[0], interval_earliest);
    assert_eq!(data.work_hours()[1], interval_latest);

    // Add custom work hours for an entity
    assert!(data.add_entity("E").is_ok());
    let custom_interval = TimeInterval::new(Time::new(8, 0), Time::new(10, 0));
    assert!(data
        .add_custom_work_interval_for("E", custom_interval)
        .is_ok());

    // Get custom work hour of an entity
    assert_eq!(data.work_hours_of("E").unwrap().len(), 1);
    assert_eq!(data.work_hours_of("E").unwrap()[0], custom_interval);

    // Remove custom work hour from an entity
    assert!(data
        .remove_custom_work_interval_for("E", custom_interval)
        .is_ok());
    assert_eq!(data.work_hours_of("E").unwrap(), data.work_hours());

    // Remove work hour
    assert!(data.remove_work_interval(interval_earliest).is_ok());
    assert_eq!(data.work_hours().len(), 1);

    // Remove non-existent work hour
    assert!(data.remove_work_interval(interval_earliest).is_err());
}

#[test]
fn free_time_of_entity() {
    let mut data = Data::new();
    // Add activities and entities
    let id1 = data.add_activity("Activity 1").unwrap().id();
    let id2 = data.add_activity("Activity 2").unwrap().id();
    let name = data.add_entity("Entity 1").unwrap().name();

    // Add work hours
    let global_morning_shift = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    let global_afternoon_shift = TimeInterval::new(Time::new(14, 0), Time::new(17, 0));
    let total_time = global_morning_shift.duration() + global_afternoon_shift.duration();
    data.add_work_interval(global_morning_shift).unwrap();
    data.add_work_interval(global_afternoon_shift).unwrap();

    // Set durations
    let duration1 = Time::new(1, 0);
    let duration2 = Time::new(2, 0);
    data.set_activity_duration(id1, duration1).unwrap();
    data.set_activity_duration(id2, duration2).unwrap();

    // Check free time without activity
    assert_eq!(data.free_time_of(name.clone()).unwrap(), total_time);

    // Check free time with custom hours without activity
    let custom_shift = TimeInterval::new(Time::new(8, 0), Time::new(11, 0));
    data.add_custom_work_interval_for(name.clone(), custom_shift)
        .unwrap();
    assert_eq!(
        data.free_time_of(name.clone()).unwrap(),
        custom_shift.duration()
    );

    // Check free time with activity and custom hours
    data.add_participant_to_activity(id1, name.clone()).unwrap();
    let expected_free_time = custom_shift.duration() - duration1;
    assert_eq!(data.free_time_of(name.clone()).unwrap(), expected_free_time);

    data.add_participant_to_activity(id2, name.clone()).unwrap();
    let expected_free_time = custom_shift.duration() - duration1 - duration2;
    assert_eq!(data.free_time_of(name.clone()).unwrap(), expected_free_time);

    // Check free time with activities and global work hours
    data.remove_custom_work_interval_for(name.clone(), custom_shift)
        .unwrap();
    let expected_free_time = total_time - duration1 - duration2;
    assert_eq!(data.free_time_of(name.clone()).unwrap(), expected_free_time);

    // Check free time of wrong entity
    assert!(data.free_time_of("I don't even exist").is_err());
}

#[test]
fn activities_add_remove() {
    let mut data = Data::new();

    // Add activity
    let z_activity_name = "Z";
    let z_activity = data.add_activity(z_activity_name);
    assert!(z_activity.is_ok());
    let z_activity_id = z_activity.unwrap().id();
    assert!(data.activity(z_activity_id).is_ok());

    // Add other activities - check sorting
    let a_activity_name = "A";
    data.add_activity(a_activity_name).unwrap();
    let b_activity_name = "B";
    data.add_activity(b_activity_name).unwrap();

    let activities = data.activities_sorted();
    assert_eq!(activities[0].name(), a_activity_name);
    assert_eq!(activities[1].name(), b_activity_name);
    assert_eq!(activities[2].name(), z_activity_name);

    // Remove invalid activity
    let invalid_id = 1234;
    assert!(activities
        .iter()
        .all(|activity| activity.id() != invalid_id));
    assert!(data.remove_activity(invalid_id).is_err());
    let activities = data.activities_sorted();
    assert_eq!(activities.len(), 3);

    // Remove valid activity
    assert!(data.remove_activity(z_activity_id).is_ok());
    let activities = data.activities_sorted();
    assert_eq!(activities.len(), 2);
    assert!(activities
        .iter()
        .all(|activity| activity.id() != z_activity_id));
}

#[test]
fn activities_add_remove_participants() {
    let mut data = Data::new();
    let id = data.add_activity("My New Activity").unwrap().id();

    let b_participant_name = data.add_entity("B name").unwrap().name();
    let a_participant_name = data.add_entity("A name").unwrap().name();
    let z_participant_name = data.add_entity("C Name").unwrap().name();

    let morning_shift = TimeInterval::new(Time::new(9, 0), Time::new(12, 0));
    data.add_work_interval(morning_shift).unwrap();

    // Add new participants to activity
    assert!(data
        .add_participant_to_activity(id, b_participant_name.clone())
        .is_ok());
    assert!(data
        .add_participant_to_activity(id, a_participant_name.clone())
        .is_ok());
    assert!(data
        .add_participant_to_activity(id, z_participant_name.clone())
        .is_ok());

    // Check that participant names are sorted
    let participants = data.activity(id).unwrap().participants_sorted();
    assert_eq!(participants[0], a_participant_name);
    assert_eq!(participants[1], b_participant_name);
    assert_eq!(participants[2], z_participant_name);

    // Add new participant to the wrong activity
    let invalid_id = id + 1;
    assert!(data
        .add_participant_to_activity(invalid_id, b_participant_name.clone())
        .is_err());

    // Add the same participant to the activity
    assert!(data
        .add_participant_to_activity(id, b_participant_name.clone())
        .is_err());
    assert_eq!(
        participants,
        data.activity(id).unwrap().participants_sorted()
    );

    // Add wrong participant to activity
    let invalid_participant_name = "Invalid participant";
    assert!(data
        .add_participant_to_activity(id, invalid_participant_name)
        .is_err());
    assert_eq!(
        participants,
        data.activity(id).unwrap().participants_sorted()
    );

    // Add participant who has not enough time
    let very_long_activity_id = data.add_activity("Very Long").unwrap().id();
    let very_long_duration = Time::new(4, 0);
    data.set_activity_duration(very_long_activity_id, very_long_duration)
        .unwrap();
    assert!(very_long_duration > morning_shift.duration());
    assert!(data
        .add_participant_to_activity(very_long_activity_id, a_participant_name.clone())
        .is_err());

    // Remove the wrong participant from the activity
    assert!(data
        .remove_participant_from_activity(id, invalid_participant_name)
        .is_err());
    assert_eq!(
        participants,
        data.activity(id).unwrap().participants_sorted()
    );

    // Remove participant from the wrong activity
    assert!(data
        .remove_participant_from_activity(invalid_id, b_participant_name.clone())
        .is_err());
    assert_eq!(
        participants,
        data.activity(id).unwrap().participants_sorted()
    );

    // Remove a participant from the activity
    assert!(data
        .remove_participant_from_activity(id, b_participant_name)
        .is_ok());

    let participants = data.activity(id).unwrap().participants_sorted();
    assert_eq!(participants.len(), 2);
    assert_eq!(participants[0], a_participant_name);
    assert_eq!(participants[1], z_participant_name);
}

#[test]
fn activities_set_data() {
    let mut data = Data::new();
    let id = data.add_activity("Name").unwrap().id();

    // Set name of invalid activity
    let invalid_id = id + 1;
    let valid_name = "New Name";
    assert!(data.set_activity_name(invalid_id, valid_name).is_err());

    // Set invalid name
    let invalid_name = " \t";
    assert!(data.set_activity_name(id, invalid_name).is_err());

    // Set name of valid activity
    assert!(data.set_activity_name(id, valid_name).is_ok());
    assert_eq!(data.activity(id).unwrap().name(), valid_name);

    // Set duration of invalid activity
    let valid_duration = Time::new(0, 10);
    assert!(data
        .set_activity_duration(invalid_id, valid_duration)
        .is_err());

    // Set invalid duration
    let invalid_duration = Time::new(0, 0);
    assert!(data.set_activity_duration(id, invalid_duration).is_err());

    // Set duration where an entity does not have enough free time
    let duration_too_long = Time::new(5, 0);
    let entity = data.add_entity("Joel").unwrap().name();
    let short_interval = TimeInterval::new(Time::new(8, 0), Time::new(10, 0));
    data.add_work_interval(short_interval).unwrap();
    data.add_participant_to_activity(id, entity).unwrap();
    assert!(data.set_activity_duration(id, duration_too_long).is_err());

    // Set duration of valid activity
    assert!(data.set_activity_duration(id, valid_duration).is_ok());
    assert_eq!(data.activity(id).unwrap().duration(), valid_duration);
}
