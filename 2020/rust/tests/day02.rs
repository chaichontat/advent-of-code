extern crate advent_of_code_2020;

use advent_of_code_2020::utils::*;
use itertools::Itertools;

fn process(f: fn((Vec<usize>, char, String)) -> bool, ans: usize) {
    let input = read("day02.txt");
    let mut valid = 0;
    for x in input {
        let (num, c, pwd) = x.splitn(3, " ").collect_tuple().unwrap();
        let num = num
            .split("-")
            .map(|i| i.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let c = str_idx(&c.to_string(), 0);

        if f((num, c, pwd.to_string())) {
            valid += 1;
        }
    }
    assert_eq!(valid, ans);
}

#[test]
fn part1() {
    let f = |(num, c, pwd): (Vec<usize>, char, String)| -> bool {
        (num[0]..=num[1]).contains(&(pwd.matches(c).count()))
    };
    process(f, 447);
}

#[test]
fn part2() {
    let f = |(num, c, pwd): (Vec<usize>, char, String)| -> bool {
        (str_idx(&pwd, num[0] - 1) == c) ^ (str_idx(&pwd, num[1] - 1) == c)
    };
    process(f, 249);
}
