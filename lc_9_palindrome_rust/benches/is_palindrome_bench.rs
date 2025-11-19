use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use std::hint::black_box;

use lc_9_palindrome_rust::is_palindrome;
use lc_9_palindrome_rust::v2::is_palindrome_v2;

// pub fn criterion_benchmark(c: &mut Criterion) {
//     c.bench_function("is_palindrome_v2", |b| {
//         b.iter(|| is_palindrome(black_box(1234567890987654321)))
//     });
// }

fn bench_compare_functions(c: &mut Criterion) {
    let mut group = c.benchmark_group("is_palindrome_benchgroup");
    let inp = 123456789987654321;
    group.bench_function(BenchmarkId::new("is_palindrome_v1", inp), |b| {
        b.iter(|| is_palindrome(black_box(inp)))
    });
    group.bench_function(BenchmarkId::new("is_palindrome_v2", inp), |b| {
        b.iter(|| is_palindrome_v2(black_box(inp)))
    });
    group.finish();
}
criterion_group!(benches, bench_compare_functions);
criterion_main!(benches);
