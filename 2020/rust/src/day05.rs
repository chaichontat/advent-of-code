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

pub fn part1(raw: &Vec<String>) -> usize {
    raw.iter().map(|seat| process(seat)).max().unwrap()
}

pub fn part2(raw: &Vec<String>) -> usize {
    let seats: HashSet<usize> = HashSet::from_iter(raw.iter().map(|seat| process(seat)));
    let cand = seats
        .iter()
        .filter(|x| !seats.contains(&(*x + 1)) && seats.contains(&(*x + 2)))
        .map(|x| x + 1)
        .collect::<Vec<_>>();

    assert_eq!(cand.len(), 1);
    cand[0]
}

#[cfg(test)]
mod tests {
    use super::super::utils::*;
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(part1(&read("day05.txt")), 935)
    }

    #[test]
    fn test2() {
        assert_eq!(part2(&read("day05.txt")), 743);
    }
}
