use criterion::{criterion_group, criterion_main, Criterion};

use felix_computation_api::{
    compute_insertion_costs::{compute_insertion_costs, get_activity_beginnings_with_conflicts},
    structs::{ActivityComputationStaticData, ActivityInsertionBeginningMinutes},
};

fn create_data() -> (
    Vec<ActivityComputationStaticData>,
    Vec<ActivityInsertionBeginningMinutes>,
) {
    let static_data = vec![
        ActivityComputationStaticData {
            possible_insertion_beginnings_minutes_sorted: (0..1000).step_by(5).collect(),
            indexes_of_incompatible_activities: vec![1, 2, 3, 4],
            duration_minutes: 10,
        },
        ActivityComputationStaticData {
            possible_insertion_beginnings_minutes_sorted: (100..200).step_by(5).collect(),
            indexes_of_incompatible_activities: vec![0, 3],
            duration_minutes: 100,
        },
        ActivityComputationStaticData {
            possible_insertion_beginnings_minutes_sorted: (1000..2000).step_by(10).collect(),
            indexes_of_incompatible_activities: vec![0, 4],
            duration_minutes: 150,
        },
        ActivityComputationStaticData {
            possible_insertion_beginnings_minutes_sorted: (150..300).step_by(5).collect(),
            indexes_of_incompatible_activities: vec![0, 1],
            duration_minutes: 30,
        },
        ActivityComputationStaticData {
            possible_insertion_beginnings_minutes_sorted: (500..700).step_by(10).collect(),
            indexes_of_incompatible_activities: vec![0, 2],
            duration_minutes: 55,
        },
    ];

    let insertion_data = vec![None, None, Some(1500), Some(200), None];

    (static_data, insertion_data)
}

fn bench_filter_conflicts(c: &mut Criterion) {
    let (static_data, insertion_data) = create_data();
    c.bench_function("Bench filter_conflicts", |b| {
        b.iter(|| get_activity_beginnings_with_conflicts(&static_data, &insertion_data))
    });
}

fn bench_compute_costs(c: &mut Criterion) {
    let (static_data, insertion_data) = create_data();

    c.bench_function("Bench compute insertion costs", |b| {
        b.iter(|| compute_insertion_costs(&static_data, &insertion_data))
    });
}

criterion_group!(benches, bench_filter_conflicts, bench_compute_costs);

criterion_main!(benches);
