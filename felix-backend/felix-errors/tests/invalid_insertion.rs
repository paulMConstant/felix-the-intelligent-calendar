use felix_datatypes::Time;
use felix_errors::invalid_insertion::{InvalidInsertion, InvalidOrNotComputed, WhyInvalid};

#[test]
fn en_display_cannot_fit_or_would_block_other_activities() {
    let error =
        InvalidInsertion::cannot_fit_or_would_block_other_activities("Activity", Time::new(8, 0));
    assert_eq!(
        error.to_string(),
        "Activity cannot be inserted with beginning 08:00 because this beginning is invalid or will cause problems in the future."
    );
}

#[test]
fn en_display_would_overlap_with_activity() {
    let error = InvalidInsertion::would_overlap_with_activity(
        "Activity",
        Time::new(8, 0),
        "Blocking Activity",
    );
    assert_eq!(
        error.to_string(),
        "Activity cannot be inserted with beginning 08:00 because it would overlap with 'Blocking Activity'."
    );
}

#[test]
fn en_display_insertion_not_computed() {
    let error = InvalidInsertion::insertions_not_computed_yet("Activity");
    assert_eq!(
        error.to_string(),
        "The possible beginnings of the activity 'Activity' have not been computed yet."
    );
}

// TODO translate
#[test]
fn fr_display_entity_already_in_group() {}

#[test]
fn fr_display_cannot_fit_or_would_block_other_activities() {}

#[test]
fn fr_display_would_overlap_with_activity() {}

#[test]
fn invalid_insertion_getters() {
    let error =
        InvalidInsertion::cannot_fit_or_would_block_other_activities("Activity", Time::new(8, 0));
    assert_eq!(error.who(), "Activity");
    assert_eq!(error.in_who(), Time::new(8, 0));
    assert_eq!(
        error.reason(),
        InvalidOrNotComputed::Invalid(WhyInvalid::CannotFitOrWouldBlockOtherActivities)
    );
}
