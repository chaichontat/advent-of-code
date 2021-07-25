use std::ops::Mul;
use std::str::from_utf8;

use bytecount::count;
use crc32fast::Hasher;
use itertools::{izip, Itertools};
use packed_simd::u8x32;

type Parsed = Vec<u8>;

pub fn parse(raw: &str) -> Vec<Parsed> {
    raw.split('\n')
        .map(|line| line.chars().map(|c| c as u8).collect())
        .collect()
}

pub fn part1(parsed: &[Parsed]) -> usize {
    let (mut two, mut three) = (0, 0);
    let mut arr;
    for line in parsed {
        arr = [0_u8; 26]; // N_alphabets
        for c in line {
            arr[*c as usize - 'a' as usize] += 1;
        }
        two += if count(&arr, 2) > 0 { 1 } else { 0 };
        three += if count(&arr, 3) > 0 { 1 } else { 0 };
    }
    two * three
}

fn run_hash(x: &[u8]) -> usize {
    let mut hasher = Hasher::new();
    hasher.update(x);
    hasher.finalize() as usize
}

/// Find a unique pair of strings with only one difference between them.
///
/// Use lazy evaluation for comparison which terminates if there are more
/// than one diffs.
///
/// Also, since we are looking for a pair with one diff, if we split that
/// string in half, one side must be exactly the same. Therefore,
/// we can perform fast comparisons with the hardware-accelerated CRC32
/// checksum to eliminate strings without any half-sameness to other strings.

#[allow(dead_code)]
pub fn part2(parsed: &[Parsed]) -> Option<String> {
    let hashes = parsed
        .iter()
        .enumerate()
        .flat_map(|(i, x)| vec![run_hash(&x[..13]) << 32 | i, run_hash(&x[13..]) << 32 | i])
        .sorted_unstable()
        .collect_vec();

    let mask: usize = (0..32).fold(0, |sum, i| sum | 1 << i);
    let hs: Vec<&[u8]> = hashes // halfsame
        .iter()
        .tuple_windows::<(_, _)>()
        .filter(|(&x, &y)| x >> 32 == y >> 32)
        .flat_map(|(&x, &y)| vec![x & mask, y & mask])
        .unique()
        .map(|i| &parsed[i][..])
        .collect_vec();

    for i in 0..hs.len() {
        for j in (i + 1)..hs.len() {
            let mut diffs = hs[i]
                .iter()
                .zip(hs[j])
                .enumerate()
                .filter_map(|(k, (x, y))| if x != y { Some(k) } else { None });

            let ans = match diffs.next() {
                Some(x) if diffs.next().is_none() => x, // Only 1 diff.
                _ => continue,
            };

            let mut out = hs[i].to_owned();
            out.remove(ans);
            return Some(std::str::from_utf8(&out).unwrap().to_string());
        }
    }
    None
}

/// https://www.reddit.com/r/adventofcode/comments/a2hhw1/manually_vectorizing_d2p2/eayfbif/?context=3
pub fn part2_simd(parsed: &[Parsed]) -> Option<String> {
    #[repr(align(32))]
    #[derive(Copy, Clone)]
    struct Line([u8; 32]);

    let mut storage = [u8x32::splat(0); 250];
    let mut buf = Line([0; 32]);
    for (storage, line) in storage.iter_mut().zip(parsed) {
        buf.0[..line.len()].copy_from_slice(line);
        unsafe {
            *storage = u8x32::from_slice_aligned_unchecked(&buf.0);
        }
    }

    for (i, &a) in storage.iter().enumerate() {
        for &b in &storage[i + 1..] {
            let diff = (!a.eq(b)).bitmask();
            let one_bit = diff & diff.wrapping_neg(); // BLSI. Return only right-most bit.
            if diff == one_bit {
                let a: [u8; 32] = a.into();
                let mut s = from_utf8(&a[..26]).unwrap().to_string();
                s.remove(one_bit.trailing_zeros() as usize); // Position of difference.
                return Some(s);
            }
        }
    }

    None
}

#[cfg(test)]

mod tests {
    use super::*;
    use crate::utils::*;

    #[test]
    fn test() {
        assert_eq!(part1(&parse(&read(2018, "day02.txt"))), 6175);
    }

    #[test]
    fn test2() {
        assert_eq!(
            part2_simd(&parse(&read(2018, "day02.txt"))).unwrap(),
            "asgwjcmzredihqoutcylvzinx".to_string()
        );
    }
}
