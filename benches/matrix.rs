use criterion::{criterion_group, criterion_main, Criterion};
use math_rs::{matrix::generic::Matrix, matrix_rationals, structures::rationals::Rational};
use std::str::FromStr;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("sum matrix 3x3", |b| {
        b.iter(|| {
            let matrix_a = matrix_rationals!("{{1, 1, 1},{1, 1, 1},{1, 1, 1}}").unwrap();
            let matrix_b = matrix_rationals!("{{1, 1, 1},{1, 1, 1},{1, 1, 1}}").unwrap();
            matrix_a + matrix_b
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
