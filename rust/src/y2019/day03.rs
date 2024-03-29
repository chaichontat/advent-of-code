use std::str::FromStr;

use hashbrown::{HashMap, HashSet};
use num_complex::Complex;

use crate::spatial::Dir;
use crate::utils::*;

type Parsed = Vec<(char, i32)>;

pub fn parse(raw: &str) -> Vec<Parsed> {
    raw.split('\n')
        .map(|line| {
            line.split(',')
                .map(|x| (x.chars().next().unwrap(), x[1..].parse().unwrap()))
                .collect()
        })
        .collect()
}

fn gen_set(dirs: &[(char, i32)]) -> HashSet<Complex<i32>> {
    let mut curr = Complex::new(0, 0);
    let mut out = HashSet::new();
    for (dir, mag) in dirs.iter() {
        let d = Complex::from(Dir::from_str(&String::from(*dir)).unwrap());
        for i in 0..=*mag {
            out.insert(curr + i * d);
        }
        curr += mag * d;
    }
    out.remove(&Complex::new(0, 0));
    out
}

fn gen_dist(dirs: &[(char, i32)]) -> HashMap<Complex<i32>, i32> {
    // HashMap is significantly slower.
    let mut curr = (Complex::new(0, 0), 0); // Location and total distance.
    let mut out = HashMap::new();
    for (dir, mag) in dirs.iter() {
        let d = Complex::from(Dir::from_str(&String::from(*dir)).unwrap());
        for i in 0..=*mag {
            out.entry(curr.0 + i * d).or_insert(curr.1 + i); // Since current step is monotonically increasing.
        }
        curr.0 += mag * d;
        curr.1 += mag;
    }
    out.remove(&Complex::new(0, 0));
    out
}

pub fn part1(parsed: &[Parsed]) -> u16 {
    let parsed = parsed.to_owned();
    gen_set(&parsed[0])
        .intersection(&gen_set(&parsed[1]))
        .map(Complex::l1_norm)
        .min()
        .unwrap() as u16
}

pub fn part2(parsed: &[Parsed]) -> u16 {
    let parsed = parsed.to_owned();
    let maps = vec![gen_dist(&parsed[0]), gen_dist(&parsed[1])];
    let sets = vec![HashMap::keys_to(&maps[0]), HashMap::keys_to(&maps[1])];
    let intersect = sets[0].intersection(&sets[1]).collect::<Vec<_>>();
    intersect
        .iter()
        .map(|pos| maps[0].get(*pos).unwrap() + maps[1].get(*pos).unwrap())
        .min()
        .unwrap() as u16
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(part1(&parse(&read(2019, "day03.txt"))), 217);
        assert_eq!(
            part1(&parse(&String::from(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"
            ))),
            159
        );
        assert_eq!(
            part1(&parse(&String::from(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            ))),
            135
        );
    }

    #[test]
    fn test2() {
        assert_eq!(part2(&parse(&read(2019, "day03.txt"))), 3454);
        assert_eq!(
            part2(&parse(&String::from(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"
            ))),
            610
        );
        assert_eq!(
            part2(&parse(&String::from(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            ))),
            410
        );
    }
}
