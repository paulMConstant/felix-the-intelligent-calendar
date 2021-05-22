use felix_backend::data::TimeInterval;
use felix_backend::Time;
use test_utils::{Activity, DataBuilder};

#[test]
fn simple_add_interval() {
    let interval = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    test_ok!(data, DataBuilder::new(), {
        data.add_work_interval(interval)
            .expect("Could not add simple work interval");
        assert_eq!(
            data.work_hours()[0],
            interval,
            "Interval was not added correctly"
        );
    });
}

#[test]
fn add_overlapping_interval() {
    let interval = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    let overlap = TimeInterval::new(Time::new(8, 0), Time::new(9, 0));
    test_err!(
        data,
        DataBuilder::new().with_work_interval(interval),
        data.add_work_interval(overlap),
        "The given interval overlaps with others.",
        "Could add overlapping interval"
    );
}

/// It should be possible to add adjacent work intervals (end == beginning of other)
#[test]
fn add_adjacent_interval() {
    let interval = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    let adjacent = TimeInterval::new(Time::new(12, 0), Time::new(14, 0));
    test_ok!(
        data,
        DataBuilder::new().with_work_interval(interval),
        data.add_work_interval(adjacent)
            .expect("Could not add adjacent interval")
    );
}

#[test]
fn add_interals_check_sorted() {
    let interval1 = TimeInterval::new(Time::new(8, 0), Time::new(10, 0));
    let interval2 = TimeInterval::new(Time::new(11, 0), Time::new(12, 0));
    let interval3 = TimeInterval::new(Time::new(14, 0), Time::new(17, 0));

    test_ok!(
        data,
        DataBuilder::new().with_work_intervals(vec![interval1, interval3, interval2]),
        {
            let work_hours = data.work_hours();
            assert_eq!(work_hours[0], interval1, "Intervals are not sorted");
            assert_eq!(work_hours[1], interval2, "Intervals are not sorted");
            assert_eq!(work_hours[2], interval3, "Intervals are not sorted");
        }
    );
}

#[test]
fn simple_remove_interval() {
    // Add two intervals to check later that we removed the right one
    let interval1 = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    let interval2 = TimeInterval::new(Time::new(17, 0), Time::new(18, 0));
    test_ok!(
        data,
        DataBuilder::new().with_work_intervals(vec![interval1, interval2]),
        {
            data.remove_work_interval(interval1)
                .expect("Could not remove time interval");
            let work_hours = data.work_hours();
            assert_eq!(work_hours.len(), 1, "Did not remove time interval");
            assert_eq!(work_hours[0], interval2, "Removed wrong time interval");
        }
    );
}

#[test]
fn remove_nonexistent_time_interval() {
    let interval = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    test_err!(
        data,
        DataBuilder::new(),
        data.remove_work_interval(interval),
        "The interval '08:00 - 12:00' does not exist.",
        "Could remove time interval even though there are none"
    );
}

#[test]
fn remove_interval_wrong_beginning() {
    let interval = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    let same_end = TimeInterval::new(Time::new(9, 0), Time::new(12, 0));
    test_err!(
        data,
        DataBuilder::new().with_work_interval(interval),
        data.remove_work_interval(same_end),
        "The interval '09:00 - 12:00' does not exist.",
        "Could remove interval with same end but different beginning"
    );
}

#[test]
fn remove_interval_wrong_end() {
    let interval = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    let same_beginning = TimeInterval::new(Time::new(8, 0), Time::new(11, 0));
    test_err!(
        data,
        DataBuilder::new().with_work_interval(interval),
        data.remove_work_interval(same_beginning),
        "The interval '08:00 - 11:00' does not exist.",
        "Could remove interval with same beginning but different end"
    );
}

/// We must make sure that entities always have more time in their schedule
/// than the time their activities take
#[test]
fn remove_time_interval_not_enough_time_for_activities() {
    let interval = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    let entity = "Entity";
    test_err!(
        data,
        DataBuilder::new()
            .with_work_interval(interval)
            .with_entity(entity)
            .with_activity(Activity {
                entities: vec![entity],
                duration: Time::new(1, 0),
                ..Default::default()
            }),
        data.remove_work_interval(interval),
        "Entity will not have enough time if their work hours are shortened.",
        "Could remove interval which led to entity not having enough time"
    );
}

#[test]
fn simple_update_interval() {
    let interval = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    let new_interval = TimeInterval::new(Time::new(8, 0), Time::new(11, 0));
    test_ok!(data, DataBuilder::new().with_work_interval(interval), {
        data.update_work_interval(interval, new_interval)
            .expect("Could not update time interval");
        let interval = data.work_hours()[0];
        assert_eq!(interval, new_interval, "Interval was not updated");
    });
}

