use ndarray::prelude::*;
use ndarray::Array;

type Parsed = u32;
pub fn parse(raw: &str) -> Vec<Parsed> {
    raw.chars().map(|x| x.to_digit(10).unwrap()).collect()
}

fn process(parsed: &[u32]) -> Array3<u32> {
    Array::from_shape_vec(((parsed.len() / (25 * 6)), 6, 25), parsed.to_vec()).unwrap()
}

pub fn part1(parsed: &[u32]) -> usize {
    let arr = process(parsed);
    let clipped = arr
        .outer_iter()
        .map(|x| x.iter().filter(|y| y < &&1).count())
        .collect::<Vec<_>>();

    let min_val = clipped.iter().min().unwrap();
    let min_idx = clipped.iter().position(|x| x == min_val).unwrap();

    let counts = (
        arr.index_axis(Axis(0), min_idx)
            .iter()
            .filter(|&&y| y == 1)
            .count(),
        arr.index_axis(Axis(0), min_idx)
            .iter()
            .filter(|&&y| y == 2)
            .count(),
    );
    counts.0 * counts.1
}

pub fn part2(parsed: &[u32]) -> usize {
    let arr = process(parsed);
    for i in 0..6 {
        for j in 0..25 {
            match arr.slice(s![.., i, j]).iter().find(|&&x| x == 0 || x == 1) {
                Some(0) => (), // print!(" "),
                Some(1) => (), // print!("#"),
                _ => unreachable!(),
            }
        }
        // println!();
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::*;
    #[test]
    fn test1() {
        assert_eq!(part1(&parse(&read(2019, "day08.txt"))), 1950);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(&parse(&read(2019, "day08.txt"))), 0);
    }
}
