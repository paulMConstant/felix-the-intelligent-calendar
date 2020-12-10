#[macro_use]
extern crate test_utils;

#[macro_use]
extern crate assert_not_modified;

mod activities {
    mod activities;
    mod entities_related;
    mod groups_related;
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
}

mod errors {
    mod already_in;
    mod duration_too_short;
    mod does_not_exist;
    mod name_taken;
    mod empty_name;
    mod not_in;
    mod interval_overlaps;
    mod not_enough_time;
}
