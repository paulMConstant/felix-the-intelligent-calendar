use plan_backend::errors::not_enough_time::{NotEnoughTime, WhyNotEnoughTime};

#[test]
fn en_display_activity_added() {
    let error = NotEnoughTime::activity_added_for("Entity Name");
    assert_eq!(error.to_string(), "'Entity Name' will not have enough time if they are added to this activity.");
}

#[test]
fn en_display_activity_duration_increased() {
    let error = NotEnoughTime::activity_duration_too_long_for("Entity Name");
    assert_eq!(error.to_string(), "'Entity Name' will not have enough time if the duration of this activity is increased.");
}

#[test]
fn en_display_work_hours_shortened() {
    let error = NotEnoughTime::work_hours_shortened_for("Entity Name");
    assert_eq!(error.to_string(), "'Entity Name' will not have enough time if their work hours are shortened.");
}

// TODO translate
#[test]
fn fr_display_activity_added() {
}

#[test]
fn fr_display_activity_duration_increased() {
}

#[test]
fn fr_display_work_hours_shortened() {
}

#[test]
fn activity_added_getters() {
    let error = NotEnoughTime::activity_added_for("Entity Name");
    assert_eq!(error.entity_name(), "Entity Name");
    assert_eq!(error.why(), WhyNotEnoughTime::ActivityAdded);
}

#[test]
fn activity_duration_increased_getters() {
    let error = NotEnoughTime::activity_duration_too_long_for("Entity Name");
    assert_eq!(error.entity_name(), "Entity Name");
    assert_eq!(error.why(), WhyNotEnoughTime::ActivityDurationIncreased);
}
#[test]
fn work_hours_shortened_getters() {
    let error = NotEnoughTime::work_hours_shortened_for("Entity Name");
    assert_eq!(error.entity_name(), "Entity Name");
    assert_eq!(error.why(), WhyNotEnoughTime::WorkHoursShortened);
}

