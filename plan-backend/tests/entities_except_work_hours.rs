use plan_backend::data::{Data, Time, TimeInterval};

// Test organization
// - Add
// - Remove
// - Get individual
// - Modify mail, send_me_a_mail
// - Rename
// - Custom Work Hours
// - Get work hours of entity
// - Free time

// *** Add ***
#[test]
fn simple_add_entity() {
    let mut data = Data::new();

    let name = data
        .add_entity("Name")
        .expect("Could not add valid entity")
        .name();
    assert_eq!(name, "Name", "The given name was not returned right");

    let entities = data.entities_sorted();
    assert_eq!(entities.len(), 1, "Entity was not added to the collection");
    assert_eq!(
        entities[0].name(),
        "Name",
        "Entity was not added with the right name"
    );
}

#[test]
fn add_entity_empty_name() {
    let mut data = Data::new();

    assert_eq!(
        data.add_entity(" \t"),
        Err("The formatted name is empty.".to_owned()),
        "Could add entity with empty name"
    );
}

#[test]
fn add_entity_taken_name() {
    let mut data = Data::new();

    let name = "name";
    data.add_entity(name).expect("Could not add valid entity");
    assert_eq!(
        data.add_entity(name),
        Err("Name already exists !".to_owned()),
        "Could add entity with a taken name"
    );
}

#[test]
fn add_entity_check_name_formatting() {
    let mut data = Data::new();

    let name = data
        .add_entity("emma   carena\t")
        .expect("Could not add valid entity")
        .name();
    assert_eq!(
        name, "Emma Carena",
        "The given name was not formatted right"
    );
}

#[test]
fn add_entity_check_sorting() {
    let mut data = Data::new();

    let name_a = data
        .add_entity("name a")
        .expect("Could not add valid entity")
        .name();
    let name_c = data
        .add_entity("name c")
        .expect("Could not add valid entity")
        .name();
    let name_b = data
        .add_entity("name b")
        .expect("Could not add valid entity")
        .name();

    let entities = data.entities_sorted();
    assert_eq!(entities.len(), 3, "Entities were not added right");
    assert_eq!(entities[0].name(), name_a, "Entities were not sorted right");
    assert_eq!(entities[1].name(), name_b, "Entities were not sorted right");
    assert_eq!(entities[2].name(), name_c, "Entities were not sorted right");
}

// *** Remove ***
#[test]
fn simple_remove_entity() {
    let mut data = Data::new();

    // Add two entities to check that the right one was deleted
    let name1 = data
        .add_entity("Emma")
        .expect("Could not add valid entity")
        .name();
    let name2 = data
        .add_entity("Emma Carena")
        .expect("Could not add valid entity")
        .name();

    data.remove_entity(name1)
        .expect("Could not remove valid entity");

    let entities = data.entities_sorted();
    assert_eq!(entities.len(), 1, "The entity was not removed");
    assert_eq!(entities[0].name(), name2, "The wrong entity was removed");
}

#[test]
fn remove_entity_invalid_name() {
    let mut data = Data::new();

    data.add_entity("Name").expect("Could not add valid entity");

    assert_eq!(
        data.remove_entity(" \t"),
        Err("The formatted name is empty.".to_owned()),
        "Could remove empty entity"
    );
    assert_eq!(
        data.remove_entity("Other name"),
        Err("Other Name does not exist !".to_owned()),
        "Could remove nonexistent entity"
    );
    assert_eq!(
        data.entities_sorted().len(),
        1,
        "Entity was removed after erroneous remove operation"
    );
}

#[test]
fn remove_entity_check_name_formatting() {
    let mut data = Data::new();

    data.add_entity("Name").expect("Could not add valid entity");
    data.remove_entity("name  \t")
        .expect("Could not remove entity: name formatting was not done");
    assert_eq!(data.entities_sorted().len(), 0, "Entity was not removed");
}

#[test]
fn remove_entity_check_remove_in_activity() {
    let mut data = Data::new();

    // Add work hours and an activity
    let interval = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    data.add_work_interval(interval)
        .expect("Could not add valid work interval");
    let id = data
        .add_activity("Activity")
        .expect("Could not add valid activity")
        .id();

    // Add two entities to check that the right one is removed
    let name1 = data
        .add_entity("Entity name1")
        .expect("Could not add valid entity")
        .name();
    let name2 = data
        .add_entity("Entity name2")
        .expect("Could not add valid entity")
        .name();
    data.add_participant_to_activity(id, name1.clone())
        .expect("Could not add participant to activity");
    data.add_participant_to_activity(id, name2.clone())
        .expect("Could not add participant to activity");

    let participants = data
        .activity(id)
        .expect("Could not get activity")
        .participants_sorted();
    assert_eq!(
        participants.len(),
        2,
        "Entities were not added to the activity"
    );

    data.remove_entity(name1).expect("Could not remove entity");
    let participants = data
        .activity(id)
        .expect("Could not get activity")
        .participants_sorted();
    assert_eq!(
        participants.len(),
        1,
        "Entity was not removed from the activity"
    );
    assert_eq!(participants[0], name2, "The wrong participant was removed");
}

