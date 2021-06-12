extern crate advent_of_code_2020;

use advent_of_code_2020::utils::*;

fn run(modx: usize, mody: usize) -> usize {
    let raw = read("day03.txt");
    let block_size = raw[0].len();
    let mut count: usize = 0;
    for i in 0..raw.len() {
        let y = mody * i;
        if y < raw.len() && str_idx(&raw[y], (modx * i) % block_size) == '#' {
            count += 1;
        }
    }
    count
}

#[test]
fn part1() {
    assert_eq!(run(3, 1), 265);
}

#[test]
fn part2() {
    assert_eq!(
        run(1, 2) * (1..8).step_by(2).map(|i| run(i, 1)).product::<usize>(),
        3154761400
    );
}
