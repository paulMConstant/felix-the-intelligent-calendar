#[macro_use]
extern crate test_utils;

#[macro_use]
extern crate assert_not_modified;

mod activities {
    mod activities;
    mod entities_related;
    mod groups_related;
    mod work_hours_related;
}

mod entities {
    mod activities_related;
    mod entities;
    mod groups_related;
    mod work_hours_related;
}

mod groups {
    mod activities_related;
    mod entities_related;
    mod groups;
}

mod time {
    mod time;
    mod time_interval;
    mod work_hours;
    mod custom_work_hours;
}

mod errors {
    mod add_entity_to_inserted_activity_invalid_spot;
    mod already_in;
    mod change_work_hours_while_activity_inserted;
    mod does_not_exist;
    mod duration_too_short;
    mod empty_name;
    mod interval_overlaps;
    mod invalid_insertion;
    mod invalid_interval;
    mod name_taken;
    mod not_enough_time;
    mod not_in;
}
