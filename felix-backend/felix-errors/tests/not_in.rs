use felix_errors::not_in::{ActivityOrGroup, EntityOrGroup, NotIn};

#[test]
fn en_display_entity_not_in_group() {
    let error = NotIn::entity_not_in_group("Entity", "Group");
    assert_eq!(error.to_string(), "Entity is not in the group 'Group'.");
}

#[test]
fn en_display_entity_not_in_activity() {
    let error = NotIn::entity_not_in_activity("Entity", "A");
    assert_eq!(error.to_string(), "Entity is not in the activity 'A'.");
}

#[test]
fn en_display_group_not_in_activity() {
    let error = NotIn::group_not_in_activity("Group", "A");
    assert_eq!(
        error.to_string(),
        "The group 'Group' is not in the activity 'A'."
    );
}

// TODO translate
#[test]
fn fr_display_entity_not_in_group() {}

#[test]
fn fr_display_entity_not_in_activity() {}

#[test]
fn fr_display_group_not_in_activity() {}

#[test]
fn entity_not_in_group_getters() {
    let error = NotIn::entity_not_in_group("Entity", "Group");
    assert_eq!(error.what(), EntityOrGroup::Entity);
    assert_eq!(error.who(), "Entity");
    assert_eq!(error.in_what(), ActivityOrGroup::Group);
    assert_eq!(error.in_who(), "Group");
}

#[test]
fn entity_not_in_activity_getters() {
    let error = NotIn::entity_not_in_activity("Entity", "Activity");
    assert_eq!(error.what(), EntityOrGroup::Entity);
    assert_eq!(error.who(), "Entity");
    assert_eq!(error.in_what(), ActivityOrGroup::Activity);
    assert_eq!(error.in_who(), "Activity");
}

#[test]
fn group_not_in_activity_getters() {
    let error = NotIn::group_not_in_activity("Group", "Activity");
    assert_eq!(error.what(), EntityOrGroup::Group);
    assert_eq!(error.who(), "Group");
    assert_eq!(error.in_what(), ActivityOrGroup::Activity);
    assert_eq!(error.in_who(), "Activity");
}
