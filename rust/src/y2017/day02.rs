use anyhow::anyhow;
use anyhow::Result;
use itertools::Itertools;
use itertools::MinMaxResult::{MinMax, NoElements, OneElement};
use regex::Regex;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Invalid character")]
    InvalidCharacter,

    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),

    #[error("Not enough numbers in a row")]
    NotEnough,
}

pub fn parse(raw: &str) -> Result<Vec<Vec<u16>>, ParseError> {
    if !raw.chars().all(|c| c.is_numeric() || c == '\n' || c == '\t') {
        return Err(ParseError::InvalidCharacter);
    }

    let re = Regex::new(r"\d+").unwrap();
    raw.split('\n')
        .map(|line| {
            let u = re
                .find_iter(line)
                .map(|m| m.as_str().parse::<_>())
                .collect::<Result<Vec<u16>, _>>()?; // See https://doc.rust-lang.org/rust-by-example/error/iter_result.html#fail-the-entire-operation-with-collect

            if u.len() < 2 {
                return Err(ParseError::NotEnough);
            }
            Ok(u)
        })
        .collect()
}

pub fn combi(parsed: &[Vec<u16>]) -> (Option<u16>, Option<u16>) {
    let part1 = parsed
        .iter()
        .map(
            |line| {
                if let MinMax(x, y) = line.iter().minmax() {
                    Some(y - x)
                } else {
                    None
                }
            },
        )
        .sum();

    let part2 = parsed
        .iter()
        .map(|line| {
            line.iter().tuple_combinations().find_map(|(&x, &y)| {
                if x > y && x % y == 0 {
                    Some(x / y)
                } else if y > x && y % x == 0 {
                    Some(y / x)
                } else {
                    None
                }
            })
        })
        .sum();

    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::{combi, parse};
    use crate::utils::read;

    #[test]
    fn test_combi() {
        assert_eq!(
            combi(&parse(&read(2017, "day02.txt")).unwrap()),
            (Some(54426), Some(333))
        );
    }
}
