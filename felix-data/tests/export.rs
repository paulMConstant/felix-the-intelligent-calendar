use felix_datatypes::{Time, TimeInterval};
use felix_test_utils::{Activity, Group, DataBuilder, test_ok};

#[test]
fn test_printable_data_conversion() {
    let (entity1, entity2, entity3) = ("1", "2", "3");
    let entities = vec![entity1, entity2, entity3];

    let work_hours = TimeInterval::new(Time::new(8, 0), Time::new(12, 0));

    let group1 = Group {
        name: "Group",
        entities: Vec::new(),
    };

    let activity1 = Activity {
        name: "Activity1",
        duration: Time::new(0, 30),
        entities: vec![entity1, entity3],
        groups: vec![group1.name],
        insertion_time: Some(Time::new(9, 0)),
    };
    let activities = vec![activity1.clone()];

    test_ok!(
        data,
        DataBuilder::new()
            .with_work_interval(work_hours)
            .with_group(group1)
            .with_entities(entities.clone())
            .with_activities(activities),
    {
        let printable_data = data.as_printable();
        
        for entity in entities {
            assert!(printable_data.contains_key(entity),
            "At least one entity was not taken into account");
        }
        
        let activities_of_entity1 = &printable_data[entity1];

        assert_eq!(activities_of_entity1.len(), 1, "Invalid number of activities");
        let real_activity1 = &activities_of_entity1[0];

        assert_eq!(real_activity1.name(), activity1.name, 
                   "Invalid activity name");
        assert_eq!(real_activity1.duration(), activity1.duration, 
                   "Invalid activity duration");
        assert_eq!(real_activity1.entities_sorted(), activity1.entities,
                   "Invalid activity participants");
        assert_eq!(real_activity1.groups_sorted(), activity1.groups,
                   "Invalid activity groups");

        let activities_of_entity2 = &printable_data[entity2];
        assert!(activities_of_entity2.is_empty());

        let activities_of_entity3 = &printable_data[entity3];
        assert_eq!(activities_of_entity3, activities_of_entity1);
    });
}
 #[test]
fn pdfs_are_generated() {
// TODO create data with two entities then for each entity check that one pdf has been generated
}
