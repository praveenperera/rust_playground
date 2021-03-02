use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use strs::*;

fn joining_interspersed_strings(c: &mut Criterion) {
    let mut group = c.benchmark_group("Joining Interspersed Strings");
    group.bench_function("std_join", |b| b.iter(|| std_join()));
    group.bench_function("join_iter_with_intersperse", |b| {
        b.iter(|| join_iter_with_intersperse())
    });
    group.bench_function("join_iter_strs", |b| b.iter(|| join_iter_strs()));
    group.bench_function("join_iter_chars", |b| b.iter(|| join_iter_chars()));

    group.bench_function("join_chars_with_string_capacity", |b| {
        b.iter(|| join_chars_with_string_capacity())
    });
    group.finish();
}

fn joining_simple_strings(c: &mut Criterion) {
    let mut group = c.benchmark_group("Simple Strings");
    group.bench_function("join_chars_with_collect", |b| {
        b.iter(|| join_chars_with_collect())
    });
    group.bench_function("join_chars_with_string_capacity", |b| {
        b.iter(|| join_chars_with_string_capacity())
    });

    group.finish();
}

fn prefixing(c: &mut Criterion) {
    let mut group = c.benchmark_group("Prefixing");

    let string = "abcdef".to_string();
    group.bench_function("prefix_collect", move |b| {
        b.iter_batched(
            || string.clone(),
            |s| prefix_collect(criterion::black_box(s)),
            BatchSize::SmallInput,
        )
    });

    let string = "abcdef".to_string();
    group.bench_function("prefix_string_capacity", move |b| {
        b.iter_batched(
            || string.clone(),
            |s| prefix_string_capacity(criterion::black_box(s)),
            BatchSize::SmallInput,
        )
    });

    group.finish();
}

criterion_group!(
    benches,
    joining_interspersed_strings,
    joining_simple_strings,
    prefixing
);
criterion_main!(benches);
