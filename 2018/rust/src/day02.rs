use ascii::{AsciiChar, AsciiString};
use bytecount::count;
use crc32fast::Hasher;
use itertools::Itertools;

pub fn part1(raw: &[AsciiString]) -> usize {
    let (mut two, mut three) = (0, 0);
    let mut arr;
    for line in raw.iter() {
        arr = [0_u8; 26]; // N_alphabets
        for c in line.as_bytes() {
            arr[(*c - AsciiChar::a as u8) as usize] += 1;
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

pub fn part2(raw: &[AsciiString]) -> Option<AsciiString> {
    let hashes = raw
        .iter()
        .enumerate()
        .flat_map(|(i, x)| {
            vec![
                run_hash(&x.as_bytes()[..13]) << 32 | i,
                run_hash(&x.as_bytes()[13..]) << 32 | i,
            ]
        })
        .sorted_unstable()
        .collect_vec();

    let mask: usize = (0..32).fold(0, |sum, i| sum | 1 << i);
    let hs: Vec<&AsciiString> = hashes // halfsame
        .iter()
        .tuple_windows::<(_, _)>()
        .filter(|(&x, &y)| x >> 32 == y >> 32)
        .flat_map(|(&x, &y)| vec![x & mask, y & mask])
        .unique()
        .map(|i| &raw[i])
        .collect_vec();

    for i in 0..hs.len() {
        for j in (i + 1)..hs.len() {
            let mut diffs = hs[i]
                .into_iter()
                .zip(hs[j].into_iter())
                .enumerate()
                .filter_map(|(k, (x, y))| if x != y { Some(k) } else { None });

            let ans = match diffs.next() {
                Some(x) if diffs.next().is_none() => x, // Only 1 diff.
                _ => continue,
            };

            let mut out = hs[i].to_owned();
            out.remove(ans);
            return Some(out);
        }
    }
    None
}

#[cfg(test)]

mod tests {
    use std::str::FromStr;

    use super::*;
    use crate::utils::*;

    #[test]
    fn test() {
        assert_eq!(part1(&read_ascii("day02.txt")), 6175);
    }

    #[test]
    fn test2() {
        assert_eq!(
            part2(&read_ascii("day02.txt")).unwrap(),
            AsciiString::from_str("asgwjcmzredihqoutcylvzinx").unwrap()
        );
    }
}
