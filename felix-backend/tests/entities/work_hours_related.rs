use felix_backend::{Time, TimeInterval};
use felix_test_utils::DataBuilder;

#[test]
fn rename_entity_keeps_custom_work_hours() { 
    let name = "Paul";
    let new_name = "Test";
    let custom_time_interval = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    test_ok!(
        data,
        DataBuilder::new()
        .with_entity(name)
        .with_custom_work_interval_for(name, custom_time_interval),
    {
        let custom_work_hours = data
            .custom_work_hours_of(name)
           .expect("Could not get entity by name");
        assert_eq!(custom_work_hours.len(), 1);
        assert_eq!(custom_work_hours[0], custom_time_interval);

        data.set_entity_name(name, new_name).expect("Could not rename entity");

        // Check that the entity keeps its work hours after a rename
        let custom_work_hours = data
            .custom_work_hours_of(new_name)
           .expect("Could not get entity by name");
        assert_eq!(custom_work_hours.len(), 1);
        assert_eq!(custom_work_hours[0], custom_time_interval);
    });
}
 #[test]
fn remove_entity_resets_custom_work_hours_for_future_entities() {
    let name = "Paul";
    let custom_time_interval = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));
    test_ok!(
        data,
        DataBuilder::new()
        .with_entity(name)
        .with_custom_work_interval_for(name, custom_time_interval),
    {
        data.remove_entity(name).expect("Could not remove entity");
        // Check that the custom work hours for this name have been erased
        // Create a new entity with the same name and check that they don't have any
        data.add_entity(name).expect("Could not add entity");
        assert!(data.custom_work_hours_of(name)
                .expect("Could not get custom work hours of entity")
                .is_empty());
    });
}
