#[macro_use(criterion_main, criterion_group)]
extern crate criterion;
extern crate branchless_algorithms;

use branchless_algorithms::sorting;
use criterion::Criterion;

fn check_bubble_sort(c: &mut Criterion) {
    c.bench_function("bubble_sort", |b| b.iter(|| sorting::bubble_sort(&mut [100, -100, 1, 79, -4])));
}

fn check_selection_sort(c: &mut Criterion) {
    c.bench_function("selection_sort", |b| b.iter(|| sorting::selection_sort(&mut [100, -100, 1, 79, -4])));
}

criterion_group!(
    benches,
    check_bubble_sort,
    check_selection_sort,
);

criterion_main!(
    benches,
);
