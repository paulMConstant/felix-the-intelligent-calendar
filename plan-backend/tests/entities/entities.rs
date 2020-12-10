//! Basic operations on entities.
//! Does not check interaction with activities and groups.
//!
//! Includes
//! - Addition
//! - Deletion
//! - Edition (name, mail, send_me_a_mail)
//! - Getter

use test_utils::data_builder::DataBuilder;

// *** Add ***
#[test]
fn simple_add_entity() {
    test_ok!(data, DataBuilder::new(), {
        let name = data.add_entity("Name").expect("Could not add valid entity");
        assert_eq!(name, "Name", "The given name was not returned right");

        let entities = data.entities_sorted();
        assert_eq!(entities.len(), 1, "Entity was not added to the collection");
        assert_eq!(
            entities[0].name(),
            "Name",
            "Entity was not added with the right name"
        );
    });
}

#[test]
fn add_entity_empty_name() {
    test_err!(
        data,
        DataBuilder::new(),
        data.add_entity(" \t"),
        "The given name is empty.",
        "Could add entity with empty name"
    );
}

#[test]
fn add_entity_entity_has_same_name() {
    let name = "Entity";
    test_err!(
        data,
        DataBuilder::new().with_entity(name),
        data.add_entity(name),
        "The name 'Entity' is already taken by an entity.",
        "Could add entity with a taken name"
    );
}

#[test]
fn add_entity_check_name_formatting() {
    test_ok!(data, DataBuilder::new(), {
        let name = data
            .add_entity("emma   carena\t")
            .expect("Could not add valid entity");
        assert_eq!(
            name, "Emma Carena",
            "The given name was not formatted right"
        );
    });
}

#[test]
fn add_entity_check_sorting() {
    let (name_a, name_b, name_c) = ("Name A", "Name B", "Name C");
    test_ok!(
        data,
        DataBuilder::new().with_entities(vec![name_b, name_c, name_a]),
        {
            let entities = data.entities_sorted();
            assert_eq!(entities[0].name(), name_a, "Entities were not sorted right");
            assert_eq!(entities[1].name(), name_b, "Entities were not sorted right");
            assert_eq!(entities[2].name(), name_c, "Entities were not sorted right");
        }
    );
}

// *** Remove ***
#[test]
fn simple_remove_entity() {
    let (name1, name2) = ("Name1", "Name2");
    test_ok!(
        data,
        DataBuilder::new().with_entities(vec![name1, name2]),
        {
            data.remove_entity(name1)
                .expect("Could not remove valid entity");

            let entities = data.entities_sorted();
            assert_eq!(entities.len(), 1, "The entity was not removed");
            assert_eq!(entities[0].name(), name2, "The wrong entity was removed");
        }
    );
}

#[test]
fn remove_entity_empty_name() {
    test_err!(
        data,
        DataBuilder::new(),
        data.remove_entity(" \t"),
        "The given name is empty.",
        "Could remove empty entity"
    );
}

#[test]
fn remove_entity_does_not_exist() {
    test_err!(
        data,
        DataBuilder::new(),
        data.remove_entity("Other name"),
        "Other Name does not exist.",
        "Could remove nonexistent entity"
    );
}

#[test]
fn remove_entity_check_name_formatting() {
    test_ok!(data, DataBuilder::new().with_entity("Name"), {
        data.remove_entity("name  \t")
            .expect("Could not remove entity: name formatting was not done");
        assert_eq!(data.entities_sorted().len(), 0, "Entity was not removed");
    });
}

// *** Get individual ***
#[test]
fn simple_get_entity() {
    let name = "Name";
    test_ok!(data, DataBuilder::new().with_entity(name), {
        let entity = data
            .entity(name.clone())
            .expect("Could not get entity by name");
        assert_eq!(
            data.entities_sorted()[0],
            entity,
            "Did not get the right entity"
        );
    });
}

#[test]
fn get_invalid_entity() {
    test_err!(
        data,
        DataBuilder::new(),
        data.remove_entity("Does not exist"),
        "Does Not Exist does not exist.",
        "Could remove invalid entity"
    );
}

// *** Modify mail, send_me_a_mail ***
#[test]
fn set_entity_mail() {
    let name = "Name";
    test_ok!(data, DataBuilder::new().with_entity(name), {
        let mail = "name@xyz.com";
        data.set_entity_mail(name, mail.clone())
            .expect("Could not set entity mail");

        let entity_mail = data
            .entity(name)
            .expect("Could not get entity by name")
            .mail();
        assert_eq!(entity_mail, mail, "Mail was not set");
    });
}

#[test]
fn set_send_me_a_mail() {
    let name = "Name";
    test_ok!(data, DataBuilder::new().with_entity(name), {
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
    });
}

// *** Rename ***
#[test]
fn simple_rename_entity() {
    let name = "Name";
    test_ok!(data, DataBuilder::new().with_entity(name), {
        let new_name = data
            .set_entity_name(name.clone(), "New Name")
            .expect("Could not rename entity");
        data.entity(new_name).expect("Could not get renamed entity");
        // Check that the entity with old name is removed
        assert_not_modified!(data, {
            assert_eq!(
                format!(
                    "{}",
                    data.entity(name)
                        .expect_err("Could get renamed entity with old name")
                ),
                "Name does not exist.",
                "Got wrong error message"
            );
        });
    });
}

#[test]
fn rename_entity_empty_name() {
    let name = "Name";
    test_err!(
        data,
        DataBuilder::new().with_entity(name),
        data.set_entity_name(name, " \t"),
        "The given name is empty.",
        "Could rename to empty name"
    );
}

#[test]
fn rename_entity_entity_already_exists() {
    let name = "Name";
    let other_name = "Other Name";
    test_err!(
        data,
        DataBuilder::new().with_entities(vec![other_name, name]),
        data.set_entity_name(name, other_name),
        "The name 'Other Name' is already taken by an entity.",
        "Could rename with name taken by other entity"
    );
}

#[test]
fn rename_entity_check_formatting() {
    let name = "Name";
    test_ok!(data, DataBuilder::new().with_entity(name), {
        let new_name = data
            .set_entity_name(name, "new  name  \t")
            .expect("Could not rename entity");
        assert_eq!(new_name, "New Name", "New name was not formatted right");
    });
}
