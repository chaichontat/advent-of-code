use std::str::FromStr;

use ahash::{AHashMap, AHashSet};
use num_complex::Complex;

use super::utils::*;

fn gen_set(path: &str) -> AHashSet<Complex<isize>> {
    let dirs: Vec<(char, isize)> = path
        .split(',')
        .map(|x| (x.chars().next().unwrap(), x[1..].parse::<isize>().unwrap()))
        .collect();

    let mut curr = Complex::new(0, 0);
    let mut out = AHashSet::new();
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

fn gen_dist(path: &str) -> AHashMap<Complex<isize>, isize> {
    // HashMap is significantly slower.
    let dirs: Vec<(char, isize)> = path
        .split(',')
        .map(|x| (x.chars().next().unwrap(), x[1..].parse::<isize>().unwrap()))
        .collect();

    let mut curr = (Complex::new(0, 0), 0); // Location and total distance.
    let mut out = AHashMap::new();
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

pub fn part1(raw: &[String]) -> usize {
    gen_set(&raw[0])
        .intersection(&gen_set(&raw[1]))
        .map(Complex::l1_norm)
        .min()
        .unwrap() as usize
}

pub fn part2(raw: &[String]) -> usize {
    let maps = vec![gen_dist(&raw[0]), gen_dist(&raw[1])];
    let sets = vec![AHashMap::keys_to(&maps[0]), AHashMap::keys_to(&maps[1])];
    let intersect = sets[0].intersection(&sets[1]).collect::<Vec<_>>();
    intersect
        .iter()
        .map(|pos| maps[0][pos] + maps[1][pos])
        .min()
        .unwrap() as usize
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(part1(&read("day03.txt")), 217);
        assert_eq!(
            part1(&[
                String::from("R75,D30,R83,U83,L12,D49,R71,U7,L72"),
                String::from("U62,R66,U55,R34,D71,R55,D58,R83")
            ]),
            159
        );
        assert_eq!(
            part1(&[
                String::from("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"),
                String::from("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7")
            ]),
            135
        );
    }

    #[test]
    fn test2() {
        assert_eq!(part2(&read("day03.txt")), 3454);
        assert_eq!(
            part2(&[
                String::from("R75,D30,R83,U83,L12,D49,R71,U7,L72"),
                String::from("U62,R66,U55,R34,D71,R55,D58,R83")
            ]),
            610
        );
        assert_eq!(
            part2(&[
                String::from("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"),
                String::from("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7")
            ]),
            410
        );
    }
}
