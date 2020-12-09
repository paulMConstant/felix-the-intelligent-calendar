use plan_backend::errors::name_taken::{NameTaken, GroupOrEntity};

#[test]
fn en_display_name_taken_by_entity() {
    let error = NameTaken::name_taken_by_entity("Entity Name");
    assert_eq!(error.to_string(), "The name 'Entity Name' is already taken by an entity.");
}

#[test]
fn en_display_name_taken_by_group() {
    let error = NameTaken::name_taken_by_group("Group Name");
    assert_eq!(error.to_string(), "The name 'Group Name' is already taken by a group.");
}

// TODO translate
#[test]
fn fr_display_name_taken_by_entity() {

}

#[test]
fn fr_display_name_taken_by_group() {

}

#[test]
fn name_taken_by_entity_getters() {
    let error = NameTaken::name_taken_by_entity("Entity Name");
    assert_eq!(error.by(), GroupOrEntity::Entity);
    assert_eq!(error.name(), "Entity Name");
}

#[test]
fn name_taken_by_group_getters() {
    let error = NameTaken::name_taken_by_group("Group Name");
    assert_eq!(error.by(), GroupOrEntity::Group);
    assert_eq!(error.name(), "Group Name");
}
