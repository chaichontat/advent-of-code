use itertools::Itertools;
use ndarray::prelude::*;
use regex::Regex;

use crate::{pathfinding::bfs_bucket, utils::ModAdd};

const GEO_X: u32 = 16807;
const GEO_Y: u32 = 48271;
const ERO_MOD: u32 = 20183;
const GEO_YM: u32 = GEO_Y % ERO_MOD; // Since we always mod.

const X_DIM: usize = 128;
const X_MAX: u8 = 80;
const Y_MAX_MARG: u16 = 8;

const ROCKY: u8 = 0;
const WET: u8 = 1;
const NARROW: u8 = 2;

type Parsed = (usize, usize, usize);
type ArrT = u8;

pub fn parse(raw: &str) -> Parsed {
    let re = Regex::new(r"\d+").unwrap();
    re.find_iter(raw)
        .map(|m| m.as_str().parse().unwrap())
        .collect_tuple()
        .unwrap()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Coord {
    x: u8,
    y: u16,
}

#[allow(dead_code)]
struct Cave {
    depth:  u32,
    target: Coord,
    map:    Array2<u8>,
    passed: Array2<u8>
}

impl Cave {
    fn new(depth: u32, target: Coord) -> Self {
        let ymax = Y_MAX_MARG as usize + target.y as usize;

        let mut typ = Array2::<ArrT>::zeros((ymax, X_DIM));
        let mut prev_geo = [0u32; X_MAX as usize];

        // Calculate erosion and type when y=0.
        prev_geo.iter_mut().zip(typ.iter_mut()).fold(depth, |ers, (geo, typ)| {
            *typ = 1 << (ers % 3) as ArrT;
            let next_ers = ers.mod_add(GEO_X, ERO_MOD);
            *geo = next_ers;
            next_ers
        });

        // At x=0    => f(y)    = (GEO_YMx + depth) % ERO_MOD
        // Otherwise => f(x, y) = (f(x-1, y) * f(x, y-1) + depth) % ERO_MOD
        // Then all mod 3.
        typ.outer_iter_mut()
            .enumerate()
            .skip(1) // Row-by-row. Multiplication by repeated addition.
            .fold(depth + GEO_YM, |x0, (y, mut yy)| {
                let mut carry = x0;

                for (x, (xx, geo)) in yy.iter_mut().zip(prev_geo.iter_mut()).enumerate() {
                    *xx = 1 << (carry % 3) as ArrT;
                    if y == target.y as usize && x == target.x as usize {
                        carry = 0; // At target.
                        *xx = 1 << (depth % 3) as ArrT;
                    }
                    carry = (*geo * carry + depth) % ERO_MOD;
                    *geo = carry;
                }

                x0.mod_add(GEO_YM, ERO_MOD)
            });

        Cave {
            depth,
            target,
            passed: Array2::<ArrT>::zeros((ymax, X_DIM)),
            map: typ,
        }
    }
}

#[allow(dead_code)]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Tool {
    Neither = 0b001,
    Torch   = 0b010,
    Climb   = 0b100,
}

impl Default for Tool {
    fn default() -> Self {
        Tool::Torch
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct CoordTool {
    x:    u8,
    y:    u16,
    dist: u16,
    curr: u8,
}

impl CoordTool {
    #[rustfmt::skip]
    fn successors(self, cave: &mut Cave) -> [Option<(Self, usize)>; 4] {
        let mut out = [None; 4];
        let mut it = out.iter_mut();

        let passed = cave.passed.get_mut([self.y as usize, self.x as usize]).unwrap();
        if *passed & self.curr != 0  { return out; }
        *passed |= self.curr;

        let valid_tool = 7 ^ cave.map[[self.y as usize, self.x as usize]];

        for next in [
            (self.y + 1            , self.x                , self.y >= cave.target.y),
            (self.y.wrapping_sub(1), self.x                , self.y <= cave.target.y),
            (self.y                , self.x + 1            , self.x >= cave.target.x),
            (self.y                , self.x.wrapping_sub(1), self.x <= cave.target.x)
        ] {
            if next.0 > cave.target.y + Y_MAX_MARG - 1 { continue; }
            if next.1 > X_MAX -1                       { continue; }

            let new_tool;
            let new_dist;
            let heuristic;

            if cave.map[[next.0 as usize, next.1 as usize]] == self.curr as u8 {
                let mut h = 2 * next.2 as u8;
                new_tool = self.curr ^ valid_tool;
                new_dist = 7 + 1;
                if self.curr == Tool::Torch as u8  { h += 7 };
                if new_tool  != Tool::Torch as u8  { h += 7 };
                heuristic = h;
            } else {
                heuristic = 2 * next.2 as u8;
                new_tool = self.curr;
                new_dist = 1;
            }

            *it.next().unwrap() = Some((
                CoordTool{x: next.1, y: next.0, dist: self.dist + new_dist, curr: new_tool}, 
                heuristic as usize
            ))
        }
        out
    }
}


pub fn combi(input: &Parsed) -> (u16, usize) {
    let depth = input.0 as u32;
    let target = Coord {
        x: input.1 as u8,
        y: input.2 as u16,
    };
    let mut cave = Cave::new(depth, target);

    let part1 = cave
        .map
        .slice(s![..target.y as usize + 1, ..target.x as usize + 1])
        .map(|x| (*x >> 1) as u16)
        .sum();

    let part2 = bfs_bucket(
        CoordTool{x: 0, y:0, dist:0, curr: Tool::Torch as u8},
        |&c | c.successors(&mut cave),
        |&c| {
            c.x == target.x && c.y == target.y && c.curr == Tool::Torch as u8
        },
        17
    ).unwrap();

    (part1, part2.dist.into())
}


mod tests {
    use super::{combi, parse};
    use crate::utils::read;

    #[test]
    fn test_combi() {
        assert_eq!(combi(&parse(&read(2018, "day22.txt"))), (7380, 1013));
    }
}
