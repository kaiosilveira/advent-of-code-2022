use criterion::{criterion_group, criterion_main, Criterion};
use day_02::strategies::{
    rock_paper_scissors_guessed_strategy::*, rock_paper_scissors_real_strategy::*,
};

fn criterion_benchmark(c: &mut Criterion) {
    let mut input: Vec<&str> = vec![];

    for _n in 0..1000 {
        input.push("A X");
    }

    c.bench_function("guessed strategy", |b| {
        b.iter(|| run_guessed_strategy(&input))
    });

    c.bench_function("real strategy", |b| {
        b.iter(|| run_real_strategy(&input))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
