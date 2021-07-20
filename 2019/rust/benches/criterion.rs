#[macro_use]
extern crate criterion;
extern crate advent_of_code;
use advent_of_code::utils::*;
use criterion::{black_box, Criterion};
use pprof::criterion::{Output, PProfProfiler};

fn bench_2019(c: &mut Criterion) {
    use advent_of_code::y2019::*;

    let day01 = &read("day01.txt");
    c.bench_function("day01a", |b| b.iter(|| day01::part1(black_box(day01))));
    c.bench_function("day01b", |b| b.iter(|| day01::part2(black_box(day01))));

    let day02 = &read("day02.txt");
    c.bench_function("day02combi", |b| b.iter(|| day02::run(black_box(day02))));

    let day03 = &read("day03.txt");
    c.bench_function("day03a", |b| b.iter(|| day03::part1(black_box(day03))));
    c.bench_function("day03b", |b| b.iter(|| day03::part2(black_box(day03))));

    let day04 = &read("day04.txt");
    c.bench_function("day04a", |b| b.iter(|| day04::part1(black_box(day04))));
    c.bench_function("day04b", |b| b.iter(|| day04::part2(black_box(day04))));

    let day05 = &read("day05.txt");
    c.bench_function("day05a", |b| b.iter(|| day05::part1(black_box(day05))));
    c.bench_function("day05b", |b| b.iter(|| day05::part2(black_box(day05))));

    let day06 = &read("day06.txt");
    c.bench_function("day06a", |b| b.iter(|| day06::part1(black_box(day06))));
    c.bench_function("day06b", |b| b.iter(|| day06::part2(black_box(day06))));

    let day07 = &read("day07.txt");
    c.bench_function("day07a", |b| b.iter(|| day07::part1(black_box(day07))));
    c.bench_function("day07b", |b| b.iter(|| day07::part2(black_box(day07))));

    let day08 = &read("day08.txt");
    c.bench_function("day08a", |b| b.iter(|| day08::part1(black_box(day08))));
    c.bench_function("day08b", |b| b.iter(|| day08::part2(black_box(day08))));

    let day09 = &read("day09.txt");
    c.bench_function("day09a", |b| b.iter(|| day09::part1(black_box(day09))));
    c.bench_function("day09b", |b| b.iter(|| day09::part2(black_box(day09))));

    let day10 = &read("day10.txt");
    c.bench_function("day10a", |b| b.iter(|| day10::bench(black_box(day10))));

    let day11 = &read("day11.txt");
    c.bench_function("day11a", |b| b.iter(|| day11::part1(black_box(day11))));
    c.bench_function("day11b", |b| b.iter(|| day11::part2(black_box(day11))));

    let day12 = &read("day12.txt");
    c.bench_function("day12a", |b| b.iter(|| day12::part1(black_box(day12))));
    c.bench_function("day12b", |b| b.iter(|| day12::part2(black_box(day12))));

    let day13 = &read("day13.txt");
    c.bench_function("day13a", |b| b.iter(|| day13::part1(black_box(day13))));
    c.bench_function("day13b", |b| b.iter(|| day13::part2(black_box(day13))));

    let day14 = &read("day14.txt");
    c.bench_function("day14a", |b| b.iter(|| day14::part1(black_box(day14))));
    c.bench_function("day14b", |b| b.iter(|| day14::part2(black_box(day14))));

    let day15 = &read("day15.txt");
    c.bench_function("day15a", |b| b.iter(|| day15::part1(black_box(day15))));
    c.bench_function("day15b", |b| b.iter(|| day15::part2(black_box(day15))));

    let day16 = &read_ascii("day16.txt");
    c.bench_function("day16a", |b| b.iter(|| day16::part1(black_box(day16))));
    c.bench_function("day16b", |b| b.iter(|| day16::part2(black_box(day16))));

    let day17 = &read("day17.txt");
    c.bench_function("day17a", |b| b.iter(|| day17::part1(black_box(day17))));
    c.bench_function("day17b", |b| b.iter(|| day17::part2(black_box(day17))));

    let day18 = &read("day18.txt");
    let mut day18b = read("day18.txt");
    let xmid = (day18b[0].len() - 1) / 2;
    let ymid = (day18b.len() - 1) / 2;
    day18b[ymid - 1].replace_range(xmid - 1..=xmid + 1, "@#@");
    day18b[ymid].replace_range(xmid - 1..=xmid + 1, "###");
    day18b[ymid + 1].replace_range(xmid - 1..=xmid + 1, "@#@");

    c.bench_function("day18a", |b| b.iter(|| day18::run(black_box(day18))));
    c.bench_function("day18b", |b| b.iter(|| day18::run(black_box(&day18b))));

    let day19 = &read("day19.txt");
    c.bench_function("day19a", |b| b.iter(|| day19::part1(black_box(day19))));
    c.bench_function("day19b", |b| b.iter(|| day19::part2(black_box(day19))));

    let day20 = &read_ascii("day20.txt");
    c.bench_function("day20a", |b| b.iter(|| day20::part1(black_box(day20))));
    c.bench_function("day20b", |b| b.iter(|| day20::part2(black_box(day20))));

    let day21 = &read("day21.txt");
    c.bench_function("day21a", |b| b.iter(|| day21::part1(black_box(day21))));
    c.bench_function("day21b", |b| b.iter(|| day21::part2(black_box(day21))));

    let day22 = &read("day22.txt");
    c.bench_function("day22a", |b| b.iter(|| day22::part1(black_box(day22))));
    c.bench_function("day22b", |b| b.iter(|| day22::part2(black_box(day22))));

    let day23 = &read("day23.txt");
    c.bench_function("day23a", |b| b.iter(|| day23::part1(black_box(day23))));
    c.bench_function("day23b", |b| b.iter(|| day23::part2(black_box(day23))));

    let day24 = &read("day24.txt");
    c.bench_function("day24a", |b| b.iter(|| day24::part1(black_box(day24))));
    c.bench_function("day24b", |b| b.iter(|| day24::part2(black_box(day24))));

    let day25 = &read("day25.txt");
    c.bench_function("day25a", |b| b.iter(|| day25::part1(black_box(day25))));
}

criterion_group! {
    name = benches;
    config = Criterion::default().with_profiler(PProfProfiler::new(100, Output::Protobuf));
    targets = bench_2019
}

criterion_main!(benches);