#[test]
fn update_nonexistent_interval() {
    let interval = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    test_err!(
        data,
        DataBuilder::new(),
        data.update_work_interval(interval, interval),
        "The interval '08:00 - 12:00' does not exist.",
        "Could update nonexistent work interval"
    );
}

#[test]
fn update_time_interval_not_enough_time_for_activities() {
    let interval = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    let entity = "Entity";
    test_err!(
        data,
        DataBuilder::new()
            .with_work_interval(interval)
            .with_entity(entity)
            .with_activity(Activity {
                duration: Time::new(4, 0),
                entities: vec![entity],
                ..Default::default()
            }),
        {
            let new_interval_too_short = TimeInterval::new(Time::new(9, 0), Time::new(12, 0));
            data.update_work_interval(interval, new_interval_too_short)
        },
        "Entity will not have enough time if their work hours are shortened.",
        "Could update interval which left entity with not enough time"
    );
}

#[test]
fn update_time_interval_overlaps() {
    let interval1 = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    let interval2 = TimeInterval::new(Time::new(13, 0), Time::new(15, 0));
    let new_interval_overlaps = TimeInterval::new(Time::new(10, 0), Time::new(15, 0));
    test_err!(
        data,
        DataBuilder::new().with_work_intervals(vec![interval1, interval2]),
        data.update_work_interval(interval1, new_interval_overlaps),
        "The given interval overlaps with others.",
        "Could add overlapping work interval"
    );
}

#[test]
fn update_time_interval_check_sorted() {
    let interval1 = TimeInterval::new(Time::new(8, 0), Time::new(10, 0));
    let interval2 = TimeInterval::new(Time::new(11, 0), Time::new(12, 0));
    let interval3 = TimeInterval::new(Time::new(14, 0), Time::new(15, 0));

    test_ok!(
        data,
        DataBuilder::new().with_work_intervals(vec![interval1, interval2]),
        {
            data.update_work_interval(interval1, interval3)
                .expect("Could not update work interval");

            let intervals = data.work_hours();
            assert_eq!(intervals[0], interval2, "Work hours are not sorted");
            assert_eq!(intervals[1], interval3, "Work hours are not sorted");
        }
    );
}

/// Tests that work hours cannot be added while activities are inserted
#[test]
fn add_work_hours_with_inserted_activities() {
    let entity = "Jean";
    let work_interval1 = TimeInterval::new(Time::new(8, 0), Time::new(11, 0));
    let work_interval2 = TimeInterval::new(Time::new(14, 0), Time::new(16, 0));
    test_err!(
        data,
        DataBuilder::new()
            .with_work_interval(work_interval1)
            .with_entity(entity)
            .with_activity(Activity {
                entities: vec![entity],
                insertion_time: Some(Time::new(8, 0)),
                ..Default::default()
            }),
        {
            data.add_work_interval(work_interval2)
        },
        "Work hours cannot be modified while an activity is inserted.",
        "Could add work hours with one inserted activity"
    );
}

/// Tests that work hours cannot be removed while activities are inserted
#[test]
fn remove_work_hours_with_inserted_activities() {
    let entity = "Jean";
    let work_interval1 = TimeInterval::new(Time::new(8, 0), Time::new(11, 0));
    let work_interval2 = TimeInterval::new(Time::new(14, 0), Time::new(16, 0));
    test_err!(
        data,
        DataBuilder::new()
            .with_work_intervals(vec![work_interval1, work_interval2])
            .with_entity(entity)
            .with_activity(Activity {
                entities: vec![entity],
                insertion_time: Some(Time::new(8, 0)),
                ..Default::default()
            }),
        {
            data.remove_work_interval(work_interval2)
        },
        "Work hours cannot be modified while an activity is inserted.",
        "Could remove  work hours with one inserted activity"
    );
}

/// Tests that work hours cannot be updated while activities are inserted
#[test]
fn update_work_hours_with_inserted_activities() {
    let entity = "Jean";
    let work_interval1 = TimeInterval::new(Time::new(8, 0), Time::new(11, 0));
    let work_interval2 = TimeInterval::new(Time::new(14, 0), Time::new(16, 0));
    test_err!(
        data,
        DataBuilder::new()
            .with_work_intervals(vec![work_interval1, work_interval2])
            .with_entity(entity)
            .with_activity(Activity {
                entities: vec![entity],
                insertion_time: Some(Time::new(8, 0)),
                ..Default::default()
            }),
        {
            let new_work_interval = TimeInterval::new(Time::new(14, 0), Time::new(15, 0));
            data.update_work_interval(work_interval2, new_work_interval)
        },
        "Work hours cannot be modified while an activity is inserted.",
        "Could update work hours with one inserted activity"
    );
}
