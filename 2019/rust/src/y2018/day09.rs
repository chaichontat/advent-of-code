use std::collections::VecDeque;

use itertools::Itertools;
use regex::Regex;

type Parsed = (usize, usize); // n_player, last_marble

pub fn parse(raw: &str) -> Parsed {
    let re = Regex::new(r"\d+").unwrap();
    re.find_iter(raw)
        .map(|x| x.as_str().parse::<usize>().unwrap())
        .collect_tuple()
        .unwrap()
}

fn rot(n_player: usize, n_mb: usize) -> u32 {
    let mut v = VecDeque::with_capacity(n_mb + 1);
    v.push_front(0);
    let mut scores = vec![0u32; n_player];

    for m in 1..=n_mb as u32 {
        if m % 23 == 0 {
            v.rotate_right(7);
            scores[m as usize % n_player] += m + v.pop_back().unwrap();
            v.rotate_left(1);
        } else {
            v.rotate_left(1);
            v.push_back(m);
        }
    }
    *scores.iter().max().unwrap()
}

pub fn combi(&(n_player, n_mb): &Parsed) -> (u32, u32) {
    (rot(n_player, n_mb), rot(n_player, 100 * n_mb))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::*;

    #[test]
    fn test_combi() {
        assert_eq!(combi(&parse(&read(2018, "day09.txt"))), (374690, 3009951158));
    }
}
