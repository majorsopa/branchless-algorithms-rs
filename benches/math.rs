#[macro_use(criterion_main, criterion_group)]
extern crate criterion;
extern crate branchless_algorithms;

use branchless_algorithms::math;
use criterion::Criterion;


fn check_absolute_value(c: &mut Criterion) {
    c.bench_function("absolute_value", |b| b.iter(|| math::absolute_value(-100)));
}

fn check_greater_than(c: &mut Criterion) {
    c.bench_function("greater_than", |b| b.iter(|| math::greater_than(100, -100)));
}

fn check_less_than(c: &mut Criterion) {
    c.bench_function("less_than", |b| b.iter(|| math::less_than(100, -100)));
}

fn check_greater_than_as_one(c: &mut Criterion) {
    c.bench_function("greater_than_as_one", |b| b.iter(|| math::greater_than_as_one(100, -100)));
}

fn check_less_than_as_one(c: &mut Criterion) {
    c.bench_function("less_than_as_one", |b| b.iter(|| math::less_than_as_one(100, -100)));
}

fn check_equal_to(c: &mut Criterion) {
    c.bench_function("equal_to", |b| b.iter(|| math::equal_to(100, -100)));
}

fn check_maximum(c: &mut Criterion) {
    c.bench_function("maximum", |b| b.iter(|| math::maximum(100, -100)));
}

fn check_maximum_of_list(c: &mut Criterion) {
    c.bench_function("maximum_of_list", |b| b.iter(|| math::maximum_of_list(&vec![100, -100])));
}

fn check_minimum(c: &mut Criterion) {
    c.bench_function("minimum", |b| b.iter(|| math::minimum(100, -100)));
}

fn check_minimum_of_list(c: &mut Criterion) {
    c.bench_function("minimum_of_list", |b| b.iter(|| math::minimum_of_list(&vec![100, -100])));
}

fn check_is_perfect_number(c: &mut Criterion) {
    c.bench_function("is_perfect_number", |b| b.iter(|| math::is_perfect_number(100)));
}

fn check_num_factors(c: &mut Criterion) {
    c.bench_function("num_factors", |b| b.iter(|| math::num_factors(100)));
}

fn check_check_prime(c: &mut Criterion) {
    c.bench_function("check_prime", |b| b.iter(|| math::check_prime(100)));
}

fn check_check_non_zero_unsigned(c: &mut Criterion) {
    c.bench_function("check_non_zero_unsigned", |b| b.iter(|| math::check_non_zero_unsigned(100)));
}

fn check_check_zero_unsigned(c: &mut Criterion) {
    c.bench_function("check_zero_unsigned", |b| b.iter(|| math::check_zero_unsigned(100)));
}


criterion_group!(
    benches_compare,
    check_greater_than,
    check_less_than,
    check_greater_than_as_one,
    check_less_than_as_one,
    check_equal_to,
    check_check_non_zero_unsigned,
    check_check_zero_unsigned,
);

criterion_group!(
    benches_check,
    check_absolute_value,
    check_maximum,
    check_maximum_of_list,
    check_minimum,
    check_minimum_of_list,
    check_is_perfect_number,
    check_num_factors,
    check_check_prime,
);


criterion_main!(
    benches_compare,
    benches_check,
);