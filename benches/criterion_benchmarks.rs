use std::fs::read;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use levenshtein_diff as levenshtein;

pub fn distance(c: &mut Criterion) {
    let mut group = c.benchmark_group("distance");
    let bytes = read("benches/data/atoz-2k.bin").unwrap();
    assert_eq!(bytes.len(), 2_000);

    // test naive separately because it is too slow for larger cases
    let input = (&bytes[0..10], &bytes[10..20]);
    group.bench_with_input(BenchmarkId::new("naive", 10), &input, |b, i| {
        b.iter(|| levenshtein::levenshtein_naive(i.0, i.1))
    });

    for len in [10, 100, 1000] {
        let input = (&bytes[0..len], &bytes[len..2 * len]);
        group.bench_with_input(BenchmarkId::new("tabluation", len), &input, |b, i| {
            b.iter(|| levenshtein::levenshtein_tabulation(i.0, i.1))
        });
        group.bench_with_input(BenchmarkId::new("memoization", len), &input, |b, i| {
            b.iter(|| levenshtein::levenshtein_memoization(i.0, i.1))
        });
    }
    group.finish();
}

pub fn generate_edits(c: &mut Criterion) {
    let mut group = c.benchmark_group("generate_edits");
    let bytes = read("benches/data/atoz-2k.bin").unwrap();
    assert_eq!(bytes.len(), 2_000);

    for len in [100, 1000] {
        let s1 = &bytes[0..len];
        let s2 = &bytes[len..2 * len];
        let (_, mtx) = levenshtein::distance(s1, s2);
        let input = (s1, s2, &mtx);
        group.bench_with_input(len.to_string(), &input, |b, i| {
            b.iter(|| levenshtein::generate_edits(i.0, i.1, i.2))
        });
    }
    group.finish();
}

pub fn apply_edits(c: &mut Criterion) {
    let mut group = c.benchmark_group("apply_edits");
    let bytes = read("benches/data/atoz-2k.bin").unwrap();
    assert_eq!(bytes.len(), 2_000);

    for len in [100, 1000] {
        let s1 = &bytes[0..len];
        let s2 = &bytes[len..2 * len];
        let (_, mtx) = levenshtein::distance(s1, s2);
        let edits = levenshtein::generate_edits(s1, s2, &mtx).unwrap();
        let input = (s1, edits.as_slice());
        group.bench_with_input(len.to_string(), &input, |b, i| {
            b.iter(|| levenshtein::apply_edits(i.0, i.1))
        });
    }
    group.finish();
}

criterion_group!(benches, distance, generate_edits, apply_edits);
criterion_main!(benches);
