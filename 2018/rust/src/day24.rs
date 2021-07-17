use crate::utils::*;
use itertools::Itertools;
use regex::Regex;
use std::str::{from_utf8, FromStr};
use strum_macros::EnumString;

type Bits = u8;

#[allow(non_camel_case_types)]
#[derive(Debug, EnumString)]
enum Attack {
    cold = 1,
    radiation = 2,
    slashing = 4,
    bludgeoning = 8,
    fire = 16,
}

#[derive(Debug)]
pub struct Units {
    n: u32,
    hp: u32,
    attack: Bits,
    dp: u32,
    initiative: u32,
    weak: Bits,
    immune: Bits,
}

fn miniparse(lines: &[&str]) -> Option<Vec<Units>> {
    let re = [
        Regex::new(r"(\d+)").unwrap(),
        Regex::new(r"does \d+ ([a-z]+)").unwrap(),
        Regex::new(r"\((.*)\)").unwrap(),
    ];
    let mut out = Vec::new();
    for line in lines {
        let (n, hp, dp, initiative) = re[0]
            .find_iter(line)
            .map(|m| m.as_str().parse::<u32>().unwrap())
            .collect_tuple()?;

        let attack = Attack::from_str(re[1].captures(line)?.get(1)?.as_str()).unwrap() as Bits;

        let (mut weak, mut immune) = (0, 0);
        if let Some(wi) = re[2].captures(line) {
            // Have parenthesis.
            let wi = wi.get(1)?;
            for y in wi.as_str().split("; ") {
                if y.starts_with("weak to ") {
                    weak = from_utf8(&y.as_bytes()[8..])
                        .unwrap()
                        .split(", ")
                        .map(|a| Attack::from_str(a).unwrap() as Bits)
                        .sum();
                } else if y.starts_with("immune to ") {
                    immune = from_utf8(&y.as_bytes()[10..])
                        .unwrap()
                        .split(", ")
                        .map(|a| Attack::from_str(a).unwrap() as Bits)
                        .sum();
                }
            }
        }
        out.push(Units {
            n,
            hp,
            attack,
            dp,
            initiative,
            weak,
            immune,
        })
    }
    Some(out)
}

pub fn parse(raw: String) -> Option<(Vec<Units>, Vec<Units>)> {
    printt(&"hi");
    let mut big_it = raw
        .split("\n\n")
        .map(|s| s.split('\n').skip(1).collect_vec());

    let imm = miniparse(&big_it.next()?)?;
    let infect = miniparse(&big_it.next()?)?;

    Some((imm, infect))
}

pub fn part1(parsed: &Option<(Vec<Units>, Vec<Units>)>) -> usize {
    let (imm, infect) = parsed.as_ref().unwrap();
    printt(&imm);
    0
}

mod tests {
    use super::*;

    // #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(read_nosep("day24.txt"))), 584);
    }
}
