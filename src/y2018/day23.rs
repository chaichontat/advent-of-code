use std::collections::BinaryHeap;

use hashbrown::HashSet;
use itertools::Itertools;
use ndarray::prelude::*;
use ndarray::Array2;
use regex::Regex;
use rstar::{Point, PointDistance, RTree, RTreeObject, AABB};

use crate::utils::*;

type Parsed = i32;

pub fn parse(raw: &str) -> Vec<Parsed> {
    let re = Regex::new(r"-?\d+").unwrap();
    re.find_iter(raw).map(|x| x.as_str().parse().unwrap()).collect()
}

pub fn part1(parsed: &[Parsed]) -> usize {
    let parsed = parsed.to_owned();
    let arr = Array2::from_shape_vec((parsed.len() / 4, 4), parsed).unwrap();
    let (max_r, max_idx) = arr
        .column(3)
        .indexed_iter()
        .map(|(idx, &val)| (val, idx))
        .max()
        .unwrap();

    let max_pos = arr.row(max_idx).to_owned();
    let max_pos = max_pos.insert_axis(Axis(0));
    let mut arr = arr - max_pos;
    arr.mapv_inplace(i32::abs);
    arr.outer_iter()
        .map(|x| x.slice(s![..3]).sum())
        .filter(|&x| x <= max_r)
        .count()
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Bot {
    p: [i32; 3],
    r: i32,
}

impl RTreeObject for Bot {
    type Envelope = AABB<[i32; 3]>;

    fn envelope(&self) -> Self::Envelope {
        let corner_1 = [self.p[0] - self.r, self.p[1] - self.r, self.p[2] - self.r];
        let corner_2 = [self.p[0] + self.r, self.p[1] + self.r, self.p[2] + self.r];
        AABB::from_corners(corner_1, corner_2)
    }
}

impl PointDistance for Bot {
    fn distance_2(&self, point: &[i32; 3]) -> i32 {
        let d_x = self.p[0] - point[0];
        let d_y = self.p[1] - point[1];
        let d_z = self.p[2] - point[2];
        let d = d_x.abs() + d_y.abs() + d_z.abs();
        i32::max(0, d - self.r)
    }
}

fn max_point(s: &[i32; 3], o: &[i32; 3]) -> [i32; 3] {
    [s[0].max(o[0]), s[1].max(o[1]), s[2].max(o[2])]
}

fn min_point(s: &[i32; 3], o: &[i32; 3]) -> [i32; 3] {
    [s[0].min(o[0]), s[1].min(o[1]), s[2].min(o[2])]
}

fn common_box(s: AABB<[i32; 3]>, other: AABB<[i32; 3]>) -> AABB<[i32; 3]> {
    AABB::from_corners(
        max_point(&s.lower(), &other.lower()),
        min_point(&s.upper(), &other.upper()),
    )
}
pub fn part2(parsed: &[i32]) -> usize {
    let parsed = parsed.to_owned();

    let points = parsed
        .chunks_exact(4)
        .map(|x| Bot { p: [x[0], x[1], x[2]], r: x[3] })
        .collect_vec();

    let rt = RTree::bulk_load(points);

    // for b in &rt {
    //     let r = b.r;
    //     let p = b.p;
    //     let t = (0..3)
    //         .map(|i| p[i] - r..p[i] + r)
    //         .multi_cartesian_product()
    //         .filter(|v| b.distance_2(&[v[0], v[1], v[2]]) <= r)
    //         .map(|v| (rt.locate_within_distance([v[0], v[1], v[2]], 0).count(), v))
    //         .collect_vec();
    //     printt(&t);
    // }
    let mut curr_bound = BinaryHeap::new();
    let mut already = HashSet::new();
    for b in &rt {
        let now = rt
            .locate_in_envelope_intersecting(&b.envelope())
            .map(|x| common_box(b.envelope(), x.envelope()))
            .collect_vec();
        let len = now.len();
        for c in now {
            if already.insert((c.lower(), c.upper())) {
                curr_bound.push((len, c));
            }
        }
    }

    let mut max_intersect = 0;
    for _ in 0..3 {
        let (curr_intersect, e) = curr_bound.pop().unwrap();
        if curr_intersect < max_intersect {
            continue;
        }
        let now = rt
            .locate_in_envelope_intersecting(&e)
            .map(|x| common_box(e, x.envelope()))
            .collect_vec();
        let len = now.len();
        for c in now {
            if already.insert((c.lower(), c.upper())) {
                curr_bound.push((len, c));
            }
        }
    }

    printt(&curr_bound);

    // for p in rt.iter() {

    // }
    // printt(&rt.locate_within_distance([12, 12, 12], 0).count());
    // println!("{:#?}", x);
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(&read(2018, "day23.txt"))), 584);
    }

    // #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(&read(2018, "day23.txt"))), 0);
    }
}
