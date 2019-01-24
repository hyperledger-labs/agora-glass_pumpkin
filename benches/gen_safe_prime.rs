#[macro_use]
extern crate criterion;
extern crate glass_pumpkin;

use glass_pumpkin::safe_prime::new;
use criterion::Criterion;


fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("gen_safe_prime 1", |b| b.iter(|| new(256).unwrap()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
