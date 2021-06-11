extern crate advent_of_code_2020;

use advent_of_code_2020::utils;

#[test]
fn part1() {
    let input = utils::read("day01.txt");
    for i in 0..input.len() {
        for j in i..input.len() {
            if i + j == 2020 {
                assert_eq!(i * j, 605364);
            }
        }
    }
}

#[test]
fn part2() {
    let input = utils::read("day01.txt");
    for i in 0..input.len() {
        for j in i..input.len() {
            for k in j..input.len() {
                if i + j + k == 2020 {
                    assert_eq!(i * j * k, 128397680);
                }
            }
        }
    }
}
