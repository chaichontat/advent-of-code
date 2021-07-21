#[macro_use]
extern crate criterion;
extern crate advent_of_code_2018;
use advent_of_code_2018::utils::*;
use advent_of_code_2018::*;
use criterion::{black_box, Criterion};
use pprof::criterion::{Output, PProfProfiler};

fn bench_fi(c: &mut Criterion) {
    let day01 = &read("day01.txt");
    c.bench_function("day01combi", |b| b.iter(|| day01::bench(black_box(day01))));

    let day02 = &read_ascii_sep("day02.txt");
    c.bench_function("day02a", |b| b.iter(|| day02::part1(black_box(day02))));
    c.bench_function("day02b", |b| b.iter(|| day02::part2(black_box(day02))));

    let day08 = day08::parse(read_nosep("day08.txt"));
    c.bench_function("day08combi", |b| b.iter(|| day08::combi(black_box(&day08))));

    let day09 = day09::parse(&read_nosep("day09.txt"));
    c.bench_function("day09combi", |b| b.iter(|| day09::combi(black_box(&day09))));

    let day10 = &read_ascii_sep("day10.txt");
    c.bench_function("day10combi", |b| b.iter(|| day10::combi(black_box(day10))));

    let day12 = &read_ascii_sep("day12.txt");
    c.bench_function("day12combi", |b| b.iter(|| day12::combi(black_box(day12))));

    let day14 = &read_ascii_sep("day14.txt");
    c.bench_function("day14combi", |b| b.iter(|| day14::combi(black_box(day14))));

    // let day23 = day23::parse(&read_ascii_sep("day23.txt"));
    // c.bench_function("day23combi", |b| b.iter(|| day23::combi(black_box(&day23))));
}

criterion_group! {
    name = benches;
    config = Criterion::default().with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
    targets = bench_fi
}

criterion_main!(benches);
