use ascii::AsciiString;
use core::panic;
use itertools::Itertools;
// use ndarray::prelude::*;
// use ndarray::{Array2, ShapeBuilder};
use crate::utils::*;
use std::str;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}

fn parse(raw: &[AsciiString]) -> Vec<Point> {
    let idxs = [(10, 16), (18, 24), (36, 38), (40, 42)];

    #[allow(clippy::string_from_utf8_as_bytes)]
    raw.iter()
        .flat_map(|r| {
            idxs.iter()
                .map(move |(s, t)| -> SomeResult<i32> {
                    Ok(str::from_utf8(&r.as_bytes()[*s..*t])?
                        .trim_start()
                        .parse::<i32>()?)
                })
                .flatten() // Unwrap result.
        })
        .tuple_windows()
        .step_by(4)
        .map(|(x, y, vx, vy)| Point { x, y, vx, vy })
        .collect_vec()
}

/// https://github.com/Voltara/advent2018-fast/blob/master/src/day10.cpp
fn ocr(x: &u64) -> char {
    match *x {
        0x861861fe186148c => 'A',
        0x7e186185f86185f => 'B',
        0xfc104105f04107f => 'E',
        0x04104105f04107f => 'F',
        0xbb1861e4104185e => 'G',
        0x86186187f861861 => 'H',
        0x8512450c3149461 => 'K',
        0xfc1041041041041 => 'L',
        _ => panic!("Unknown character."),
    }
}

pub fn combi(raw: &[AsciiString]) -> Option<(String, u32)> {
    let points = parse(raw);

    let (min, max) = points
        .iter()
        .minmax_by(|&s, &oth| (s.vy, -s.y).cmp(&(oth.vy, -oth.y)))
        .into_option()?;

    let part2 = (max.y - min.y) / (min.vy - max.vy);

    let moved = points
        .iter()
        .map(|p| (p.x + part2 * p.vx, p.y + part2 * p.vy))
        .collect_vec();

    let x_bound = moved.iter().min_by(|s, o| s.0.cmp(&o.0))?.0;
    let y_bound = moved.iter().min_by(|s, o| s.1.cmp(&o.1))?.1;

    let mut texts = [0u64; 8];
    moved.iter().for_each(|&p| {
        let x = p.0 - x_bound;
        let y = p.1 - y_bound;
        texts[(x / 8 % 8) as usize] |= 1 << (6 * y + x % 8);
    });

    let part1 = texts.iter().map(ocr).collect::<String>();
    Some((part1, part2 as u32))
}

// fn parse_ndarray(raw: &[AsciiString]) -> Vec<i32> {
//     let idxs = [(10, 16), (18, 24), (36, 38), (40, 42)];

//     #[allow(clippy::string_from_utf8_as_bytes)]
//     raw.iter()
//         .flat_map(|r| {
//             idxs.iter().map(move |(s, t)| {
//                 str::from_utf8(&r.as_bytes()[*s..*t])
//                     .unwrap()
//                     .trim_start()
//                     .parse::<i32>()
//                     .unwrap()
//             })
//         })
//         .collect_vec()
// }

// pub fn ndarray_trial(raw: &[AsciiString]) {
//     let p2 = parse_ndarray(raw);
//     let mut arr = Array2::from_shape_vec((p2.len() / 4, 4), p2).unwrap();

//     let (min, max) = arr
//         .outer_iter()
//         .minmax_by(|&s, &oth| (s[3], -s[1]).cmp(&(oth[3], -oth[1])))
//         .into_option()
//         .unwrap();

//     let part2 = (max[1] - min[1]) / (min[3] - max[3]);
// Cannot find a simple way for broadcasting.
// Too painful.
// }

mod tests {
    use super::*;

    #[test]
    fn test_combi() {
        assert_eq!(
            combi(&read_ascii("day10.txt")),
            Some(("FBHKLEAG".to_string(), 10009))
        );
    }
}
