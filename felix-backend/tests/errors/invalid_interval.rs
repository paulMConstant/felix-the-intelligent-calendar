use felix_backend::errors::invalid_interval::InvalidInterval;

#[test]
fn en_display_invalid_interval() {
    let error = InvalidInterval::new();
    assert_eq!(
        error.to_string(),
        "This interval is not valid. The end must be greater than the beginning."
    );
}

// TODO translate
#[test]
fn fr_display_invalid_interval() {}
