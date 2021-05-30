use super::*;
use felix_collections::Activities;
use felix_datatypes::Time;

fn make_activities() -> Vec<Activity> {
    let entity1 = "Paul".to_string();
    let entity2 = "Déborah".to_string();

    let mut activities = Activities::new();
    activities.add("I am an activity with a very long name".to_string());

    let id = activities.get_not_sorted()[0].id();
    let duration = Time::new(0, 30);
    let beginning = Time::new(8, 0);
    activities.add_entity(id, entity1.clone()).unwrap();
    activities.add_entity(id, entity2).unwrap();
    activities.set_duration(id, duration);
    activities.insert_activity(id, Some(beginning));

    activities.add("Short-named activity".to_string());
    let id = activities.get_not_sorted()[1].id();
    let duration = Time::new(0, 30);
    let beginning = Time::new(9, 0);
    activities.add_entity(id, entity1).unwrap();
    activities.set_duration(id, duration);
    activities.insert_activity(id, Some(beginning));

    activities.get_not_sorted()
}

#[test]
fn test_pdf_size_always_coherent() {
    // TODO generate fuzzy data
    // then make sure that
    // - every pdf does not exceed one page
    // - either size is maximal or if size increases then pdf exceeds one page
    let entity = "Paul".to_string();

    let mut activities = make_activities();
    activities.append(&mut make_activities());
    activities.append(&mut make_activities());
    activities.append(&mut make_activities());
    activities.append(&mut make_activities());
    activities.append(&mut make_activities());
    activities.append(&mut make_activities());

    let res = Pdf::new(entity, activities);
    // Check that line is generated

    // TODO remove this later
    res.render("");
    // Check line length
    // Check height
    // Check title
}
#[test]
fn generate_pdf_of_every_size_for_manual_checks() {
    // TODO generate various pdfs and manually check for out of line proportions
}

#[test]
fn test_lines_extracted_from_activities() {
    let activities = make_activities();

    let lines = extract_lines_from_activities(activities.clone());

    assert_eq!(lines.len(), activities.len());
    assert_eq!(lines.len(), 2);

    assert_eq!(
        lines[0].print(),
        "08:00 - 08:30 : I am an activity with a very long name (Déborah, Paul)"
    );
    assert_eq!(
        lines[1].print(),
        "09:00 - 09:30 : Short-named activity (Paul)"
    );
}

#[test]
fn test_lines_split_if_too_long() {
    let activities = make_activities();
    let entity = "Paul".to_string();
    let mut res = Pdf::new(entity, activities);
    res.compute_line_breaks();
    assert!(!res.line_breaks.is_empty());
}
