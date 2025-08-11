use {
    criterion::{criterion_group, criterion_main, Criterion},
    std::hint::black_box, testing::add,
};

fn bench(c: &mut Criterion) {
    c.bench_function("add", |bencher|{
        bencher.iter(|| {
            black_box(add(2, 3));
        });
    });
}

criterion_group!(
    name = benches;
    config = Criterion::default();
    targets = bench
);

criterion_main!(benches);
