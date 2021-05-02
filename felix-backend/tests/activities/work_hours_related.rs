use felix_backend::data::{Time, TimeInterval};

use test_utils::{DataBuilder, Activity, test_ok};

#[test]
fn on_work_hours_changed_update_insertion_costs() {

}

/// Makes sure that activities which cannot be inserted
/// (e.g. duration == zero or no participants)
/// keep their empty insertion costs (Some([]) instead of None)
#[test]
fn on_work_hours_changed_check_not_insertable_activities_insertion_costs_untouched() {
    let entity = "Entity";    

    test_ok!(data,
            DataBuilder::new()
            .with_work_interval(TimeInterval::new(Time::new(9, 0), Time::new(12, 0)))
            .with_entity(entity)
            .with_activities(vec![
                 // Duration == zero
                 Activity {
                    entities: vec![entity],
                    duration: Time::new(0, 0),
                    ..Default::default()
                 },
                 // No participants
                 Activity {
                    entities: Vec::new(),
                    duration: Time::new(1, 0),
                    ..Default::default()
                 }]),
        {
            let id1 = data.activities_sorted()[0].id();
            let id2 = data.activities_sorted()[1].id();

            assert_eq!(data.activity(id1).insertion_costs(), Some(Vec::new()));
            assert_eq!(data.activity(id2).insertion_costs(), Some(Vec::new()));

            // Add new work interval and check that the insertion costs are still valid
            let new_time_interval = TimeInterval::new(Time::new(13, 0), Time::new(16, 0));
            data.add_work_interval(new_time_interval).expect("Could not add time interval");

            assert_eq!(data.activity(id1).insertion_costs(), Some(Vec::new()));
            assert_eq!(data.activity(id2).insertion_costs(), Some(Vec::new()));
        }
    );
}
