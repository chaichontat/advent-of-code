#[macro_use]
extern crate num_derive;

extern crate jemallocator;

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

pub mod utils;

pub mod day01;
pub mod day02;
pub mod day08;
pub mod day10;
pub mod day12;
pub mod day14;
// pub mod day23;
pub mod day24;
