use std::{collections::BinaryHeap, ops::Range};

use ahash::AHashSet;
use num::rational::Ratio;
use num_complex::Complex;

type Pos = Complex<i32>;

fn parse(raw: &[String]) -> AHashSet<Pos> {
    let mut out = AHashSet::new();
    for (line, y) in raw.iter().zip(Range {
        start: 0i32,
        end:   raw.len() as i32,
    }) {
        line.chars()
            .zip(Range {
                start: 0i32,
                end:   raw[0].len() as i32,
            })
            .for_each(|(c, x)| match c {
                '.' => (),
                '#' => {
                    out.insert(Complex::new(x, -y));
                }
                _ => unreachable!(),
            })
    }
    out
}

fn count(asteroids: &AHashSet<Pos>, ast: Pos) -> usize {
    let mut fracs_p = AHashSet::new();
    let mut fracs_n = AHashSet::new(); // Since division mods out negative.
    let (mut top, mut bottom) = (0, 0); // Top and bottom. ∞ slope.
    for &nbh in asteroids.iter() {
        if ast == nbh {
            continue;
        }
        let shifted = nbh - ast; //(ast.0 - nbh.0, ast.1 - nbh.1);
        if shifted.re > 0 {
            // Split at y = 0.
            let frac = Ratio::new(shifted.im, shifted.re); // slope
            fracs_p.insert(frac);
        } else if shifted.re < 0 {
            let frac = Ratio::new(shifted.im, shifted.re);
            fracs_n.insert(frac);
        } else if shifted.im > 0 {
            top = 1
        } else if shifted.im < 0 {
            bottom = 1
        } else {
            unreachable!()
        }
    }
    fracs_p.len() + fracs_n.len() + top + bottom
}

type Neighbors = Vec<(Ratio<i32>, i32, (i32, i32))>;
fn list_nbr(asteroids: &AHashSet<Pos>, ast: Pos) -> (Neighbors, Neighbors) {
    let mut fracs_p = BinaryHeap::new();
    let mut fracs_n = BinaryHeap::new(); // Since division mods out negative.
    for &nbh in asteroids.iter() {
        if ast == nbh {
            continue;
        }
        let shifted = nbh - ast; //(ast.0 - nbh.0, ast.1 - nbh.1);
        let norm = shifted.norm_sqr();

        if shifted.re > 0 {
            fracs_p.push((-Ratio::new(shifted.im, shifted.re), norm, (nbh.re, nbh.im)));
        } else if shifted.re < 0 {
            fracs_n.push((-Ratio::new(shifted.im, shifted.re), norm, (nbh.re, nbh.im)));
        } else if shifted.im > 0 {
            fracs_p.push((Ratio::new(i32::MIN, 1), norm, (nbh.re, nbh.im)));
        } else if shifted.im < 0 {
            fracs_n.push((Ratio::new(i32::MIN, 1), norm, (nbh.re, nbh.im)));
        } else {
            unreachable!()
        }
    }
    (fracs_p.into_sorted_vec(), fracs_n.into_sorted_vec())
}

fn scan((fracs_p, fracs_n): &mut (Neighbors, Neighbors), target: usize) -> Option<(i32, i32)> {
    fn sub_scan(fracs: &mut Neighbors, n: &mut usize, target: usize) -> Option<(i32, i32)> {
        let mut i = 0;
        let mut prev = None;
        while let Some(x) = fracs.get(i) {
            if prev == Some(x.0) {
                i += 1;
                continue;
            }

            if *n == target {
                return Some(x.2);
            }

            let x = fracs.remove(i);
            prev = Some(x.0);
            *n += 1;
        }
        None
    }

    let mut n = 1;
    while !fracs_p.is_empty() || !fracs_n.is_empty() {
        if let Some(x) = sub_scan(fracs_p, &mut n, target) {
            return Some(x);
        }

        if let Some(x) = sub_scan(fracs_n, &mut n, target) {
            return Some(x);
        }
    }
    None
}

pub fn bench(raw: &[String]) -> (usize, usize) {
    let asteroids = parse(raw);
    let (argmax, part1) = asteroids
        .iter()
        .map(|&ast| (ast, count(&asteroids, ast)))
        .max_by_key(|x| x.1)
        .unwrap();
    let mut nbrs = list_nbr(&asteroids, argmax);
    let res = scan(&mut nbrs, 200).unwrap();
    (part1, (res.0 * 100 - res.1) as usize)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::*;
    #[test]
    fn test() {
        assert_eq!(bench(&read("day10.txt")), (269, 612));
    }
}
