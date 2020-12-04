#[macro_use]
extern crate assert_not_modified;

mod activities {
    mod activities;
    mod entities_related;
    mod groups_related;
}

mod entities {
    mod entities;
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
