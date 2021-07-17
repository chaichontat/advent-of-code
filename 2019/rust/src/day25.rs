use std::collections::VecDeque;
use std::io;

use ahash::AHashSet;
use nohash_hasher::{IntMap, IntSet};
use regex::Regex;

use crate::intcode::*;

const END: isize = 10;
const SP: isize = 32;

const DROP: [isize; 5] = [100, 114, 111, 112, SP];
const TAKE: [isize; 5] = [116, 97, 107, 101, SP];

struct Droid {
    ic:          IntCode,
    bag:         AHashSet<String>,
    weight_room: Vec<isize>,
    pos:         VecDeque<isize>,
}

impl Droid {
    fn step(&mut self, dir: isize) {
        match dir {
            0 => self.ic.input.extend(&[110, 111, 114, 116, 104, END]), // N
            1 => self.ic.input.extend(&[115, 111, 117, 116, 104, END]), // S
            2 => self.ic.input.extend(&[119, 101, 115, 116, END]),      // W
            3 => self.ic.input.extend(&[101, 97, 115, 116, END]),       // E
            -2 => (),
            _ => unreachable!(),
        }
        self.ic.run_wait_input();
    }

    fn explore(&mut self, from: isize) {
        self.ic.run_wait_input();
        let out = self
            .ic
            .output
            .iter()
            .map(|&x| x as u8 as char)
            .collect::<String>();
        self.ic.output.clear();

        if out.contains("Security Checkpoint") {
            self.weight_room = Vec::from(self.pos.to_owned());
            return;
        }

        for chunk in out.split("\n\n") {
            if chunk.starts_with("Doors here lead:") {
                for dir in chunk.split('\n').skip(1) {
                    let new_dir = match dir {
                        "- north" => 0,
                        "- south" => 1,
                        "- west" => 2,
                        "- east" => 3,
                        _ => unreachable!(),
                    };
                    if new_dir != from {
                        self.pos.push_back(new_dir);
                        self.step(new_dir);
                        self.explore(new_dir ^ 1); // Recurse
                        self.step(new_dir ^ 1);
                        self.ic.output.clear();
                        self.pos.pop_back();
                    }
                }
            }
            if chunk.starts_with("Items here:") {
                for item in chunk.split('\n').skip(1) {
                    let obj = &item[2..];
                    match obj {
                        "photons"
                        | "molten lava"
                        | "giant electromagnet"
                        | "escape pod"
                        | "infinite loop" => continue,
                        _ => (),
                    }
                    self.call(&TAKE, obj);
                    self.bag.insert(obj.to_string());
                    self.ic.run_wait_input();
                    self.ic.output.clear();
                }
            }
        }
    }

    #[allow(dead_code)]
    fn interactive(&mut self) {
        loop {
            let s = self
                .ic
                .output
                .iter()
                .map(|&x| x as u8 as char)
                .collect::<String>();
            self.ic.output.clear();
            println!("{}", s);

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Couldn't read line");
            for c in input.chars() {
                self.ic.push(c as isize);
            }
        }
    }

    fn call(&mut self, op: &[isize], name: &str) {
        self.ic.input.extend(op);
        name.chars()
            .map(|x| x as u8 as isize)
            .for_each(|x| self.ic.push(x));
        self.ic.push(END);
        self.ic.run_wait_input();
    }

    fn trial(&mut self) -> Option<usize> {
        // Go into checkpoint.
        for dir in self.weight_room.to_owned() {
            self.step(dir);
            self.ic.output.clear();
        }

        let mut key = IntMap::default();
        self.bag.iter().enumerate().for_each(|(i, x)| {
            key.insert(i, x.to_owned());
        });
        let n = self.bag.len();

        let mut too_heavy = IntSet::default();
        let mut too_light = IntSet::default();
        let mut curr = (0..n).fold(0, |sum, i| sum | 1 << i);

        'outer: for mut i in (0..(1 << n)).rev() {
            i = i ^ (i >> 1); // Gray code.

            for &h in too_heavy.iter() {
                if (i & h) == i {
                    continue 'outer; // Current is superset of too heavy.
                }
            }

            for &l in too_light.iter() {
                if (l & i) == l {
                    continue 'outer; // Current is subset of too light.
                }
            }

            // Actual check.
            let diff = i ^ curr;
            for j in 0..n {
                if diff & (1 << j) > 0 {
                    // Changed
                    if curr & (1 << j) > 0 {
                        self.call(&DROP, key.get(&j).unwrap()); // Have, drop.
                        curr &= !(1 << j);
                    } else {
                        self.call(&TAKE, key.get(&j).unwrap()); // Don't have, pick.
                        curr |= 1 << j;
                    }
                    self.ic.output.clear();
                }
            }
            self.step(0); // Enter room.
            let s = self
                .ic
                .output
                .iter()
                .map(|&x| x as u8 as char)
                .collect::<String>();
            if s.contains("proceed") {
                let re = Regex::new(r"\d+").unwrap();
                return Some(re.find(&s).unwrap().as_str().parse::<usize>().unwrap());
            } else if s.contains("heavier") {
                too_heavy.insert(i);
            } else if s.contains("lighter") {
                too_light.insert(i);
            } else {
                unreachable!();
            }
            self.ic.output.clear();
        }
        None
    }
}

pub fn part1(raw: &[String]) -> usize {
    let mut droid = Droid {
        ic:          IntCode::from(&raw[0]),
        bag:         AHashSet::new(),
        weight_room: Vec::new(),
        pos:         VecDeque::new(),
    };
    droid.explore(-1);
    droid.trial().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::*;
    #[test]
    fn test1() {
        assert_eq!(part1(&read("day25.txt")), 1090529280);
    }
}
