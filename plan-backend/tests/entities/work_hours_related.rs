use plan_backend::data::{Data, Time, TimeInterval};

#[test]
fn simple_add_custom_work_interval() {
    let mut data = Data::new();

    let name = data
        .add_entity("Name")
        .expect("Could not add entity")
        .name();

    let interval = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    data.add_custom_work_interval_for(name.clone(), interval)
        .expect("Could not add custom work interval");

    let custom_work_hours = data
        .entity(name.clone())
        .expect("Could not get entity by name")
        .custom_work_hours();

    assert_eq!(
        custom_work_hours.len(),
        1,
        "Custom work interval was not added"
    );
    assert_eq!(
        custom_work_hours[0], interval,
        "Custom work interval was not added right"
    );
}

#[test]
fn add_custom_work_interval_nonexistent_entity() {
    let mut data = Data::new();

    let interval = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    assert_not_modified!(data, {
        assert_eq!(
            data.add_custom_work_interval_for("Name", interval),
            Err("The entity 'Name' does not exist.".to_owned()),
            "Could add custom work interval for nonexistent entity"
        );
    });
}

#[test]
fn add_overlapping_custom_work_interval() {
    let mut data = Data::new();

    let name = data
        .add_entity("Name")
        .expect("Could not add entity")
        .name();

    let interval = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    data.add_custom_work_interval_for(name.clone(), interval)
        .expect("Could not add custom work interval");

    let overlap = TimeInterval::new(Time::new(8, 0), Time::new(9, 0));
    assert_not_modified!(data, {
        assert_eq!(
            data.add_custom_work_interval_for(name, overlap),
            Err("The given interval overlaps with other work intervals.".to_owned()),
            "Could add overlapping interval"
        )
    });
}

#[test]
fn add_custom_work_interval_not_enough_free_time() {
    let mut data = Data::new();

    let name = data
        .add_entity("Name")
        .expect("Could not add entity")
        .name();
    let interval = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    let id = data
        .add_activity("Name")
        .expect("Could not add activity")
        .id();
    let duration = Time::new(4, 0);

    data.add_work_interval(interval)
        .expect("Could not add work interval");
    data.set_activity_duration(id, duration)
        .expect("Could not set activity duration");
    data.add_entity_to_activity(id, name.clone())
        .expect("Could not add entity");

    let custom_interval_too_short = TimeInterval::new(Time::new(8, 0), Time::new(10, 0));
    assert_not_modified!(data, {
        assert_eq!(
        data.add_custom_work_interval_for(name, custom_interval_too_short),
        Err(
            "Name will not have enough time for their activities using these custom work hours."
                .to_owned()
        ),
        "Coud add custom work interval which led to entity not having enough time"
    );
    });
}

/// It should be possible to add adjacent work intervals (end == beginning of other)
#[test]
fn add_custom_adjacent_work_interval() {
    let mut data = Data::new();

    let name = data
        .add_entity("Name")
        .expect("Could not add entity")
        .name();
    let interval = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    let adjacent = TimeInterval::new(Time::new(12, 0), Time::new(14, 0));

    data.add_custom_work_interval_for(name.clone(), interval)
        .expect("Could not add custom work interval");
    data.add_custom_work_interval_for(name, adjacent)
        .expect("Could not add custom adjacent work interval");
}

#[test]
fn check_custom_work_intervals_sorted() {
    let mut data = Data::new();

    let name = data
        .add_entity("Name")
        .expect("Could not add entity")
        .name();
    let interval1 = TimeInterval::new(Time::new(8, 0), Time::new(10, 0));
    let interval2 = TimeInterval::new(Time::new(11, 0), Time::new(12, 0));
    let interval3 = TimeInterval::new(Time::new(14, 0), Time::new(17, 0));

    data.add_custom_work_interval_for(name.clone(), interval2)
        .expect("Could not add custom work interval");
    data.add_custom_work_interval_for(name.clone(), interval3)
        .expect("Could not add custom work interval");
    data.add_custom_work_interval_for(name.clone(), interval1)
        .expect("Could not add custom work interval");

    let work_hours = data
        .entity(name)
        .expect("Could not get entity by name")
        .custom_work_hours();
    assert_eq!(work_hours[0], interval1, "Intervals are not sorted");
    assert_eq!(work_hours[1], interval2, "Intervals are not sorted");
    assert_eq!(work_hours[2], interval3, "Intervals are not sorted");
}

