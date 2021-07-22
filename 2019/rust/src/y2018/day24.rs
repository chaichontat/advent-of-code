use std::str::{from_utf8, FromStr};

use itertools::Itertools;
use regex::Regex;
use strum_macros::EnumString;

use crate::utils::*;

type Bits = u8;

#[allow(non_camel_case_types)]
#[derive(Debug, EnumString)]
enum Attack {
    cold        = 1,
    radiation   = 2,
    slashing    = 4,
    bludgeoning = 8,
    fire        = 16,
}

#[derive(Debug)]
pub struct Units {
    n:          u32,
    hp:         u32,
    attack:     Bits,
    dp:         u32,
    initiative: u32,
    weak:       Bits,
    immune:     Bits,
}

fn miniparse(lines: &[String]) -> Option<Vec<Units>> {
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

type Parsed = Vec<String>;

pub fn parse(raw: &str) -> Vec<Parsed> {
    raw.split("\n\n")
        .map(|s| s.split('\n').skip(1).map(String::from).collect_vec())
        .collect()
}

pub fn gen_unit(raw2: &[Parsed]) -> Option<(Vec<Units>, Vec<Units>)> {
    let imm = miniparse(&raw2[0][..])?;
    let infect = miniparse(&raw2[1][..])?;

    Some((imm, infect))
}

pub fn part1(parsed: &[Parsed]) -> usize {
    let parsed = gen_unit(parsed);
    let (imm, infect) = parsed.as_ref().unwrap();
    printt(&imm);
    0
}

mod tests {
    use super::*;

    // #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(&read(2018, "day24.txt"))), 584);
    }
}
