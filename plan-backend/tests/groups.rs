//use plan_backend::data::Data;

// Test organization
// - Add group
// - Remove group
// - Get individual group
// - Add entity to group
// - Remove entity from group
// - Rename group

// *** Add group ***
#[test]
fn simple_add_group() {}

#[test]
fn add_group_check_sorting() {}

#[test]
fn add_group_entity_has_same_name() {}

#[test]
fn add_group_group_has_same_name() {}

// *** Remove group ***
#[test]
fn remove_group() {}

#[test]
fn remove_group_check_removed_in_activities() {}

#[test]
fn remove_invalid_group() {}

// *** Get individual group ***
#[test]
fn simple_get_group() {}

// *** Add entity to group ***
#[test]
fn simple_add_entity_to_group() {}

#[test]
fn add_entity_to_group_already_in_group() {}

#[test]
fn add_nonexistent_entity_to_group() {}

#[test]
fn add_entity_to_nonexistent_group() {}

// *** Remove entity from group ***
#[test]
fn simple_remove_entity() {}

#[test]
fn remove_entity_not_in_group() {}

#[test]
fn remove_nonexistent_entity_from_group() {}

#[test]
fn remove_entity_from_nonexistent_group() {}

// *** Rename group ***
#[test]
fn simple_rename_group() {}

#[test]
fn rename_invalid_group() {}

#[test]
fn rename_group_check_renamed_in_activities() {}