#[test]
fn simple_remove_custom_work_interval() {
    let mut data = Data::new();

    let name = data
        .add_entity("Name")
        .expect("Could not add entity")
        .name();

    // Add two intervals to check that the right one is removed
    let interval1 = TimeInterval::new(Time::new(8, 0), Time::new(10, 0));
    let interval2 = TimeInterval::new(Time::new(11, 0), Time::new(12, 0));

    data.add_custom_work_interval_for(name.clone(), interval2)
        .expect("Could not add custom work interval");
    data.add_custom_work_interval_for(name.clone(), interval1)
        .expect("Could not add custom work interval");

    let work_hours = data
        .entity(name.clone())
        .expect("Could not get entity by name")
        .custom_work_hours();
    assert_eq!(work_hours.len(), 2, "Custom work hours were not added");

    data.remove_custom_work_interval_for(name.clone(), interval1)
        .expect("Could not remove custom work interval");
    let work_hours = data
        .entity(name)
        .expect("Could not get entity by name")
        .custom_work_hours();
    assert_eq!(work_hours.len(), 1, "Custom work interval was not removed");
    assert_eq!(
        work_hours[0], interval2,
        "The wrong time interval was removed"
    );
}

#[test]
fn remove_custom_work_interval_nonexistent_entity() {
    let mut data = Data::new();

    let interval = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    assert_not_modified!(data, {
        assert_eq!(
            data.remove_custom_work_interval_for("Name", interval),
            Err("The entity 'Name' does not exist.".to_owned()),
            "Could add custom work interval for nonexistent entity"
        );
    });
}

#[test]
fn remove_custom_work_interval_not_enough_free_time() {
    let mut data = Data::new();

    let interval = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    let name = data
        .add_entity("Name")
        .expect("Could not add entity")
        .name();
    let id = data
        .add_activity("Name")
        .expect("Could not add activity")
        .id();
    let duration = Time::new(3, 0);
    data.set_activity_duration(id, duration)
        .expect("Could not set activity duration");

    data.add_custom_work_interval_for(name.clone(), interval)
        .expect("Could not add custom work interval.");
    data.add_entity_to_activity(id, name.clone())
        .expect("Could not add entity.");
    assert_not_modified!(data, {
        assert_eq!(
        data.remove_custom_work_interval_for(name, interval),
        Err(
            "Name will not have enough time for their activities once this interval is removed."
                .to_owned()
        ),
        "Could remove custom work interval which led to entity having not enough free time",
    )
    });
}

#[test]
fn remove_invalid_custom_work_interval() {
    let mut data = Data::new();

    let name = data
        .add_entity("Name")
        .expect("Could not add entity")
        .name();
    let interval = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    assert_not_modified!(data, {
        assert_eq!(
            data.remove_custom_work_interval_for(name.clone(), interval),
            Err("The given time interval was not found.".to_owned()),
            "Could remove time interval even though there are none"
        );
    });

    data.add_custom_work_interval_for(name.clone(), interval)
        .expect("Could not add custom work interval");

    assert_not_modified!(data, {
        let same_beginning = TimeInterval::new(Time::new(8, 0), Time::new(11, 0));
        assert_eq!(
            data.remove_custom_work_interval_for(name.clone(), same_beginning),
            Err("The given time interval was not found.".to_owned()),
            "Could remove interval with same beginning but different end"
        );

        let same_end = TimeInterval::new(Time::new(9, 0), Time::new(12, 0));
        assert_eq!(
            data.remove_custom_work_interval_for(name, same_end),
            Err("The given time interval was not found.".to_owned()),
            "Could remove interval with same end but different beginning"
        );
    });
}

#[test]
fn simple_update_custom_interval() {
    let mut data = Data::new();

    let name = data
        .add_entity("Name")
        .expect("Could not add entity")
        .name();
    let interval = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    data.add_custom_work_interval_for(name.clone(), interval)
        .expect("Could not add custom time interval");

    let new_interval = TimeInterval::new(Time::new(8, 0), Time::new(11, 0));
    data.update_custom_work_interval_for(name.clone(), interval, new_interval)
        .expect("Could not update time interval");

    let interval = data
        .work_hours_of(name)
        .expect("Could not find work hours of entity")[0];
    assert_eq!(interval, new_interval, "Interval was not updated");
}

#[test]
fn update_nonexistent_custom_interval() {
    let mut data = Data::new();

    let name = data
        .add_entity("Name")
        .expect("Could not add entity")
        .name();
    let interval = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    assert_not_modified!(data, {
        assert_eq!(
            data.update_custom_work_interval_for(name, interval, interval),
            Err("The given time interval was not found.".to_owned()),
            "Could update nonexistent work interval"
        );
    });
}
#[test]
fn update_custom_interval_nonexistent_entity() {
    let mut data = Data::new();

    let interval = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    assert_not_modified!(data, {
        assert_eq!(
            data.update_custom_work_interval_for("Does not exist", interval, interval),
            Err("The entity 'Does Not Exist' does not exist.".to_owned()),
            "Could update custom work interval for nonexistent entity"
        );
    });
}

