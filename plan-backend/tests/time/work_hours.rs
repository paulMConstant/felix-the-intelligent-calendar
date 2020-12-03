use plan_backend::data::{Data, Time, TimeInterval};

#[test]
fn simple_add_interval() {
    let mut data = Data::new();

    let interval = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    data.add_work_interval(interval)
        .expect("Could not add simple work interval");
    assert_eq!(
        data.work_hours()[0],
        interval,
        "Interval was not added correctly"
    );
}

#[test]
fn add_overlapping_interval() {
    let mut data = Data::new();

    let interval = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    data.add_work_interval(interval)
        .expect("Could not add simple work interval");

    let overlap = TimeInterval::new(Time::new(8, 0), Time::new(9, 0));
    assert_not_modified!(data, {
        assert_eq!(
            data.add_work_interval(overlap),
            Err("The given interval overlaps with other work intervals.".to_owned()),
            "Could add overlapping interval"
        );
    });
}

/// It should be possible to add adjacent work intervals (end == beginning of other)
#[test]
fn add_adjacent_interval() {
    let mut data = Data::new();

    let interval = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    data.add_work_interval(interval)
        .expect("Could not add simple work interval");

    let adjacent = TimeInterval::new(Time::new(12, 0), Time::new(14, 0));
    data.add_work_interval(adjacent)
        .expect("Could not add adjacent interval");
}

#[test]
fn add_interals_check_sorted() {
    let mut data = Data::new();

    let interval1 = TimeInterval::new(Time::new(8, 0), Time::new(10, 0));
    let interval2 = TimeInterval::new(Time::new(11, 0), Time::new(12, 0));
    let interval3 = TimeInterval::new(Time::new(14, 0), Time::new(17, 0));

    data.add_work_interval(interval2)
        .expect("Could not add simple work interval");
    data.add_work_interval(interval3)
        .expect("Could not add simple work interval");
    data.add_work_interval(interval1)
        .expect("Could not add simple work interval");

    let work_hours = data.work_hours();
    assert_eq!(work_hours[0], interval1, "Intervals are not sorted");
    assert_eq!(work_hours[1], interval2, "Intervals are not sorted");
    assert_eq!(work_hours[2], interval3, "Intervals are not sorted");
}

#[test]
fn simple_remove_interval() {
    let mut data = Data::new();

    // Add two intervals to check later that we removed the right one
    let interval1 = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    let interval2 = TimeInterval::new(Time::new(17, 0), Time::new(18, 0));

    data.add_work_interval(interval2)
        .expect("Could not add simple work interval");
    data.add_work_interval(interval1)
        .expect("Could not add simple work interval");

    data.remove_work_interval(interval1)
        .expect("Could not remove time interval");
    let work_hours = data.work_hours();
    assert_eq!(work_hours.len(), 1, "Did not remove time interval");
    assert_eq!(work_hours[0], interval2, "Removed wrong time interval");
}

#[test]
fn remove_wrong_time_interval() {
    let mut data = Data::new();

    let interval = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    assert_not_modified!(data, {
        assert_eq!(
            data.remove_work_interval(interval),
            Err("The given time interval was not found.".to_owned()),
            "Could remove time interval even though there are none"
        );
    });

    data.add_work_interval(interval)
        .expect("Could not add simple work interval");

    assert_not_modified!(data, {
        let same_beginning = TimeInterval::new(Time::new(8, 0), Time::new(11, 0));
        assert_eq!(
            data.remove_work_interval(same_beginning),
            Err("The given time interval was not found.".to_owned()),
            "Could remove interval with same beginning but different end"
        );

        let same_end = TimeInterval::new(Time::new(9, 0), Time::new(12, 0));
        assert_eq!(
            data.remove_work_interval(same_end),
            Err("The given time interval was not found.".to_owned()),
            "Could remove interval with same end but different beginning"
        );
    });
}

