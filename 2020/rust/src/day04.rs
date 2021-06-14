use super::utils::*;
use itertools::Itertools;
use std::str::FromStr;
use strum_macros::EnumString;

fn process(raw: &Vec<String>) -> Vec<String> {
    let seped: Vec<String> = raw.join("\n").split("\n\n").map(str::to_string).collect();
    seped
        .into_iter()
        .filter(|x| {
            gen_re(r"[bie]yr").find_iter(&x).count() == 3
                && gen_re(r"hgt").is_match(&x)
                && gen_re(r"[he]cl").find_iter(&x).count() == 2
                && gen_re(r"pid").is_match(&x)
        })
        .collect::<Vec<_>>()
}

#[allow(non_camel_case_types)]
#[derive(EnumString)]
enum Criteria {
    byr,
    iyr,
    eyr,
    hgt,
    hcl,
    ecl,
    pid,
    cid,
}

fn proc_hgt(x: &str) -> bool {
    let mut x = x.to_string();

    let unit = x.split_off(x.len() - 2);
    let x = x.parse::<usize>().unwrap();

    match unit.as_str() {
        "cm" => (150..=193).contains(&x),
        "in" => (59..=76).contains(&x),
        _ => false,
    }
}

fn check(s: &str) -> bool {
    let (k, x) = s.split(":").collect_tuple().unwrap();
    match Criteria::from_str(k).unwrap() {
        Criteria::byr => (1920..=2002).contains(&(x.parse::<usize>().unwrap())),
        Criteria::iyr => (2010..=2020).contains(&(x.parse::<usize>().unwrap())),
        Criteria::eyr => (2020..=2030).contains(&(x.parse::<usize>().unwrap())),
        Criteria::hgt => proc_hgt(x),
        Criteria::hcl => gen_re(r"#[0-9a-f]{6}").is_match(&x),
        Criteria::ecl => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&x),
        Criteria::pid => x.parse::<usize>().is_ok() && x.len() == 9,
        Criteria::cid => x.len() > 0,
    }
}

pub fn part1(raw: &Vec<String>) -> usize {
    process(raw).len()
}

pub fn part2(raw: &Vec<String>) -> usize {
    process(raw)
        .iter()
        .filter(|line| line.split_whitespace().all(check))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(part1(&read("day04.txt")), 190);
    }
    #[test]
    fn test2() {
        assert_eq!(part2(&read("day04.txt")), 121);
    }
}
