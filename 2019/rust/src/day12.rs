use std::cmp::Ordering;

use num::Integer;
use regex::Regex;

type Index = usize;

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

    pub fn step_axis(&mut self, i: Index) { self.pos[i] += self.vel[i]; }

    pub fn energy(&self) -> isize {
        let pos = self.pos.iter().fold(0, |sum, x| sum + x.abs());
        let kin = self.vel.iter().fold(0, |sum, x| sum + x.abs());
        pos * kin
    }
}

fn parse(raw: &[String]) -> Vec<Moon> {
    raw.iter()
        .map(|line| {
            let re = Regex::new(r"-?\d+").unwrap();
            let mut iter = re.find_iter(line);
            Moon {
                pos: [
                    iter.next().unwrap().as_str().parse::<isize>().unwrap(),
                    iter.next().unwrap().as_str().parse::<isize>().unwrap(),
                    iter.next().unwrap().as_str().parse::<isize>().unwrap(),
                ],
                vel: [0; 3],
            }
        })
        .collect::<Vec<_>>()
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
pub fn part1(raw: &[String]) -> usize {
    let mut moons = parse(raw);
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

pub fn part2(raw: &[String]) -> usize {
    let moons_ori = parse(raw);
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
        assert_eq!(part1(&read("day12.txt")), 8287);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(&read("day12.txt")), 528250271633772);
    }
}
