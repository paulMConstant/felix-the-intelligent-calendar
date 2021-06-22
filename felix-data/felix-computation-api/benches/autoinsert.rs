use criterion::{criterion_group, criterion_main, Criterion};

use felix_computation_api::{autoinsert, structs::ActivityComputationStaticData};

fn bench_autoinsert_light(c: &mut Criterion) {
    let static_data = vec![
        // 0
        ActivityComputationStaticData {
            possible_insertion_beginnings_minutes_sorted: (0..100).step_by(5).collect(),
            indexes_of_incompatible_activities: vec![1, 2, 3, 4, 5],
            duration_minutes: 25,
        },
        // 1
        ActivityComputationStaticData {
            possible_insertion_beginnings_minutes_sorted: (0..500).step_by(15).collect(),
            indexes_of_incompatible_activities: vec![0, 2, 3, 4],
            duration_minutes: 30,
        },
        // 2
        ActivityComputationStaticData {
            possible_insertion_beginnings_minutes_sorted: (0..1000).step_by(35).collect(),
            indexes_of_incompatible_activities: vec![0, 1, 3, 4],
            duration_minutes: 25,
        },
        // 3
        ActivityComputationStaticData {
            possible_insertion_beginnings_minutes_sorted: (0..100).step_by(10).collect(),
            indexes_of_incompatible_activities: vec![0, 1, 2, 4],
            duration_minutes: 25,
        },
        // 4
        ActivityComputationStaticData {
            possible_insertion_beginnings_minutes_sorted: (0..200).step_by(5).collect(),
            indexes_of_incompatible_activities: vec![0, 1, 2, 3],
            duration_minutes: 20,
        },
        // 5
        ActivityComputationStaticData {
            possible_insertion_beginnings_minutes_sorted: (0..300).step_by(5).collect(),
            indexes_of_incompatible_activities: vec![0],
            duration_minutes: 70,
        },
        // 6
        ActivityComputationStaticData {
            possible_insertion_beginnings_minutes_sorted: (100..500).step_by(5).collect(),
            indexes_of_incompatible_activities: vec![8, 9],
            duration_minutes: 35,
        },
        // 7
        ActivityComputationStaticData {
            possible_insertion_beginnings_minutes_sorted: (50..200).step_by(5).collect(),
            indexes_of_incompatible_activities: vec![8, 9],
            duration_minutes: 25,
        },
        // 8
        ActivityComputationStaticData {
            possible_insertion_beginnings_minutes_sorted: (0..100).step_by(5).collect(),
            indexes_of_incompatible_activities: vec![6, 7, 9],
            duration_minutes: 15,
        },
        // 9
        ActivityComputationStaticData {
            possible_insertion_beginnings_minutes_sorted: (0..100).step_by(5).collect(),
            indexes_of_incompatible_activities: vec![6, 7, 10, 11],
            duration_minutes: 10,
        },
        // 10
        ActivityComputationStaticData {
            possible_insertion_beginnings_minutes_sorted: (50..100).step_by(5).collect(),
            indexes_of_incompatible_activities: vec![9, 11],
            duration_minutes: 20,
        },
        // 11
        ActivityComputationStaticData {
            possible_insertion_beginnings_minutes_sorted: (50..200).step_by(5).collect(),
            indexes_of_incompatible_activities: vec![9, 10],
            duration_minutes: 15,
        },
        // 12
        ActivityComputationStaticData {
            possible_insertion_beginnings_minutes_sorted: (0..100).step_by(5).collect(),
            indexes_of_incompatible_activities: vec![],
            duration_minutes: 100,
        },
    ];
    let insertion_data = vec![];

    c.bench_function("Bench autoinsert_light", |b| {
        b.iter(|| {
            let handle = autoinsert(&static_data, &insertion_data);
            // Wait for computation result
            handle.get_final_result().unwrap();
        });
    });
}

