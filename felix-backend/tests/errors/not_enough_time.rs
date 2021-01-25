use felix_backend::errors::not_enough_time::{NotEnoughTime, WhyNotEnoughTime};

#[test]
fn en_display_activity_added() {
    let error = NotEnoughTime::activity_added_for("Entity Name", "Activity");
    assert_eq!(
        error.to_string(),
        "Entity Name will not have enough time if they are added to 'Activity'."
    );
}

#[test]
fn en_display_activity_duration_increased() {
    let error = NotEnoughTime::activity_duration_too_long_for("Entity Name", "Activity");
    assert_eq!(
        error.to_string(),
        "Entity Name will not have enough time if the duration of 'Activity' is increased."
    );
}

#[test]
fn en_display_work_hours_shortened() {
    let error = NotEnoughTime::work_hours_shortened_for("Entity Name");
    assert_eq!(
        error.to_string(),
        "Entity Name will not have enough time if their work hours are shortened."
    );
}

#[test]
fn en_display_added_to_group() {
    let error = NotEnoughTime::added_to_group("Entity Name", "Group");
    assert_eq!(error.to_string(), "Entity Name will not have enough time if they take part in the activities of the group 'Group'.");
}

// TODO translate
#[test]
fn fr_display_activity_added() {}

#[test]
fn fr_display_activity_duration_increased() {}

#[test]
fn fr_display_work_hours_shortened() {}

#[test]
fn fr_display_added_to_group() {}

#[test]
fn activity_added_getters() {
    let error = NotEnoughTime::activity_added_for("Entity Name", "Activity");
    assert_eq!(error.entity_name(), "Entity Name");
    assert_eq!(error.why(), WhyNotEnoughTime::ActivityAdded);
}

#[test]
fn activity_duration_increased_getters() {
    let error = NotEnoughTime::activity_duration_too_long_for("Entity Name", "Activity");
    assert_eq!(error.entity_name(), "Entity Name");
    assert_eq!(error.why(), WhyNotEnoughTime::ActivityDurationIncreased);
}

#[test]
fn work_hours_shortened_getters() {
    let error = NotEnoughTime::work_hours_shortened_for("Entity Name");
    assert_eq!(error.entity_name(), "Entity Name");
    assert_eq!(error.why(), WhyNotEnoughTime::WorkHoursShortened);
}

#[test]
fn added_to_group_getters() {
    let error = NotEnoughTime::added_to_group("Entity Name", "Group");
    assert_eq!(error.entity_name(), "Entity Name");
    assert_eq!(error.why(), WhyNotEnoughTime::AddedToGroup);
}
