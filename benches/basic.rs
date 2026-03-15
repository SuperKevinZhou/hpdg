use criterion::{criterion_group, criterion_main, Criterion};
use hpdg::math::{fibonacci, prime_sieve};

fn bench_fibonacci(c: &mut Criterion) {
    c.bench_function("fibonacci_40", |b| b.iter(|| fibonacci(40)));
}

fn bench_prime_sieve(c: &mut Criterion) {
    c.bench_function("prime_sieve_10000", |b| b.iter(|| prime_sieve(10_000)));
}

criterion_group!(benches, bench_fibonacci, bench_prime_sieve);
criterion_main!(benches);