fn bench_autoinsert_heavy(c: &mut Criterion) {
    // TODO make a tighter version
    let static_data = vec![
        // 0
        ActivityComputationStaticData {
            possible_insertion_beginnings_minutes_sorted: (0..1000).step_by(5).collect(),
            indexes_of_incompatible_activities: vec![1, 2, 3, 4, 5],
            duration_minutes: 30,
        },
        // 1
        ActivityComputationStaticData {
            possible_insertion_beginnings_minutes_sorted: (0..5000).step_by(15).collect(),
            indexes_of_incompatible_activities: vec![0, 2, 3, 4, 6],
            duration_minutes: 30,
        },
        // 2
        ActivityComputationStaticData {
            possible_insertion_beginnings_minutes_sorted: (0..1000).step_by(35).collect(),
            indexes_of_incompatible_activities: vec![0, 1, 3, 4, 7],
            duration_minutes: 25,
        },
        // 3
        ActivityComputationStaticData {
            possible_insertion_beginnings_minutes_sorted: (0..100).step_by(10).collect(),
            indexes_of_incompatible_activities: vec![0, 1, 2, 4],
            duration_minutes: 25,
        },
        // 4
        ActivityComputationStaticData {
            possible_insertion_beginnings_minutes_sorted: (0..200).step_by(5).collect(),
            indexes_of_incompatible_activities: vec![0, 1, 2, 3],
            duration_minutes: 20,
        },
        // 5
        ActivityComputationStaticData {
            possible_insertion_beginnings_minutes_sorted: (0..300).step_by(5).collect(),
            indexes_of_incompatible_activities: vec![0],
            duration_minutes: 70,
        },
        // 6
        ActivityComputationStaticData {
            possible_insertion_beginnings_minutes_sorted: (100..500).step_by(5).collect(),
            indexes_of_incompatible_activities: vec![8, 9, 1],
            duration_minutes: 35,
        },
        // 7
        ActivityComputationStaticData {
            possible_insertion_beginnings_minutes_sorted: (50..200).step_by(5).collect(),
            indexes_of_incompatible_activities: vec![8, 9, 2],
            duration_minutes: 25,
        },
        // 8
        ActivityComputationStaticData {
            possible_insertion_beginnings_minutes_sorted: (0..100).step_by(5).collect(),
            indexes_of_incompatible_activities: vec![6, 7, 9],
            duration_minutes: 15,
        },
        // 9
        ActivityComputationStaticData {
            possible_insertion_beginnings_minutes_sorted: (0..100).step_by(5).collect(),
            indexes_of_incompatible_activities: vec![6, 7, 10, 11],
            duration_minutes: 10,
        },
        // 10
        ActivityComputationStaticData {
            possible_insertion_beginnings_minutes_sorted: (50..100).step_by(5).collect(),
            indexes_of_incompatible_activities: vec![9, 11, 14],
            duration_minutes: 20,
        },
        // 11
        ActivityComputationStaticData {
            possible_insertion_beginnings_minutes_sorted: (50..200).step_by(5).collect(),
            indexes_of_incompatible_activities: vec![9, 10, 12, 13],
            duration_minutes: 15,
        },
        // 12
        ActivityComputationStaticData {
            possible_insertion_beginnings_minutes_sorted: (50..200).step_by(5).collect(),
            indexes_of_incompatible_activities: vec![9, 10, 11, 13],
            duration_minutes: 15,
        },
        // 13
        ActivityComputationStaticData {
            possible_insertion_beginnings_minutes_sorted: (50..200).step_by(5).collect(),
            indexes_of_incompatible_activities: vec![9, 10, 12, 13],
            duration_minutes: 15,
        },
        // 14
        ActivityComputationStaticData {
            possible_insertion_beginnings_minutes_sorted: (0..100).step_by(5).collect(),
            indexes_of_incompatible_activities: vec![10, 15],
            duration_minutes: 50,
        },
        // 15
        ActivityComputationStaticData {
            possible_insertion_beginnings_minutes_sorted: (0..100).step_by(5).collect(),
            indexes_of_incompatible_activities: vec![14],
            duration_minutes: 50,
        },
        // 16
        ActivityComputationStaticData {
            possible_insertion_beginnings_minutes_sorted: (0..100).step_by(5).collect(),
            indexes_of_incompatible_activities: vec![],
            duration_minutes: 100,
        },
        // 17
        ActivityComputationStaticData {
            possible_insertion_beginnings_minutes_sorted: (0..100).step_by(5).collect(),
            indexes_of_incompatible_activities: vec![],
            duration_minutes: 100,
        },
    ];
    let insertion_data = vec![];

    c.bench_function("Bench autoinsert_heavy", |b| {
        b.iter(|| {
            let handle = autoinsert(&static_data, &insertion_data);
            // Wait for computation result
            handle.get_final_result().unwrap();
        });
    });
}

criterion_group!(benches, bench_autoinsert_light, bench_autoinsert_heavy);

criterion_main!(benches);
