use felix_errors::duration_too_short::DurationTooShort;

#[test]
fn en_display_duration_too_short() {
    assert_eq!(
        DurationTooShort::new().to_string(),
        "The given duration is too short."
    );
}

// TODO translate
#[test]
fn fr_display_duration_too_short() {}
