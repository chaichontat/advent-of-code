use super::utils::*;
use itertools::Itertools;
use std::collections::HashSet;
use std::iter::FromIterator;

pub fn part1(raw: &Vec<String>) -> usize {
    let nums: HashSet<usize> = HashSet::from_iter(raw.iter().map(|x| int(&x)));
    let out = nums
        .iter()
        .filter_map(|x| {
            if nums.contains(&(2020usize.checked_sub(*x)?)) {
                Some(x * (2020 - x))
            } else {
                None
            }
        })
        .collect::<Vec<usize>>();

    assert_eq!(out.len(), 2);  // Commutativity of summation.
    out[0]
}

pub fn part2(raw: &Vec<String>) -> usize{
    let nums: HashSet<usize> = HashSet::from_iter(raw.iter().map(|x| int(&x)));
    let out = nums
        .iter()
        .combinations(2)
        .filter_map(|x| {
            if nums.contains(&(2020usize.checked_sub(x[0] + x[1])?)) {
                Some(x[0] * x[1] * (2020 - x[0] - x[1]))
            } else {
                None
            }
        })
        .collect::<Vec<usize>>();

    assert_eq!(out.len(), 3);
    out[0]
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(part1(&read("day01.txt")), 605364);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(&read("day01.txt")), 128397680);
    }
}