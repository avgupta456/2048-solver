use criterion::{criterion_group, criterion_main, Criterion};

use rust_solver::{load_precomputed, run_game};

fn criterion_benchmark(c: &mut Criterion) {
    let precomputed = load_precomputed();
    c.bench_function("run_game", |b| b.iter(|| run_game(&precomputed)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