/// We must make sure that entities always have more time in their schedule
/// than the time their activities take
#[test]
fn remove_time_interval_not_enough_time_for_activities() {
    let mut data = Data::new();

    let interval = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    data.add_work_interval(interval)
        .expect("Could not add time interval");

    let id = data
        .add_activity("Name")
        .expect("Could not add activity")
        .id();
    let name = data
        .add_entity("Entity")
        .expect("Could not add entity")
        .name();
    data.add_entity_to_activity(id, name)
        .expect("Could not add entity");
    data.set_activity_duration(id, Time::new(1, 0))
        .expect("Could not set activity duration");

    assert_not_modified!(data, {
        assert_eq!(data.remove_work_interval(interval), Err("Entity does not have enough time left. Free up their time before removing the work interval.".to_owned()),
        "Could remove interval which led to entity not having enough time");
    });
}

#[test]
fn simple_update_interval() {
    let mut data = Data::new();

    let interval = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    data.add_work_interval(interval)
        .expect("Could not add time interval");

    let new_interval = TimeInterval::new(Time::new(8, 0), Time::new(11, 0));
    data.update_work_interval(interval, new_interval)
        .expect("Could not update time interval");
    let interval = data.work_hours()[0];
    assert_eq!(interval, new_interval, "Interval was not updated");
}

#[test]
fn update_nonexistent_interval() {
    let mut data = Data::new();

    let interval = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    assert_not_modified!(data, {
        assert_eq!(
            data.update_work_interval(interval, interval),
            Err("The given time interval was not found.".to_owned()),
            "Could update nonexistent work interval"
        );
    });
}

#[test]
fn update_time_interval_not_enough_time_for_activities() {
    let mut data = Data::new();

    let interval = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    data.add_work_interval(interval)
        .expect("Could not add interval");
    let id = data
        .add_activity("Name")
        .expect("Could not add activity")
        .id();
    let name = data
        .add_entity("Name")
        .expect("Could not add entity")
        .name();
    let duration = Time::new(4, 0);
    data.set_activity_duration(id, duration)
        .expect("Could not set activity duration");
    data.add_entity_to_activity(id, name)
        .expect("Could not add entity");

    let new_interval_too_short = TimeInterval::new(Time::new(9, 0), Time::new(12, 0));
    assert!(
        new_interval_too_short.duration() < duration,
        "Test is pointless : there is enough time for the activities"
    );
    assert_not_modified!(data, {
        assert_eq!(
            data.update_work_interval(interval, new_interval_too_short),
            Err("Name does not have enough free time to reduce this interval.".to_owned()),
            "Could update interval which left entity with not enough time"
        );
    });
}

#[test]
fn update_time_interval_overlaps() {
    let mut data = Data::new();

    let interval1 = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    let interval2 = TimeInterval::new(Time::new(13, 0), Time::new(15, 0));

    data.add_work_interval(interval1)
        .expect("Could not add interval");
    data.add_work_interval(interval2)
        .expect("Could not add interval");
    let new_interval_overlaps = TimeInterval::new(Time::new(10, 0), Time::new(15, 0));
    assert_not_modified!(data, {
        assert_eq!(
            data.update_work_interval(interval1, new_interval_overlaps),
            Err("The given interval overlaps with other work intervals.".to_owned()),
            "Could add overlapping work interval"
        );
    });
}

#[test]
fn update_time_interval_check_sorted() {
    let mut data = Data::new();

    let interval1 = TimeInterval::new(Time::new(8, 0), Time::new(10, 0));
    let interval2 = TimeInterval::new(Time::new(11, 0), Time::new(12, 0));
    let interval3 = TimeInterval::new(Time::new(14, 0), Time::new(15, 0));

    data.add_work_interval(interval1)
        .expect("Could not add interval");
    data.add_work_interval(interval2)
        .expect("Could not add interval");
    data.update_work_interval(interval1, interval3)
        .expect("Could not update work interval");

    let intervals = data.work_hours();
    assert_eq!(intervals[0], interval2, "Work hours are not sorted");
    assert_eq!(intervals[1], interval3, "Work hours are not sorted");
}
