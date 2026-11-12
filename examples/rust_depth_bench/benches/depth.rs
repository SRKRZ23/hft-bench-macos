use criterion::{black_box, criterion_group, criterion_main, Criterion};
use depth_bench::PriceLevels;

fn bench_depth(c: &mut Criterion) {
    let levels = PriceLevels::new(100);

    c.bench_function("depth_before_collect_5", |b| {
        b.iter(|| black_box(levels.depth_before(5)))
    });

    c.bench_function("depth_after_with_capacity_5", |b| {
        b.iter(|| black_box(levels.depth_after(5)))
    });
}

criterion_group!(benches, bench_depth);
criterion_main!(benches);
