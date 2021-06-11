extern crate advent_of_code_2020;

use advent_of_code_2020::utils;
use itertools::Itertools;

#[test]
fn part1() {
    let input = utils::read("day02.txt");
    let mut valid = 0;
    for x in input {
        let (num, c, pwd) = x.splitn(3, " ").collect_tuple().unwrap();
        let num = num
            .split("-")
            .map(|i| i.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let c = c.chars().nth(0).unwrap();

        if (num[0]..=num[1]).contains(&(pwd.matches(c).count() as i32)) {
            valid += 1;
        }
    }
    assert_eq!(valid, 447);
}
