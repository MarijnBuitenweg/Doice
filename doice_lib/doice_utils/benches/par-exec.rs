use criterion::{black_box, criterion_group, criterion_main, Criterion};
use doice_utils::ParExecutor;

fn startup_bench() -> ParExecutor<f64> {
    black_box(ParExecutor::<f64>::new())
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("par_exec startup", |b| {
        b.iter(|| black_box(startup_bench()))
    });
    let mut exec = ParExecutor::<u64>::new();
    c.bench_function("par_exec delay", |b| {
        b.iter(|| {
            exec.process_into(black_box(42u32));
            black_box(exec.get_data().unwrap())
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
