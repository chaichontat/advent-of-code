use itertools::Itertools;
use ndarray::prelude::*;
use regex::Regex;

use crate::pathfinding::bfs_bucket;
use crate::utils::ModAdd;

const GEO_X: u32 = 16807;
const GEO_Y: u32 = 48271;
const ERO_MOD: u32 = 20183;
const GEO_YM: u32 = GEO_Y % ERO_MOD; // Since we always mod.

const X_MARGIN: u8 = 60;
const Y_MARGIN: u16 = 8;

#[allow(dead_code)]
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
enum Topo {
    Rocky  = 0b001,
    Wet    = 0b010,
    Narrow = 0b100,
}

#[allow(dead_code)]
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
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
        let xmax = X_MARGIN as usize + target.x as usize;
        let ymax = Y_MARGIN as usize + target.y as usize;

        let mut typ = Array2::<ArrT>::zeros((ymax, xmax));
        let mut prev_geo = vec![0u32; xmax];

        // Calculate erosion and type when y=0.
        prev_geo
            .iter_mut()
            .zip(typ.iter_mut())
            .fold(depth, |ers, (geo, typ)| {
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

        Cave { depth, target, map: typ }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct CoordTool {
    x:    u8,
    y:    u16,
    dist: u16,
    tool: u8,
}

impl CoordTool {
    fn successors(self, cave: &Cave, used_tools: &mut Array2<u8>) -> [Option<(Self, usize)>; 4] {
        let mut succ = [None; 4];
        let mut succ_it = succ.iter_mut();

        let p = used_tools.get_mut([self.y as usize, self.x as usize]).unwrap();
        if *p & self.tool != 0 {
            return succ;
        }
        *p |= self.tool;

        let valid_tool = 7 ^ cave.map[[self.y as usize, self.x as usize]];

        #[rustfmt::skip]
        for next in [
            (self.y + 1            , self.x                , self.y >= cave.target.y),
            (self.y.wrapping_sub(1), self.x                , self.y <= cave.target.y),
            (self.y                , self.x + 1            , self.x >= cave.target.x),
            (self.y                , self.x.wrapping_sub(1), self.x <= cave.target.x),
        ] {
            if next.0 > cave.target.y + Y_MARGIN - 1 || next.1 > cave.target.x + X_MARGIN - 1{
                continue;
            }

            let new_tool;
            let new_dist;
            let heuristic;

            if cave.map[[next.0 as usize, next.1 as usize]] == self.tool as u8 {
                let mut h = 2 * next.2 as u8;
                new_tool = self.tool ^ valid_tool;
                new_dist = 7 + 1;
                if self.tool == Tool::Torch as u8  { h += 7 };
                if new_tool  != Tool::Torch as u8  { h += 7 };
                heuristic = h;
            } else {
                heuristic = 2 * next.2 as u8;
                new_tool = self.tool;
                new_dist = 1;
            }

            *succ_it.next().unwrap() = Some((
                CoordTool {
                    x:    next.1,
                    y:    next.0,
                    dist: self.dist + new_dist,
                    tool: new_tool,
                },
                heuristic as usize,
            ))
        }
        succ
    }
}

pub fn combi(input: &Parsed) -> (u16, usize) {
    let depth = input.0 as u32;
    let target = Coord { x: input.1 as u8, y: input.2 as u16 };
    let cave = Cave::new(depth, target);

    let part1 = cave
        .map
        .slice(s![..target.y as usize + 1, ..target.x as usize + 1])
        .map(|x| (*x >> 1) as u16)
        .sum();

    let mut used_tools = Array2::<u8>::zeros(cave.map.dim());
    let part2 = bfs_bucket(
        CoordTool {
            x:    0,
            y:    0,
            dist: 0,
            tool: Tool::Torch as u8,
        },
        |&c| c.successors(&cave, &mut used_tools),
        |&c| c.x == target.x && c.y == target.y && c.tool == Tool::Torch as u8,
        17,
    )
    .unwrap();

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
