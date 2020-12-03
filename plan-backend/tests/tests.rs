#[macro_use]
extern crate assert_not_modified;

mod activities {
    mod basic;
    mod entities_related;
    mod groups_related;
}

mod entities {
    mod basic;
    mod work_hours_related;
}

mod groups {
    mod activities_related;
    mod basic;
    mod entities_related;
}

mod work_hours {
    mod time;
    mod time_interval;
    mod work_hours;
}
