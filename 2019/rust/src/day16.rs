use std::iter;

use ascii::AsciiString;
use itertools::Itertools;

use super::utils::*;

const ZERO: i32 = 48;

fn fft(start: &[i32], step: u16) -> Vec<i32> {
    let mut cs = start.to_owned();
    let n = cs.len() - 1;
    for _ in 0..step {
        cs.iter_mut().fold(0, |acc, i| {
            *i += acc; // Cumulative sum.
            *i
        });

        cs = (0..n + 1)
            .map(|i| {
                if i == 0 {
                    return 0;
                }
                let mut sum = 0;
                let mut start_1 = i - 1; // Zero-offset first i digits.
                while start_1 < n {
                    let end_1 = (start_1 + i).min(n); // End of 1s of length i+1.
                    sum = (cs[end_1] - cs[start_1]) - sum;
                    start_1 = end_1 + i;
                }
                sum.abs() % 10
            })
            .collect_vec();
    }
    cs
}

pub fn part1(raw: &[AsciiString]) -> u32 {
    // Need to prepend 0 for a much simpler implementation of the add/subtract alternation.
    let mut cs = iter::once(0_i32)
        .chain(raw[0].into_iter().map(|&x| x as i32 - ZERO))
        .collect_vec();

    let ans = fft(&cs, 100);
    ans[1..9].iter().fold(0, |sum, &i| 10 * sum + i as u32)
}

fn comb99mod10(k: u32, b: u32) -> u32 {
    if b == 5 {
        match k {
            l if l % 125 == 0 => 1,
            l if l % 125 == 25 => 25,
            _ => 0,
        }
    } else if b == 2 {
        match k {
            l if (0..=28).step_by(4).contains(&(l % 128)) => 1,
            _ => 0,
        }
    } else {
        unreachable!()
    }
}

fn find_hidden(s: &[i32]) -> Vec<i32> {
    let mut s = s.to_owned();
    for _ in 0..100 {
        let mut cumsum = s.iter().sum::<i32>();
        for x in s.iter_mut() {
            let val = *x;
            *x = cumsum % 10;
            cumsum -= val;
        }
    }
    s
}

// https://www.reddit.com/r/adventofcode/comments/ebqgdu/2019_day_16_part_2_lets_combinatorics/
pub fn part2(raw: &[AsciiString]) -> usize {
    let cs = std::iter::repeat(raw[0].into_iter().map(|&x| x as i32 - ZERO))
        .take(10000)
        .flatten()
        .collect_vec();
    let offset = cs[0..7].iter().fold(0, |sum, &i| 10 * sum + i as usize);

    let ans = find_hidden(&cs[offset..]);
    ans[0..8].iter().fold(0, |sum, &i| 10 * sum + i as usize)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(part1(&read_ascii("day16.txt")), 27229269);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(&read_ascii("day16.txt")), 26857164);
    }
}
