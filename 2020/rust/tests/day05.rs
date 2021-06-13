extern crate advent_of_code_2020;

use advent_of_code_2020::utils::*;

use std::collections::HashSet;
use std::iter::FromIterator;
/// This is basically a binary representation of a number.

fn bin_to_dec(x: &str, zero: &char) -> usize {
    x.chars()
        .enumerate()
        .filter(|(_, c)| c == zero)
        .map(|(i, _)| 2usize.pow((x.len() - 1 - i) as u32))
        .sum()
}

fn process(x: &str) -> usize {
    let r = bin_to_dec(&x[..7], &'B');
    let c = bin_to_dec(&x[7..], &'R');
    8 * r + c
}

#[test]
fn part1() {
    let raw = read("day05.txt");
    assert_eq!(raw.iter().map(|seat| process(seat)).max().unwrap(), 935)
}

#[test]
fn part2() {
    let raw = read("day05.txt");
    let seats: HashSet<usize> = HashSet::from_iter(raw.iter().map(|seat| process(seat)));
    let cand = seats
        .iter()
        .filter(|x| !seats.contains(&(*x + 1)) && seats.contains(&(*x + 2)))
        .map(|x| x + 1)
        .collect::<Vec<_>>();

    assert_eq!(cand.len(), 1);
    assert_eq!(cand[0], 743);
}