#[test]
fn update_custom_time_interval_not_enough_time_for_activities() {
    let mut data = Data::new();

    let interval = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    let name = data
        .add_entity("Name")
        .expect("Could not add entity")
        .name();
    data.add_custom_work_interval_for(name.clone(), interval)
        .expect("Could not add custom work interval");
    let id = data
        .add_activity("Name")
        .expect("Could not add activity")
        .id();
    let duration = Time::new(4, 0);
    data.set_activity_duration(id, duration)
        .expect("Could not set activity duration");
    data.add_entity_to_activity(id, name.clone())
        .expect("Could not add entity");

    let new_interval_too_short = TimeInterval::new(Time::new(9, 0), Time::new(12, 0));
    assert!(
        new_interval_too_short.duration() < duration,
        "Test is pointless : there is enough time for the activities"
    );
    assert_not_modified!(data, {
        assert_eq!(
            data.update_custom_work_interval_for(name, interval, new_interval_too_short),
            Err("Name does not have enough free time to reduce this interval.".to_owned()),
            "Could update interval which left entity with not enough time"
        );
    });
}

#[test]
fn update_custom_time_interval_overlaps() {
    let mut data = Data::new();

    let name = data
        .add_entity("Name")
        .expect("Could not add entity")
        .name();
    let interval1 = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    let interval2 = TimeInterval::new(Time::new(13, 0), Time::new(15, 0));

    data.add_custom_work_interval_for(name.clone(), interval1)
        .expect("Could not add custom interval");
    data.add_custom_work_interval_for(name.clone(), interval2)
        .expect("Could not add custom interval");
    let new_interval_overlaps = TimeInterval::new(Time::new(10, 0), Time::new(15, 0));
    assert_not_modified!(data, {
        assert_eq!(
            data.update_custom_work_interval_for(name, interval1, new_interval_overlaps),
            Err("The given interval overlaps with other work intervals.".to_owned()),
            "Could add overlapping work interval"
        );
    });
}

#[test]
fn update_custom_time_interval_check_sorted() {
    let mut data = Data::new();

    let name = data
        .add_entity("Name")
        .expect("Could not add entity")
        .name();
    let interval1 = TimeInterval::new(Time::new(8, 0), Time::new(10, 0));
    let interval2 = TimeInterval::new(Time::new(11, 0), Time::new(12, 0));
    let interval3 = TimeInterval::new(Time::new(14, 0), Time::new(15, 0));

    data.add_custom_work_interval_for(name.clone(), interval1)
        .expect("Could not add custom interval");
    data.add_custom_work_interval_for(name.clone(), interval2)
        .expect("Could not add custom interval");

    data.update_custom_work_interval_for(name.clone(), interval1, interval3)
        .expect("Could not add custom interval");

    let intervals = data
        .work_hours_of(name)
        .expect("Could not get work hours of entity");
    assert_eq!(intervals[0], interval2, "Work hours are not sorted");
    assert_eq!(intervals[1], interval3, "Work hours are not sorted");
}

// *** Get work hours of entity ***
#[test]
fn work_hours_of_without_custom_work_hours() {
    let mut data = Data::new();

    let interval = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    data.add_work_interval(interval)
        .expect("Could not add work interval");

    let name = data
        .add_entity("Name")
        .expect("Could not add entity")
        .name();
    let entity_work_hours = data
        .work_hours_of(name)
        .expect("Could not fetch work hours of entity");
    let general_work_hours = data.work_hours();

    assert_eq!(
        entity_work_hours, general_work_hours,
        "Work hours of does not return the general work hours where it should"
    );
}

#[test]
fn work_hours_of_with_custom_work_hours() {
    let mut data = Data::new();

    let interval = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    let name = data
        .add_entity("Name")
        .expect("Could not add entity")
        .name();
    data.add_custom_work_interval_for(name.clone(), interval)
        .expect("Could not add custom work interval");

    let entity_work_hours = data
        .work_hours_of(name.clone())
        .expect("Could not fetch work hours of entity");
    let custom_work_hours = data
        .entity(name)
        .expect("Could not get entity by name")
        .custom_work_hours();

    assert_eq!(
        entity_work_hours, custom_work_hours,
        "Work hours of does not return the custom work hours where it should"
    );
}

