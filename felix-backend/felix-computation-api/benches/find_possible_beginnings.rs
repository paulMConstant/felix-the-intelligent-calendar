use criterion::{criterion_group, criterion_main, Criterion};

use felix_computation_api::{
    find_possible_beginnings::{can_fit_in_schedule, compute_all_sums, find_possible_beginnings},
    structs::WorkHourInMinutes,
};

use std::collections::HashSet;

fn bench_find_possible_beginnings(c: &mut Criterion) {
    c.bench_function("Find possible beginnings 10 activities 4 work hours", |b| {
        b.iter(|| {
            find_possible_beginnings(
                &[
                    WorkHourInMinutes::new(10, 30),
                    WorkHourInMinutes::new(50, 120),
                    WorkHourInMinutes::new(800, 935),
                    WorkHourInMinutes::new(250, 450),
                ],
                &[15, 15, 20, 20, 30, 30, 40, 45, 60, 80],
            )
        })
    });

    c.bench_function("Find possible beginnings 12 activities 2 work hours", |b| {
        b.iter(|| {
            find_possible_beginnings(
                &[
                    WorkHourInMinutes::new(250, 550),
                    WorkHourInMinutes::new(800, 1235),
                ],
                &[25, 25, 30, 30, 40, 40, 45, 45, 60, 80, 90, 120],
            )
        })
    });
}

fn bench_can_fit_in_schedule(c: &mut Criterion) {
    bench_case_can_fit_in_schedule(
        c,
        "Can fit in schedule false",
        &[240, 180],
        &[20, 20, 25, 30, 50, 60, 225],
    );
    bench_case_can_fit_in_schedule(
        c,
        "Can fit in schedule true",
        &[240, 180],
        &[20, 30, 30, 40, 50, 60, 70, 120],
    );
}

/// Creates a bench case. Is not called directly by criterion.
fn bench_case_can_fit_in_schedule(
    c: &mut Criterion,
    bench_name: &str,
    work_hour_durations: &[u16],
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
                work_hour_durations,
                time_which_can_be_wasted,
                HashSet::new(),
            )
        })
    });
}

fn bench_compute_all_sums(c: &mut Criterion) {
    let activity_durations = &[20, 30, 30, 40, 50, 60, 70, 80, 90, 120];
    c.bench_function("Compute all sums 10 activities", |b| {
        b.iter(|| compute_all_sums(activity_durations))
    });
}

criterion_group!(
    benches,
    bench_can_fit_in_schedule,
    bench_compute_all_sums,
    bench_find_possible_beginnings
);

criterion_main!(benches);
