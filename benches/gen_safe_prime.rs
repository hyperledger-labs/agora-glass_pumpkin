use criterion::{criterion_group, criterion_main, Criterion};
use glass_pumpkin::safe_prime::new;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("gen_safe_prime 1", |b| b.iter(|| new::<4>(256).unwrap()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
