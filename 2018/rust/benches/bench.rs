#[macro_use]
extern crate criterion;
extern crate advent_of_code_2018;
use advent_of_code_2018::utils::*;
use advent_of_code_2018::*;
use criterion::{black_box, Criterion};
use pprof::criterion::{Output, PProfProfiler};

fn bench_fi(c: &mut Criterion) {
    let day01 = &read("day01.txt");
    c.bench_function("day01a", |b| b.iter(|| day01::bench(black_box(day01))));
    // c.bench_function("day01b", |b| b.iter(|| day01::part2(black_box(day01))));
}

criterion_group! {
    name = benches;
    config = Criterion::default().with_profiler(PProfProfiler::new(100, Output::Protobuf));
    targets = bench_fi
}

criterion_main!(benches);