// *** Free time ***
#[test]
fn free_time_without_activities_with_global_work_hours() {
    let mut data = Data::new();

    let interval1 = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    let interval2 = TimeInterval::new(Time::new(13, 0), Time::new(18, 0));
    data.add_work_interval(interval1)
        .expect("Could not add work interval");
    data.add_work_interval(interval2)
        .expect("Could not add work interval");

    let name = data
        .add_entity("Name")
        .expect("Could not add entity")
        .name();
    let free_time = data
        .free_time_of(name)
        .expect("Could not fetch free time of entity");
    let expected_free_time = interval1.duration() + interval2.duration();

    assert_eq!(
        free_time, expected_free_time,
        "Free time was not calculated right"
    );
}

#[test]
fn free_time_without_activities_with_custom_work_hours() {
    let mut data = Data::new();

    let interval1 = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    let interval2 = TimeInterval::new(Time::new(13, 0), Time::new(18, 0));
    let name = data
        .add_entity("Name")
        .expect("Could not add entity")
        .name();

    data.add_custom_work_interval_for(name.clone(), interval1)
        .expect("Could not add custom work interval");
    data.add_custom_work_interval_for(name.clone(), interval2)
        .expect("Could not add custom work interval");

    let free_time = data
        .free_time_of(name)
        .expect("Could not fetch free time of entity");
    let expected_free_time = interval1.duration() + interval2.duration();

    assert_eq!(
        free_time, expected_free_time,
        "Free time was not calculated right"
    );
}

#[test]
fn free_time_with_activities_with_global_work_hours() {
    let mut data = Data::new();

    let interval1 = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    let interval2 = TimeInterval::new(Time::new(13, 0), Time::new(18, 0));
    data.add_work_interval(interval1)
        .expect("Could not add work interval");
    data.add_work_interval(interval2)
        .expect("Could not add work interval");

    let name = data
        .add_entity("Name")
        .expect("Could not add entity")
        .name();

    let id1 = data
        .add_activity("Activity")
        .expect("Could not add activity")
        .id();
    let id2 = data
        .add_activity("Activity2")
        .expect("Could not add activity")
        .id();
    data.set_activity_duration(id1, Time::new(1, 0))
        .expect("Could not set activity duration");
    data.set_activity_duration(id2, Time::new(1, 30))
        .expect("Could not set activity duration");

    data.add_entity_to_activity(id1, name.clone())
        .expect("Could not add entity");
    data.add_entity_to_activity(id2, name.clone())
        .expect("Could not add entity");

    let free_time = data
        .free_time_of(name)
        .expect("Could not fetch free time of entity");
    let expected_free_time = interval1.duration() + interval2.duration()
        - data
            .activity(id1)
            .expect("Could not get activity by id")
            .duration()
        - data
            .activity(id2)
            .expect("Could not get activity by id")
            .duration();

    assert_eq!(
        free_time, expected_free_time,
        "Free time was not calculated right"
    );
}

#[test]
fn free_time_with_activities_with_custom_work_hours() {
    let mut data = Data::new();

    let interval1 = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    let interval2 = TimeInterval::new(Time::new(13, 0), Time::new(18, 0));
    let name = data
        .add_entity("Name")
        .expect("Could not add entity")
        .name();

    data.add_custom_work_interval_for(name.clone(), interval1)
        .expect("Could not add custom work interval");
    data.add_custom_work_interval_for(name.clone(), interval2)
        .expect("Could not add custom work interval");

    let id1 = data
        .add_activity("Activity")
        .expect("Could not add activity")
        .id();
    let id2 = data
        .add_activity("Activity2")
        .expect("Could not add activity")
        .id();
    data.set_activity_duration(id1, Time::new(1, 0))
        .expect("Could not set activity duration");
    data.set_activity_duration(id2, Time::new(1, 30))
        .expect("Could not set activity duration");

    data.add_entity_to_activity(id1, name.clone())
        .expect("Could not add entity");
    data.add_entity_to_activity(id2, name.clone())
        .expect("Could not add entity");

    let free_time = data
        .free_time_of(name)
        .expect("Could not fetch free time of entity");
    let expected_free_time = interval1.duration() + interval2.duration()
        - data
            .activity(id1)
            .expect("Could not get activity by id")
            .duration()
        - data
            .activity(id2)
            .expect("Could not get activity by id")
            .duration();

    assert_eq!(
        free_time, expected_free_time,
        "Free time was not calculated right"
    );
}

#[test]
fn free_time_of_wrong_entity() {
    let data = Data::new();

    assert_eq!(
        data.free_time_of("Does not exist"),
        Err("The entity 'Does Not Exist' does not exist.".to_owned()),
        "Could get free time of nonexistent entity"
    );
}
