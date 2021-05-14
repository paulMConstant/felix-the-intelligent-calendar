use felix_backend::errors::change_work_hours_while_activity_inserted::ChangeWorkHoursWhileActivityInserted;

#[test]
fn en_display() {
    let error = ChangeWorkHoursWhileActivityInserted::new();
    assert_eq!(error.to_string(),
    "Work hours cannot be modified while an activity is inserted.");
}

#[test]
fn fr_display() { 
    //TODO translate
}
