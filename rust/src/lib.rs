#![feature(is_sorted)]
#![feature(stmt_expr_attributes)]
#![allow(non_snake_case)]
#![warn(unsafe_op_in_unsafe_fn)]

#[macro_use]
extern crate num_derive;

pub mod containers;
pub mod pathfinding;
// pub mod simd;
pub mod spatial;
pub mod utils;

pub mod y2020 {
    pub mod day01;
    pub mod day02;
    pub mod day03;
    pub mod day04;
    pub mod day05;
}

pub mod y2019 {
    pub mod day01;
    pub mod day02;
    pub mod day03;
    pub mod day04;
    pub mod day05;
    pub mod day06;
    pub mod day07;
    pub mod day08;
    pub mod day09;
    pub mod day10;
    pub mod day11;
    pub mod day12;
    pub mod day13;
    pub mod day14;
    pub mod day15;
    pub mod day16;
    pub mod day17;
    pub mod day18;
    pub mod day19;
    pub mod day20;
    pub mod day21;
    pub mod day22;
    pub mod day23;
    pub mod day24;
    pub mod day25;
    pub mod intcode;
}

pub mod y2018 {
    pub mod day01;
    pub mod day02;
    pub mod day03;
    pub mod day05;
    pub mod day07;
    pub mod day08;
    pub mod day09;
    pub mod day10;
    pub mod day11;
    pub mod day12;
    pub mod day13;
    pub mod day14;
    pub mod day15;
    pub mod day16;
    pub mod day17;
    pub mod day18;
    pub mod day19;
    pub mod day20;
    // pub mod day21;
    pub mod day22;
    pub mod day23;
    pub mod day24;
    pub mod day25;
}

pub mod y2017 {
    pub mod day01;
    pub mod day02;
    pub mod day04;
    pub mod day05;
}