// *** Get individual ***
#[test]
fn simple_get_entity() {
    let mut data = Data::new();

    let name = data
        .add_entity("name")
        .expect("Could not add valid entity")
        .name();
    let entities = data.entities_sorted();
    assert_eq!(entities.len(), 1, "Entity was not added to the collection");
    assert_eq!(
        entities[0].name(),
        name,
        "Entity was not added with the right name"
    );

    let entity = data
        .entity(name.clone())
        .expect("Could not get entity by name");
    assert_eq!(entities[0], entity, "Did not get the right entity");
}

#[test]
fn get_invalid_entity() {
    let mut data = Data::new();

    assert_eq!(
        data.remove_entity("Does not exist"),
        Err("Does Not Exist does not exist !".to_owned()),
        "Could remove invalid entity"
    );
}

// *** Modify mail, send_me_a_mail ***
#[test]
fn set_entity_mail() {
    let mut data = Data::new();

    let name = data
        .add_entity("Name")
        .expect("Could not add valid entity")
        .name();
    data.entity(name.clone())
        .expect("Could not get entity by name");

    let mail = "name@xyz.com";
    data.set_entity_mail(name.clone(), mail.clone())
        .expect("Could not set entity mail");

    let entity_mail = data
        .entity(name.clone())
        .expect("Could not get entity by name")
        .mail();
    assert_eq!(entity_mail, mail, "Mail was not set");
}

#[test]
fn set_send_me_a_mail() {
    let mut data = Data::new();

    let name = data
        .add_entity("Name")
        .expect("Could not add valid entity")
        .name();
    data.entity(name.clone())
        .expect("Could not get entity by name");

    let send_me_a_mail = true;
    data.set_send_mail_to(name.clone(), send_me_a_mail)
        .expect("Could not set send_me_a_mail");

    let entity_send_me_a_mail = data
        .entity(name.clone())
        .expect("Could not get entity by name")
        .send_me_a_mail();
    assert_eq!(
        entity_send_me_a_mail, send_me_a_mail,
        "Send_me_a_mail was not set"
    );

    // Check again with false value
    let send_me_a_mail = false;
    data.set_send_mail_to(name.clone(), send_me_a_mail)
        .expect("Could not set send_me_a_mail");

    let entity_send_me_a_mail = data
        .entity(name.clone())
        .expect("Could not get entity by name")
        .send_me_a_mail();
    assert_eq!(
        entity_send_me_a_mail, send_me_a_mail,
        "Send_me_a_mail was not set"
    );
}

// *** Rename ***
#[test]
fn simple_rename_entity() {
    let mut data = Data::new();

    let name = data
        .add_entity("Name")
        .expect("Could not add valid entity")
        .name();
    data.entity(name.clone())
        .expect("Could not get entity by name");

    let new_name = data
        .set_entity_name(name.clone(), "New Name")
        .expect("Could not rename entity");
    assert_eq!(
        data.entity(name),
        Err("Name does not exist !".to_owned()),
        "Could get renamed entity with old name"
    );
    data.entity(new_name).expect("Could not get renamed entity");
}

#[test]
fn rename_entity_check_formatting() {
    let mut data = Data::new();

    let name = data
        .add_entity("Name")
        .expect("Could not add valid entity")
        .name();
    let new_name = data
        .set_entity_name(name, "new  name  \t")
        .expect("Could not rename entity");
    assert_eq!(new_name, "New Name", "New name was not formatted right");
}

#[test]
fn rename_entity_check_renamed_in_activity() {
    let mut data = Data::new();

    // Add work hours and an activity
    let interval = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    data.add_work_interval(interval)
        .expect("Could not add valid work interval");
    let id = data
        .add_activity("Activity")
        .expect("Could not add valid activity")
        .id();

    // Add two entities to check that the right one is renamed
    let name1 = data
        .add_entity("Entity name1")
        .expect("Could not add valid entity")
        .name();
    let name2 = data
        .add_entity("Entity name2")
        .expect("Could not add valid entity")
        .name();
    data.add_participant_to_activity(id, name1.clone())
        .expect("Could not add participant to activity");
    data.add_participant_to_activity(id, name2.clone())
        .expect("Could not add participant to activity");

    let participants = data
        .activity(id)
        .expect("Could not get activity")
        .participants_sorted();
    assert_eq!(
        participants.len(),
        2,
        "Entities were not added to the activity"
    );

    let name3 = data
        .set_entity_name(name1, "Entity name3")
        .expect("Could not rename entity");
    let participants = data
        .activity(id)
        .expect("Could not get activity")
        .participants_sorted();

    assert_eq!(
        participants[0], name2,
        "The wrong participant was renamed or they are not sorted"
    );
    assert_eq!(
        participants[1], name3,
        "The participant was not renamed or they are not sorted"
    );
}
