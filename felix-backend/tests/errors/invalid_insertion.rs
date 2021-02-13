use felix_backend::data::Time;
use felix_backend::errors::invalid_insertion::{InvalidInsertion, InvalidOrNotComputed};

#[test]
fn en_display_invalid_insertion() {
    let error = InvalidInsertion::insertion_not_in_computed_insertions("Activity", Time::new(8, 0));
    assert_eq!(
        error.to_string(),
        "The activity 'Activity' cannot be inserted with beginning 08:00."
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
fn fr_display_insertion_not_computed() {}

#[test]
fn invalid_insertion_getters() {
    let error = InvalidInsertion::insertion_not_in_computed_insertions("Activity", Time::new(8, 0));
    assert_eq!(error.who(), "Activity");
    assert_eq!(error.in_who(), Time::new(8, 0));
    assert_eq!(error.reason(), InvalidOrNotComputed::Invalid);
}
