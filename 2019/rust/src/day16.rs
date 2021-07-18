use std::iter;

use ascii::AsciiString;
use itertools::Itertools;
use rayon::prelude::*;

use super::utils::*;

const ZERO: i32 = 48;

fn gen(x: &[u32]) -> u32 {
    x.iter().fold(0, |sum, y| 10 * sum + *y)
}

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
    let cs = iter::once(0_i32)
        .chain(raw[0].into_iter().map(|&x| x as i32 - ZERO))
        .collect_vec();

    let ans = fft(&cs, 100);
    gen(&ans[1..9].iter().map(|&x| x as u32).collect_vec())
}

/// Observation: all known offsets are typically toward the end of the sequence.
///     https://www.reddit.com/r/adventofcode/comments/ebf5cy/2019_day_16_part_2_understanding_how_to_come_up/fb4awi4/
///     When the offset n is more than ½ the length of the sequence, the FFT of that digit
///     is the partial sum from that digit to the end (mod 10). Since all numbers are positive,
///     we don't need to worry about the abs function.
///
/// Then, https://www.reddit.com/r/adventofcode/comments/ebqgdu/2019_day_16_part_2_lets_combinatorics/
/// By decomposing the input digit-by-digit, we can get the relation between the input and output as a combinatorics function.
/// Therefore, the final output is
/// > outputₙ mod 10 ≡ (inputₙ·₉₉C₀ + inputₙ₊₁·₁₀₀C₁ + inputₙ₊₂·₁₀₁C₂ + ⋯) mod 10
/// where n extends back to the diagonal.
///
///  Using the Chinese Remainder Theorem, x mod 10 ≡ (5x mod 2 - 4x mod 5) mod 10.
///
///  https://github.com/Voltara/advent2019-fast/blob/master/src/day16.cpp
///  Most of the speedup has to do with C(k+99, k) mod 10
///     C(k+99, k) % 5 = 1 ; k % 125 = 0
///                    = 4 ; k % 125 = 25
///                    = 0 ; otherwise
///     C(k+99, k) % 2 = 1 ; k % 128 = 0,4,8,12,16,20,24,28
///                    = 0 ; otherwise

fn add_parallel(x: &mut [u8], y: &[u8]) {
    x.iter_mut().zip(y.iter()).for_each(|(xx, yy)| *xx += *yy);
}

pub fn part2(raw: &[AsciiString]) -> usize {
    let cs = raw[0]
        .into_iter()
        .map(|&x| x as u8 - ZERO as u8)
        .collect_vec();
    let n = cs.len();
    let offset = gen(&cs[0..7].iter().map(|&x| x as u32).collect_vec()) as usize;

    let t = 10000 * n;

    let out = (offset..offset + 8)
        .into_par_iter()
        .map(|d| {
            let mut sum = 0u8;
            let mut idx = d % n;

            // Mod base 2
            for i in (0..t - d).step_by(128) {
                for j in (0..=28).step_by(4) {
                    if idx >= n {
                        idx -= n;
                    }
                    if i + j < t - d {
                        sum ^= cs[idx]; // We only care about the last bit.
                    }
                    idx += 4;
                }
                idx += 96;
            }
            sum = 5 * (sum % 2);

            let mut sum = sum as u32;
            let mut idx = d % n;

            for i in (0..t - d).step_by(125) {
                if idx >= n {
                    idx -= n;
                }
                sum += 6 * cs[idx] as u32;
                if i + 25 < t - d {
                    idx += 25;
                    if idx >= n {
                        idx -= n;
                    }
                    sum += 4 * cs[idx] as u32;
                }
                idx += 100;
            }
            sum % 10
        })
        .collect::<Vec<_>>();
    gen(&out[..]) as usize
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
