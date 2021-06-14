extern crate advent_of_code_2020;

use advent_of_code_2020::utils::*;
use advent_of_code_2020::*;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_fi(c: &mut Criterion) {
    c.bench_function("day01a", |b| {b.iter(|| day01::part1(black_box(&read("day01.txt"))))});
    c.bench_function("day01b", |b| {b.iter(|| day01::part2(black_box(&read("day01.txt"))))});
    c.bench_function("day02a", |b| {b.iter(|| day02::part1(black_box(&read("day02.txt"))))});
    c.bench_function("day02b", |b| {b.iter(|| day02::part2(black_box(&read("day02.txt"))))});
    c.bench_function("day03a", |b| {b.iter(|| day03::part1(black_box(&read("day03.txt"))))});
    c.bench_function("day03b", |b| {b.iter(|| day03::part2(black_box(&read("day03.txt"))))});
    c.bench_function("day04a", |b| {b.iter(|| day04::part1(black_box(&read("day04.txt"))))});
    c.bench_function("day04b", |b| {b.iter(|| day04::part2(black_box(&read("day04.txt"))))});
    c.bench_function("day05a", |b| {b.iter(|| day05::part1(black_box(&read("day05.txt"))))});
    c.bench_function("day05b", |b| {b.iter(|| day05::part2(black_box(&read("day05.txt"))))});
}

criterion_group!(benches, bench_fi);
criterion_main!(benches);
