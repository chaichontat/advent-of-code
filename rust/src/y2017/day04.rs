use std::mem;

use anyhow::anyhow;
use anyhow::Result;
use hashbrown::HashSet;
use itertools::Itertools;
use itertools::MinMaxResult::{MinMax, NoElements, OneElement};
use regex::Regex;
use thiserror::Error;

use crate::utils::printt;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Invalid character")]
    InvalidCharacter,

    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),

    #[error("Not enough numbers in a row")]
    NotEnough,
}

const MAX_NWORD: usize = 12;
const MAX_WORD_LEN: usize = 8;

type Parsed = [u64; MAX_NWORD];

pub fn parse(raw: &str) -> Result<Vec<Parsed>, ParseError> {
    if !raw.chars().all(|c| c.is_alphabetic() || c == '\n' || c == ' ') {
        return Err(ParseError::InvalidCharacter);
    }

    // let mut base = [0u8; MAX_WORD_LEN];
    // for (a, b) in base.iter_mut().zip(0..MAX_WORD_LEN) {
    //     *a = b as u8;
    // }
    // let mut arr = [base; MAX_NWORD];
    // for (a, b) in base.iter_mut().zip(0..MAX_WORD_LEN) {
    //     *a = b as u8;
    // }

    Ok(raw
        .split('\n')
        .map(|line| {
            let mut arr = [0u64; MAX_NWORD];
            for (word, a) in line.split(' ').zip(arr.iter_mut()) {
                for &c in word.as_bytes() {
                    *a = (*a << 8) | c as u64
                }
            }
            arr
        })
        .collect())
}

pub fn combi(parsed: &[Parsed]) -> (Option<u16>, Option<u16>) {
    let part1 = parsed
        .iter()
        .filter(|&line| {
            for i in 1..MAX_WORD_LEN {
                if line[i] == 0 {
                    break;
                }
                for j in 0..i {
                    if line[i] == line[j] {
                        return false;
                    }
                }
            }
            true
        })
        .count();

    let mut p = parsed
        .iter()
        .map(|&line| unsafe { mem::transmute::<_, [[u8; MAX_WORD_LEN]; MAX_NWORD]>(line) })
        .collect_vec();

    for line in p.iter_mut() {
        for word in line.iter_mut() {
            word.sort_unstable_by(|a, b| b.cmp(a));
        }
        line.sort_unstable_by(|a, b| b.cmp(a));
    }

    let part2 = p
        .iter()
        .filter(|&line| {
            let mut old = [0u8; MAX_WORD_LEN];
            for &word in line {
                if word == [0u8; MAX_WORD_LEN] {
                    return true;
                }
                if old != word {
                    old = word;
                } else {
                    return false;
                }
            }
            true
        })
        .count();

    // for u in part1 {
    //     if !set.contains(u) {
    //         printt(&u);
    //     }
    // }

    (Some(part1 as u16), Some(part2 as u16))
}

#[cfg(test)]
mod tests {
    use super::{combi, parse};
    use crate::utils::read;

    #[test]
    fn test_combi() {
        assert_eq!(
            combi(&parse(&read(2017, "day04.txt")).unwrap()),
            (Some(477), Some(167))
        );
    }
}
