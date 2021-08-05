use std::mem;

use itertools::Itertools;
use ndarray::prelude::*;
use pathfinding::prelude::{absdiff, astar};
use regex::Regex;

use crate::utils::ModAdd;

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
}

impl Cave {
    fn new(depth: u32, target: Coord) -> Self {
        let ymax = Y_MAX_MARG as usize + target.y as usize;

        let mut typ = Array2::<ArrT>::zeros((ymax, X_DIM));
        let mut prev_geo = [0u32; X_MAX as usize];

        // Calculate erosion and type when y=0.
        prev_geo.iter_mut().zip(typ.iter_mut()).fold(depth, |ers, (geo, typ)| {
            *typ = (ers % 3) as ArrT;
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
                    *xx = (carry % 3) as ArrT;
                    if y == target.y as usize && x == target.x as usize {
                        carry = 0; // At target.
                        *xx = (depth % 3) as ArrT;
                    }
                    carry = (*geo * carry + depth) % ERO_MOD;
                    *geo = carry;
                }

                x0.mod_add(GEO_YM, ERO_MOD)
            });

        Cave {
            depth,
            target,
            map: typ,
        }
    }
}

pub fn combi(input: &Parsed) -> (u16, usize) {
    let depth = input.0 as u32;
    let target = Coord {
        x: input.1 as u8,
        y: input.2 as u16,
    }; //Coord { x: 10, y:10}; //
    let cave = Cave::new(depth, target);

    let part1 = cave
        .map
        .slice(s![..target.y as usize + 1, ..target.x as usize + 1])
        .map(|x| *x as u16)
        .sum();

    // let curr_tool = Some(Tools::Climb);
    let part2 = astar(
        &CoordTool::default(),
        |c: &CoordTool| c.successors(&cave),
        |c| absdiff(c.x as usize, target.x as usize) + absdiff(c.y as usize, target.y as usize),
        |&c| {
            c == CoordTool {
                x:    target.x,
                y:    target.y,
                curr: Tool::Torch,
            }
        },
    )
    .unwrap();

    (part1, part2.1)
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Tool {
    Neither = 0,
    Torch   = 1,
    Climb   = 2,
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
    curr: Tool,
}

impl CoordTool {
    #[rustfmt::skip]
    fn successors(&self, cave: &Cave) -> Vec<(Self, usize)> {
        let mut out = Vec::new();


        // Case 1: Switch tool
        for i in 0..3 {
            if i != self.curr as u8 && i != cave.map[[self.y as usize, self.x as usize]] {
                let mut new = *self;
                new.curr = unsafe { mem::transmute(i) };
                out.push((new, 7));
            }
        }

        // Case 2: Step
        for next in [
            (self.y + 1            , self.x    ),
            (self.y.wrapping_sub(1), self.x    ),
            (self.y                , self.x + 1),
            (self.y                , self.x.wrapping_sub(1))
        ] {
            if next.0 > cave.target.y + Y_MAX_MARG - 1   { continue; }
            if next.1 > X_MAX -1                         { continue; }
            if cave.map[[next.0 as usize, next.1 as usize]] == self.curr as u8 { continue; } 
            out.push((CoordTool{x: next.1, y: next.0, curr: self.curr}, 1))
        }
        out
    }
}

mod tests {
    use super::{combi, parse};
    use crate::utils::read;

    #[test]
    fn test_combi() {
        assert_eq!(combi(&parse(&read(2018, "day22.txt"))), (7380, 1013));
    }
}
