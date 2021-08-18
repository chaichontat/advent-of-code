use std::cmp::Ordering;

use itertools::Itertools;
use num::Integer;
use regex::Regex;

type Index = usize;
type Parsed = [isize; 3];

pub fn parse(raw: &str) -> Vec<Parsed> {
    let re = Regex::new(r"-?\d+").unwrap();
    raw.split('\n')
        .map(|line| {
            // let mut iter = (line);
            let p = re
                .find_iter(line)
                .map(|x| x.as_str().parse::<isize>().unwrap())
                .collect_tuple::<(_, _, _)>()
                .unwrap();
            [p.0, p.1, p.2]
        })
        .collect()
}

fn gen_moons(parsed: &[Parsed]) -> Vec<Moon> {
    parsed.iter().map(|p| Moon { pos: *p, vel: [0; 3] }).collect_vec()
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Moon {
    pos: [isize; 3],
    vel: [isize; 3],
}

impl Moon {
    pub fn gravity_axis(&mut self, other: &mut Self, i: Index) {
        match self.pos[i].cmp(&other.pos[i]) {
            Ordering::Greater => {
                self.vel[i] -= 1;
                other.vel[i] += 1;
            }
            Ordering::Less => {
                self.vel[i] += 1;
                other.vel[i] -= 1;
            }
            Ordering::Equal => (),
        }
    }

    pub fn step_axis(&mut self, i: Index) {
        self.pos[i] += self.vel[i];
    }

    pub fn energy(&self) -> isize {
        let pos = self.pos.iter().fold(0, |sum, x| sum + x.abs());
        let kin = self.vel.iter().fold(0, |sum, x| sum + x.abs());
        pos * kin
    }
}

fn simulate_axis(moons: &mut [Moon], ax: Index) {
    let n = moons.len();
    for i in 0..n {
        let (sel, oth) = moons.split_at_mut(i + 1); // Self, other
        for j in (i + 1)..n {
            sel[i].gravity_axis(&mut oth[j - (i + 1)], ax);
        }
    }
    moons.iter_mut().for_each(|moon| moon.step_axis(ax));
}

// Simulate by axis.
pub fn part1(parsed: &[Parsed]) -> usize {
    let mut moons = gen_moons(parsed);
    for _ in 0..1000 {
        for ax in 0..3 {
            simulate_axis(&mut moons, ax);
        }
    }
    moons.iter().fold(0, |sum, x| sum + x.energy()) as usize
}

fn compare_axis(moons: &[Moon], ori: &[Moon], ax: Index) -> bool {
    for (now, or) in moons.iter().zip(ori.iter()) {
        if now.pos[ax] != or.pos[ax] || now.vel[ax] != or.vel[ax] {
            return false;
        }
    }
    true
}

pub fn part2(parsed: &[Parsed]) -> usize {
    let moons_ori = gen_moons(parsed);
    let mut moons = moons_ori.clone();
    let mut out: [usize; 3] = [0; 3];
    let mut iter = 0;

    while out.iter().any(|&x| x == 0) {
        iter += 1;
        for (ax, o) in out.iter_mut().enumerate() {
            if *o == 0 {
                simulate_axis(&mut moons, ax);
                if compare_axis(&moons, &moons_ori, ax) {
                    *o = iter;
                }
            }
        }
    }
    out[0].lcm(&out[1]).lcm(&out[2])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::*;
    #[test]
    fn test1() {
        assert_eq!(part1(&parse(&read(2019, "day12.txt"))), 8287);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(&parse(&read(2019, "day12.txt"))), 528250271633772);
    }
}
