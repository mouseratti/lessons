use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

use lc_9_palindrome_rust::is_palindrome;
use lc_9_palindrome_rust::v2::is_palindrome_v2;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("is_palindrome_v2", |b| {
        b.iter(|| is_palindrome(black_box(1234567890987654321)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
