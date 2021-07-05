use nohash_hasher::IntMap;

use super::intcode::*;

struct IC {
    mem: Vec<isize>,
}

type Pos = (isize, isize);

impl IC {
    fn run(&self, i: isize, j: isize) -> isize {
        let mut ic = IntCode::from(&self.mem[..]);
        ic.push(i);
        ic.push(j);
        ic.run_pause();
        ic.pop().unwrap()
    }

    fn binary_search<F>(&self, f: F, mut lo: isize, mut hi: isize) -> isize
    where F: Fn(isize) -> isize {
        while hi - lo > 1 {
            let mid = (hi + lo) / 2;
            match f(mid) {
                0 => lo = mid,
                1 => hi = mid,
                _ => unreachable!(),
            }
        }
        lo
    }

    fn search_edge(&self, (mut x, mut y): Pos, key: isize) -> IntMap<isize, isize> {
        let mut out = IntMap::default();
        out.insert(y, x);
        while y > 0 {
            y -= 1;
            while self.run(x, y) != key && x > 0 {
                x -= 1;
            }
            out.insert(y, x);
        }
        out
    }

    fn get_bounds(&self, start: isize) -> (isize, isize) {
        let (mut x, y) = (start, start);
        while self.run(x, y) == 0 {
            // Beam always lower than diagonal.
            x -= 1;
        }
        let upper = x;
        let lower = self.binary_search(|x| self.run(x, start), 0, start);
        (lower, upper)
    }

    fn test_square(&self, mut x: isize, mut y: isize, s: isize) -> (isize, isize) {
        assert_eq!((self.run(x, y + s - 1), self.run(x + s - 1, y)), (0, 0));
        loop {
            // println!("{} {}", x, y);
            match (self.run(x, y + s - 1), self.run(x + s - 1, y)) {
                (0, 0) => {
                    x += 1;
                    y += 1
                }
                (1, 0) => y += 1,
                (0, 1) => x += 1,
                (1, 1) => break,
                _ => unreachable!(),
            }
        }
        assert_eq!(self.run(x - 1, y + s - 1), 0);
        assert_eq!(self.run(x + s, y), 0);
        (x, y)
    }
}

pub fn part1(raw: &[String]) -> usize {
    let ic =
        IC { mem: raw[0].split(',').map(|x| x.parse::<isize>()).flatten().collect::<Vec<_>>() };

    let beam = ic.get_bounds(49);
    let lower = ic.search_edge((beam.0, 49), 0);
    let upper = ic.search_edge((beam.1, 49), 1);
    let u: isize = (0..50)
        .map(|x| upper.get(&x).unwrap() - lower.get(&x).unwrap())
        .filter(|&x| x > 0)
        .sum::<isize>();
    (u + 1) as usize // 1 at (0,0).
}

pub fn part2(raw: &[String]) -> usize {
    let ic =
        IC { mem: raw[0].split(',').map(|x| x.parse::<isize>()).flatten().collect::<Vec<_>>() };
    let beam = ic.get_bounds(100);
    let (a, b) = (100. / (beam.1 + 1) as f32, 100. / beam.0 as f32);

    let s = 100.;
    let x0 = (-s + 1. - a * s + a) / (a - b);
    let y0 = a * (x0 + s - 1.);
    let ans = &ic.test_square(x0 as isize, y0 as isize, 100);
    (ans.0 * 10_000 + ans.1) as usize
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::*;
    #[test]
    fn test1() {
        assert_eq!(part1(&read("day19.txt")), 197);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(&read("day19.txt")), 9181022);
    }
}
