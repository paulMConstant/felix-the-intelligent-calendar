use super::*;
use felix_collections::Activities;

#[test]
fn test_pdf_struct_generated_correctly() {
    // TODO
    let entity = "Paul".to_string();
    let activities = Activities::new();

    let res = Pdf::new(entity, &activities.get_not_sorted());
    // Check line length
    // Check height
    // Check title
}
