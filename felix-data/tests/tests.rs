#[macro_use]
extern crate felix_test_utils;

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
    mod custom_work_hours;
    mod work_hours;
}
