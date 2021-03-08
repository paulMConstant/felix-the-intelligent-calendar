use super::*;

#[test]
fn test_generate_next_id() {
    let used_ids: Vec<&ActivityId> = vec![&1, &3, &2, &4, &6, &0];
    let expected_next_id = 5;
    assert_eq!(generate_next_id(used_ids), expected_next_id);

    let used_ids: Vec<&ActivityId> = vec![&1, &3, &2, &4, &6];
    let expected_next_id = 0;
    assert_eq!(generate_next_id(used_ids), expected_next_id);

    let used_ids: Vec<&ActivityId> = vec![&1, &3, &0, &4, &6];
    let expected_next_id = 2;
    assert_eq!(generate_next_id(used_ids), expected_next_id);
}
// compute_incompatible_ids is tested in super::activities.rs
