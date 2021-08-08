use ndarray::prelude::*;
use num::PrimInt;
use regex::Regex;

type Parsed = Clay<u16>;
type Idx = [usize; 2];

#[derive(Debug, Default, Clone, Copy)]
pub struct Clay<T: PrimInt> {
    xmin: T,
    xmax: T,
    ymin: T,
    ymax: T,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Elem {
    Void = 0,
    Wall = 1,
    Pool = 2,
    Flow = 3,
}

enum Dir {
    L,
    R,
}

struct Map {
    m:      Array2<Elem>,
    ybound: (usize, usize),
    xmin:   u16,
}

impl Map {
    fn new(walls: &[Clay<u16>]) -> Self {
        let xmin = walls.iter().map(|c| c.xmin).min().unwrap();
        let xmax = walls.iter().map(|c| c.xmax).max().unwrap();
        let ymin = walls.iter().map(|c| c.ymin).min().unwrap();
        let ymax = walls.iter().map(|c| c.ymax).max().unwrap();

        let mut arr: Array<Elem, _> =
            Array::from_elem((ymax as usize + 2, (xmax - xmin) as usize + 2), Elem::Void);
        for w in walls {
            arr.slice_mut(s![
                w.ymin as usize..w.ymax as usize,
                (w.xmin - xmin) as usize..(w.xmax - xmin) as usize
            ])
            .assign(&array![[Elem::Wall]]);
        }

        Map {
            m: arr,
            ybound: (ymin as usize, ymax as usize),
            xmin,
        }
    }

    #[allow(dead_code)]
    fn print(&self) {
        use Elem::*;
        println!();
        for y in self.m.outer_iter() {
            for x in y {
                let u = match *x {
                    Void => ".",
                    Wall => "#",
                    Pool | Flow => "|",
                };
                print!("{}", u);
            }
            println!();
        }
        println!();
    }

    fn has_wall(&self, mut water: Idx, dir: Dir) -> bool {
        loop {
            let x = match dir {
                Dir::L => self.m[[water[1], water[0] - 1]],
                Dir::R => self.m[[water[1], water[0] + 1]],
            };

            if x == Elem::Wall || x == Elem::Pool {
                return true;
            } else if x == Elem::Void {
                return false;
            }

            water = match dir {
                Dir::L => [water[0] - 1, water[1]],
                Dir::R => [water[0] + 1, water[1]],
            };
        }
    }

    fn has_both_walls(&self, water: Idx) -> bool {
        self.has_wall(water, Dir::L) && self.has_wall(water, Dir::R)
    }

    fn fill_pool(&mut self, mut water: Idx) {
        let ori = water;
        while self.m[[water[1], water[0]]] != Elem::Wall {
            self.m[[water[1], water[0]]] = Elem::Pool;
            water = [water[0] - 1, water[1]];
        }
        water = ori;
        while self.m[[water[1], water[0]]] != Elem::Wall {
            self.m[[water[1], water[0]]] = Elem::Pool;
            water = [water[0] + 1, water[1]];
        }
    }

    fn fill_all(&mut self, water: Idx) -> bool {
        // self.print();
        use Elem::*;

        self.m[[water[1], water[0]]] = Flow;

        if water[1] > self.ybound.1 - 1 {
            return true;
        }

        if self.m[[water[1] + 1, water[0]]] == Void && self.fill_all([water[0], water[1] + 1]) {
            return true;
        }

        if self.m[[water[1] + 1, water[0]]] == Flow {
            return true;
        }

        let mut reached = false;
        if water[0] > 0 && self.m[[water[1], water[0] - 1]] == Void {
            reached = self.fill_all([water[0] - 1, water[1]]) || reached
        }
        if water[0] < self.m.dim().1 - 1 && self.m[[water[1], water[0] + 1]] == Void {
            reached = self.fill_all([water[0] + 1, water[1]]) || reached
        }
        if self.has_both_walls(water) {
            self.fill_pool(water);
        }

        reached
    }
}

pub fn parse(raw: &str) -> Vec<Parsed> {
    let mut out = Vec::new();
    let re = Regex::new(r"(x|y)=(\d+), (x|y)=(\d+)..(\d+)").unwrap();

    for line in raw.split('\n') {
        let c = re.captures(line).unwrap();
        let mut clay = Clay::<u16>::default();
        if &c[1] == "x" {
            clay.xmin = c[2].parse().unwrap();
            clay.xmax = clay.xmin + 1;
            clay.ymin = c[4].parse().unwrap();
            clay.ymax = c[5].parse::<u16>().unwrap() + 1;
        } else {
            clay.xmin = c[4].parse().unwrap();
            clay.xmax = c[5].parse::<u16>().unwrap() + 1;
            clay.ymin = c[2].parse().unwrap();
            clay.ymax = clay.ymin + 1;
        }
        out.push(clay);
    }

    out
}

pub fn combi(walls: &[Parsed]) -> Option<(usize, usize)> {
    let mut map = Map::new(walls);
    map.fill_all([500 - map.xmin as usize, 0]);

    let flow = map
        .m
        .slice(s![map.ybound.0..map.ybound.1, ..])
        .iter()
        .filter(|&&x| x == Elem::Flow)
        .count();

    let pool = map
        .m
        .slice(s![map.ybound.0..map.ybound.1, ..])
        .iter()
        .filter(|&&x| x == Elem::Pool)
        .count();

    Some((flow + pool, pool))
}

#[cfg(test)]
mod tests {
    use super::{combi, parse};
    use crate::utils::read;

    #[test]
    fn test_combi() {
        assert_eq!(combi(&parse(&read(2018, "day17.txt"))), Some((34775, 27086)));
    }
}
