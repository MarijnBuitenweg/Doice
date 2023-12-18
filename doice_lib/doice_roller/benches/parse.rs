use criterion::{black_box, criterion_group, criterion_main, Criterion};
use doice_roller::Roll;

fn parse_expr(s: &str) -> Roll {
    s.try_into().unwrap()
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("basic lincomb", |b| {
        b.iter(|| parse_expr(black_box("3+4*5")));
    });
    c.bench_function("basic dice", |b| {
        b.iter(|| parse_expr(black_box("3d4 + 69d420*6d8 + d||")));
    });
    c.bench_function("functions", |b| {
        b.iter(|| parse_expr(black_box("emp(8d6, 5) + sum(3d4, 2d3) + dick()")));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
