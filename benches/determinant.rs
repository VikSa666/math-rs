use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use math_rs::matrix::square::{determinant::DeterminantMethod, SquareMatrix};

fn bench_determinants(c: &mut Criterion) {
    let mut group = c.benchmark_group("Calculate determinant");
    let huge_matrix = SquareMatrix::from_fn(100, |i, j| {
        if (i as isize - j as isize).abs() < 3 {
            1
        } else {
            0
        }
    });
    group.bench_function(
        BenchmarkId::new(format!("Determinant using Bareiss Algorithm"), 0),
        |b| b.iter(|| huge_matrix.determinant(DeterminantMethod::BareissAlgorithm, 1e-10)),
    );

    // group.bench_with_input(
    //     BenchmarkId::new(format!("Determinant using Laplace Expansion"), 0),
    //     &0,
    //     |b, _| b.iter(|| huge_matrix.determinant(DeterminantMethod::LaplaceExpansion, 1e-10)),
    // );

    group.bench_function(
        BenchmarkId::new("Determinant using Gaussian Elimination", 0),
        |b| b.iter(|| huge_matrix.determinant(DeterminantMethod::GaussianElimination, 1e-10)),
    );

    group.finish()
}

criterion_group!(benches, bench_determinants);
criterion_main!(benches);
