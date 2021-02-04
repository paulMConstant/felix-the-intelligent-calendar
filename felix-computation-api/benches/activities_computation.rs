use criterion::{criterion_group, criterion_main, Criterion};

use felix_computation_api::find_possible_beginnings::{can_fit_in_schedule, compute_all_sums};

use std::collections::HashSet;

fn bench_case_can_fit_in_schedule(
    c: &mut Criterion,
    bench_name: &str,
    work_hour_durations: Vec<u16>,
    activity_durations: &[u16],
) {
    let all_activity_sums = compute_all_sums(activity_durations);
    let time_which_can_be_wasted =
        work_hour_durations.iter().sum::<u16>() - activity_durations.iter().sum::<u16>();

    c.bench_function(bench_name, |b| {
        b.iter(|| {
            can_fit_in_schedule(
                activity_durations.len(),
                &all_activity_sums,
                work_hour_durations.clone(),
                time_which_can_be_wasted,
                HashSet::new(),
            )
        })
    });
}

fn bench_can_fit_in_schedule(c: &mut Criterion) {
    bench_case_can_fit_in_schedule(c, "Little false", vec![30, 39, 50], &[11, 20, 39, 40]);
    bench_case_can_fit_in_schedule(c, "Little true", vec![30, 39, 50], &[10, 20, 39, 40]);
    // 4 hours, 3 hours
    bench_case_can_fit_in_schedule(
        c,
        "Big false",
        vec![240, 180],
        &[20, 20, 25, 30, 50, 60, 225],
    );
    bench_case_can_fit_in_schedule(
        c,
        "Big true",
        vec![240, 180],
        &[20, 30, 30, 40, 50, 60, 70, 120],
    );
}

fn bench_compute_all_sums(c: &mut Criterion) {
    let activity_durations = &[20, 30, 30, 40, 50, 60, 70, 80, 90, 120];
    c.bench_function("Compute all sums 10 activities", |b| {
        b.iter(|| compute_all_sums(activity_durations))
    });
}

criterion_group!(benches, bench_can_fit_in_schedule, bench_compute_all_sums);
criterion_main!(benches);
