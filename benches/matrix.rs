use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use math_rs::matrix::generic::Matrix;

// fn bench_sum(c: &mut Criterion) {
//     let mut group = c.benchmark_group("Sum 3x3 matrix");
//     for i in [20u64, 21u64].iter() {
//         let mat_a = vec![vec![1; *i as usize]; *i as usize];
//         let mat_b = vec![vec![-1; *i as usize]; *i as usize];
//         group.bench_with_input(BenchmarkId::new("With structure", i), i, |b, _| {
//             b.iter(|| {
//                 let matrix_a = Matrix {
//                     data: mat_a.clone(),
//                 };
//                 let matrix_b = Matrix {
//                     data: mat_b.clone(),
//                 };
//                 let _ = matrix_a + matrix_b;
//             })
//         });
//         group.bench_with_input(BenchmarkId::new("Native", i), i, |b, _| {
//             b.iter(|| {
//                 for i in 0..3 {
//                     for j in 0..3 {
//                         let _ = mat_a[i][j] + mat_b[i][j];
//                     }
//                 }
//             })
//         });
//     }
//     group.finish();
// }

fn bench_sum_with_incrementing_dimensions(c: &mut Criterion) {
    let mut group = c.benchmark_group("Sum matrix of growing size");
    for i in 1..2 {
        for j in 1..10 {
            let mat_a = vec![vec![1; j * 10]; j * 10];
            let mat_b = vec![vec![-1; j * 10]; j * 10];
            group.bench_with_input(
                BenchmarkId::new(format!("Benchmark structured sum {j}0x{j}0"), i),
                &i,
                |b, _| {
                    b.iter(|| {
                        let matrix_a = Matrix {
                            data: mat_a.clone(),
                        };
                        let matrix_b = Matrix {
                            data: mat_b.clone(),
                        };
                        let _ = matrix_a + matrix_b;
                    })
                },
            );
        }
    }
    group.finish()
}

criterion_group!(benches, bench_sum_with_incrementing_dimensions);
criterion_main!(benches);
