//! Basic operations on entities.
//! Does not check interaction with activities and groups.
//!
//! Includes
//! - Addition
//! - Deletion
//! - Edition (name, mail, send_me_a_mail)
//! - Getter

use plan_backend::data::Data;

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
    assert_not_modified!(data, {
        assert_eq!(
            data.add_entity(" \t"),
            Err("The given name is empty.".to_owned()),
            "Could add entity with empty name"
        );
    });
}

#[test]
fn add_entity_entity_has_same_name() {
    let mut data = Data::new();

    let name = data
        .add_entity("name")
        .expect("Could not add valid entity")
        .name();

    assert_not_modified!(data, {
        assert_eq!(
            data.add_entity(name),
            Err("The name 'Name' is already taken by an entity.".to_owned()),
            "Could add entity with a taken name"
        );
    });
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

    assert_not_modified!(data, {
        assert_eq!(
            data.remove_entity(" \t"),
            Err("The given name is empty.".to_owned()),
            "Could remove empty entity"
        );
        assert_eq!(
            data.remove_entity("Other name"),
            Err("The entity 'Other Name' does not exist.".to_owned()),
            "Could remove nonexistent entity"
        );
    });
}

#[test]
fn remove_entity_check_name_formatting() {
    let mut data = Data::new();

    data.add_entity("Name").expect("Could not add valid entity");
    data.remove_entity("name  \t")
        .expect("Could not remove entity: name formatting was not done");
    assert_eq!(data.entities_sorted().len(), 0, "Entity was not removed");
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

    assert_not_modified!(data, {
        assert_eq!(
            data.remove_entity("Does not exist"),
            Err("The entity 'Does Not Exist' does not exist.".to_owned()),
            "Could remove invalid entity"
        );
    });
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
    assert_not_modified!(data, {
        assert_eq!(
            data.entity(name),
            Err("The entity 'Name' does not exist.".to_owned()),
            "Could get renamed entity with old name"
        );
        data.entity(new_name).expect("Could not get renamed entity");
    });
}

#[test]
fn rename_entity_empty_name() {
    let mut data = Data::new();

    let name = data
        .add_entity("Name")
        .expect("Could not add entity")
        .name();
    assert_not_modified!(data, {
        assert_eq!(
            data.set_entity_name(name, " \t"),
            Err("The given name is empty.".to_owned()),
            "Could rename to empty name"
        );
    });
}

#[test]
fn rename_entity_entity_already_exists() {
    let mut data = Data::new();

    let name = data
        .add_entity("Name")
        .expect("Could not add entity")
        .name();
    let other_name = data
        .add_entity("Other Name")
        .expect("Could not add entity")
        .name();

    assert_not_modified!(data, {
        assert_eq!(
            data.set_entity_name(name, other_name),
            Err("The name 'Other Name' is already taken by another entity.".to_owned()),
            "Could rename with name taken by other entity"
        );
    });
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
