use criterion::{criterion_group, criterion_main, Criterion};
use std::collections::BTreeSet;

use felix_computation_api::{compute_insertion_costs, structs::ActivityComputationStaticData};

fn bench_filter_conflicts(c: &mut Criterion) {
    let static_data = vec![
        ActivityComputationStaticData {
            possible_insertion_beginnings_minutes_sorted: btreeset_from_slice(&[
                0, 5, 10, 20, 35, 45, 50,
            ]),
            indexes_of_incompatible_activities: vec![1, 2, 3, 4],
            duration_minutes: 10,
        },
        ActivityComputationStaticData {
            possible_insertion_beginnings_minutes_sorted: btreeset_from_slice(&[]), // We don't care
            indexes_of_incompatible_activities: vec![0],
            duration_minutes: 10,
        },
        ActivityComputationStaticData {
            possible_insertion_beginnings_minutes_sorted: btreeset_from_slice(&[]), // We don't care
            indexes_of_incompatible_activities: vec![0],
            duration_minutes: 15,
        },
        ActivityComputationStaticData {
            possible_insertion_beginnings_minutes_sorted: btreeset_from_slice(&[]), // We don't care
            indexes_of_incompatible_activities: vec![0],
            duration_minutes: 10,
        },
        ActivityComputationStaticData {
            possible_insertion_beginnings_minutes_sorted: btreeset_from_slice(&[]), // We don't care
            indexes_of_incompatible_activities: vec![0],
            duration_minutes: 5,
        },
    ];

    const INSERTION_BEGINNING_MINUTES1: u16 = 5;
    const INSERTION_BEGINNING_MINUTES2: u16 = 30;
    const INSERTION_BEGINNING_MINUTES3: u16 = 5;
    const INSERTION_BEGINNING_MINUTES4: u16 = 50;
    let insertion_data = vec![
        None,
        Some(INSERTION_BEGINNING_MINUTES1),
        Some(INSERTION_BEGINNING_MINUTES2),
        Some(INSERTION_BEGINNING_MINUTES3),
        Some(INSERTION_BEGINNING_MINUTES4),
    ];

    c.bench_function("Bench compute insertion costs", |b| {
        b.iter(|| compute_insertion_costs(&static_data, &insertion_data))
    });
}

fn btreeset_from_slice(slice: &[u16]) -> BTreeSet<u16> {
    slice.iter().map(|&i| i as u16).collect::<BTreeSet<_>>()
}

criterion_group!(benches, bench_filter_conflicts);

criterion_main!(benches);
