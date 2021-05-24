use felix_datatypes::{Time, TimeInterval};
use std::panic::catch_unwind;

#[test]
fn duration() {
    let interval = TimeInterval::new(Time::new(10, 0), Time::new(11, 0));
    assert_eq!(
        interval.duration(),
        Time::new(1, 0),
        "Duration was not calculated right"
    );
}

#[test]
fn overlap() {
    let ten_to_eleven = TimeInterval::new(Time::new(10, 0), Time::new(11, 0));
    let tenthirty_to_eleventhirty = TimeInterval::new(Time::new(10, 30), Time::new(11, 30));
    let eleven_to_twelve = TimeInterval::new(Time::new(11, 0), Time::new(12, 0));

    assert_eq!(
        ten_to_eleven.overlaps_with(&tenthirty_to_eleventhirty),
        true,
        "Did not catch overlapping intervals"
    );
    assert_eq!(
        ten_to_eleven.overlaps_with(&eleven_to_twelve),
        false,
        "Mistook non-overlapping interval for overlapping interval"
    );
}

#[test]
fn contains() {
    let interval = TimeInterval::new(Time::new(8, 0), Time::new(9, 0));
    assert_eq!(interval.contains(Time::new(7, 0)), false);
    assert!(interval.contains(Time::new(8, 0)));
    assert!(interval.contains(Time::new(8, 30)));
    assert_eq!(interval.contains(Time::new(9, 0)), false);
}

#[test]
fn contains_interval() {
    let interval1 = TimeInterval::new(Time::new(8, 0), Time::new(9, 0));
    let interval2 = TimeInterval::new(Time::new(8, 0), Time::new(8, 30));

    assert!(interval1.contains_interval(interval2));
    assert!(interval1.contains_interval(interval1));
    assert_eq!(interval2.contains_interval(interval1), false);
}

#[test]
fn invalid_new() {
    catch_unwind(|| TimeInterval::new(Time::new(10, 0), Time::new(9, 0)))
        .expect_err("Created TimeInterval with beginning > end");
    catch_unwind(|| TimeInterval::new(Time::new(10, 0), Time::new(10, 0)))
        .expect_err("Created TimeInterval of duration 0");
    catch_unwind(|| TimeInterval::new(Time::new(10, 0), Time::new(10, 27)))
        .expect_err("Created TimeInterval which is not a multiple of MIN_TIME_DISCRETIZATION");
}

#[test]
fn std_cmp() {
    let interval1 = TimeInterval::new(Time::new(8, 0), Time::new(10, 0));
    let interval2 = TimeInterval::new(Time::new(11, 0), Time::new(13, 0));
    assert!(interval1 < interval2, "TimeInterval comparison is broken");
}

#[test]
fn display() {
    let interval = TimeInterval::new(Time::new(8, 0), Time::new(12, 15));
    assert_eq!(format!("{}", interval), "08:00 - 12:15");
}
