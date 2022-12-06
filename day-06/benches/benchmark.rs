use criterion::{criterion_group, criterion_main, Criterion};
use day_06::find_first_occurrence_of_all_different_chars_in;

fn criterion_benchmark(c: &mut Criterion) {
    let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    let sequence_length = 4;

    c.bench_function("find_first_occurrence_of_all_different_chars_in", |b| {
        b.iter(|| find_first_occurrence_of_all_different_chars_in(&input, sequence_length))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
