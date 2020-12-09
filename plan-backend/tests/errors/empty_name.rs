use plan_backend::errors::empty_name::EmptyName;

#[test]
fn en_display_empty_name() {
    assert_eq!(EmptyName::new().to_string(), "The given name is empty.");
}

// TODO translate
#[test]
fn fr_display_empty_name() {}

