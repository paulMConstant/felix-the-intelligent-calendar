use plan_backend::errors::interval_overlaps::IntervalOverlaps;

#[test]
fn en_display() {
    assert_eq!(format!("{}", IntervalOverlaps::new()), "The given interval overlaps with others.")
}

// TODO translate
#[test]
fn fr_display() {

}
