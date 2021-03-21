use felix_backend::errors::add_entity_to_inserted_activity_invalid_spot::{
    AddEntityToInsertedActivityInvalidSpot, WhySpotIsInvalid,
};

#[test]
fn en_display_blocking_activity() {
    let error = AddEntityToInsertedActivityInvalidSpot::blocking_activity(
        "Entity",
        "Activity",
        "BlockingActivity",
    );
    assert_eq!(
        error.to_string(),
        "Entity cannot be added to 'Activity' because it would overlap with 'BlockingActivity'."
    );
}

#[test]
fn en_display_outside_of_work_hours() {
    let error = AddEntityToInsertedActivityInvalidSpot::outside_of_work_hours("Entity", "Activity");
    assert_eq!(
        error.to_string(),
        "Entity cannot be added to 'Activity' because it would be outside of their work hours."
    );
}
// TODO translate
#[test]
fn fr_display_blocking_activity() {}

#[test]
fn fr_display_outside_of_work_hours() {}

#[test]
fn getters() {
    let error = AddEntityToInsertedActivityInvalidSpot::blocking_activity(
        "Entity",
        "Activity",
        "BlockingActivity",
    );
    assert_eq!(error.who(), "Entity");
    assert_eq!(error.in_what(), "Activity");
    assert_eq!(
        error.why(),
        WhySpotIsInvalid::BlockingActivity("BlockingActivity".to_string())
    );

    let error = AddEntityToInsertedActivityInvalidSpot::outside_of_work_hours("Entity", "Activity");
    assert_eq!(error.who(), "Entity");
    assert_eq!(error.in_what(), "Activity");
    assert_eq!(error.why(), WhySpotIsInvalid::OutsideOfWorkHours);
}
