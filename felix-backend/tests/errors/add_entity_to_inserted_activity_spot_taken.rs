use felix_backend::errors::add_entity_to_inserted_activity_spot_taken::AddEntityToInsertedActivitySpotTaken;

#[test]
fn en_display() {
    let error = AddEntityToInsertedActivitySpotTaken::new("Entity", "Activity", "BlockingActivity");
    assert_eq!(
        error.to_string(),
        "Entity cannot be added to 'Activity' because it would overlap with 'BlockingActivity'."
    );
}

// TODO translate
#[test]
fn fr_display() {}

#[test]
fn getters() {
    let error = AddEntityToInsertedActivitySpotTaken::new("Entity", "Activity", "BlockingActivity");
    assert_eq!(error.who(), "Entity");
    assert_eq!(error.in_what(), "Activity");
    assert_eq!(error.blocking_activity(), "BlockingActivity");
}
