use felix_datatypes::{Time, TimeInterval};
use felix_errors::does_not_exist::{ComponentType, DoesNotExist};

#[test]
fn en_display_interval_does_not_exist() {
    let interval = TimeInterval::new(Time::new(8, 0), Time::new(12, 30));
    let error = DoesNotExist::interval_does_not_exist(interval);
    assert_eq!(
        error.to_string(),
        "The interval '08:00 - 12:30' does not exist."
    );
}

#[test]
fn en_display_entity_does_not_exist() {
    let name = "Entity Name";
    let error = DoesNotExist::entity_does_not_exist(name);
    assert_eq!(error.to_string(), "Entity Name does not exist.");
}

#[test]
fn en_display_group_does_not_exist() {
    let name = "Group Name";
    let error = DoesNotExist::group_does_not_exist(name);
    assert_eq!(error.to_string(), "The group 'Group Name' does not exist.");
}

// TODO translate
#[test]
fn fr_display_interval_does_not_exist() {}

#[test]
fn fr_display_entity_does_not_exist() {}

#[test]
fn fr_display_group_does_not_exist() {}

#[test]
fn interval_does_not_exist_getters() {
    let interval = TimeInterval::new(Time::new(8, 0), Time::new(12, 30));
    let error = DoesNotExist::interval_does_not_exist(interval);
    assert_eq!(error.what(), ComponentType::TimeInterval);
    assert_eq!(error.who(), "08:00 - 12:30");
}

#[test]
fn entity_does_not_exist_getters() {
    let name = "Entity Name";
    let error = DoesNotExist::entity_does_not_exist(name);
    assert_eq!(error.what(), ComponentType::Entity);
    assert_eq!(error.who(), "Entity Name");
}

#[test]
fn group_does_not_exist_getters() {
    let name = "Group Name";
    let error = DoesNotExist::group_does_not_exist(name);
    assert_eq!(error.what(), ComponentType::Group);
    assert_eq!(error.who(), "Group Name");
}
