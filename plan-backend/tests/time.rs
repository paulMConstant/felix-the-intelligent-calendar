use plan_backend::data::Time;
use std::panic::catch_unwind;

#[test]
fn std_eq() {
    assert!(Time::new(2, 30) == Time::new(2, 30));
    assert!(Time::new(2, 30) != Time::new(1, 0));
}

#[test]
fn std_ord() {
    // Compare with hours
    assert!(Time::new(2, 30) < Time::new(3, 0));
    // Compare with seconds
    assert!(Time::new(2, 30) < Time::new(2, 35));
}

#[test]
fn std_op() {
    // Add without wrap
    assert_eq!(Time::new(1, 0) + Time::new(1, 30), Time::new(2, 30));
    // Add with wrap
    assert_eq!(Time::new(1, 40) + Time::new(1, 30), Time::new(3, 10));
    // Sub without wrap
    assert_eq!(Time::new(2, 40) - Time::new(1, 30), Time::new(1, 10));
    // Sub with wrap
    assert_eq!(Time::new(10, 0) - Time::new(9, 20), Time::new(0, 40));

    // AddAssign without wrap
    let mut time = Time::new(1, 0);
    time += Time::new(1, 30);
    assert_eq!(time, Time::new(2, 30));
    // AddAssign with wrap
    time = Time::new(1, 40);
    time += Time::new(1, 30);
    assert_eq!(time, Time::new(3, 10));
    // SubAssign without wrap
    time = Time::new(2, 40);
    time -= Time::new(1, 30);
    assert_eq!(time, Time::new(1, 10));
    // SubAssign with wrap
    time = Time::new(10, 0);
    time -= Time::new(9, 20);
    assert_eq!(time, Time::new(0, 40));

    // Invalid operations
    assert!(catch_unwind(|| { Time::new(2, 35) + Time::new(23, 0) }).is_err());
    assert!(catch_unwind(|| { Time::new(2, 35) - Time::new(3, 0) }).is_err());
}

#[test]
fn std_sum() {
    let vec = vec![Time::new(0, 30); 3];
    assert_eq!(vec.iter().sum::<Time>(), Time::new(1, 30));
    assert_eq!(vec.iter().cloned().sum::<Time>(), Time::new(1, 30));
}

#[test]
fn add_hours() {
    let mut time = Time::new(1, 0);
    time.add_hours(3);
    assert!(time == Time::new(4, 0));
    time.add_hours(-1);
    assert!(time == Time::new(3, 0));

    // Invalid operations
    assert!(catch_unwind(|| { Time::new(1, 0).add_hours(-2) }).is_err());
    assert!(catch_unwind(|| { Time::new(1, 0).add_hours(23) }).is_err());
}

#[test]
fn add_minutes() {
    let mut time = Time::new(1, 0);
    time.add_minutes(55);
    assert_eq!(time, Time::new(1, 55));
    time.add_minutes(-25);
    assert_eq!(time, Time::new(1, 30));

    // Invalid operations
    assert!(catch_unwind(|| { Time::new(1, 0).add_minutes(61) }).is_err());
    assert!(catch_unwind(|| { Time::new(3, 0).add_minutes(-61) }).is_err());
}

#[test]
fn invalid_new() {
    assert!(catch_unwind(|| { Time::new(25, 0) }).is_err());
    assert!(catch_unwind(|| { Time::new(0, 60) }).is_err());
    assert!(catch_unwind(|| { Time::new(-1, 0) }).is_err());
    assert!(catch_unwind(|| { Time::new(0, -1) }).is_err());
}
