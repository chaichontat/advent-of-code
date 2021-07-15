use std::cmp::Ordering;

use ascii::{AsciiChar, AsciiString};
use bitvec::prelude::*;

#[allow(bindings_with_variant_name)]
fn parse(raw: &[AsciiString]) -> (BitVec, u32) {
    let start = raw[0][15..]
        .into_iter()
        .map(|&c| match c {
            AsciiChar::Hash => true,
            AsciiChar::Dot => false,
            _ => unreachable!(),
        })
        .collect::<BitVec>();

    let mut rules = 0u32; // 2⁵ potential rules.
    for rule in raw[2..].iter() {
        if rule[9] == AsciiChar::Dot {
            continue;
        }
        let mut r = 0u8;
        for c in rule[..5].into_iter() {
            r <<= 1;
            match *c {
                AsciiChar::Hash => r |= 1,
                AsciiChar::Dot => (),
                _ => unreachable!(),
            }
        }
        rules |= 1 << r; // Encode active rules by bit position.
    }
    (start, rules)
}

fn step(start: &BitVec, rules: u32, mut base: i16) -> (BitVec, i16) {
    let mut new = BitVec::new();
    let mut i = 0;

    while !start[i] {
        i += 1; // Skip empty bits.
        base += 1;
    }
    base -= 2; // Code structure. 2 leading zeros.

    let mut win = 0u8;
    let mask = 0b011111; // Keep only 5 most recent bits.
    while i < start.len() {
        let b = start[i] as u8;
        win = (win << 1 | b) & mask; // Sliding window; encode into rule.
        new.push((rules >> win) & 1 == 1); // Check if rule exists.
        i += 1;
    }

    while win > 0 {
        win = (win << 1) & mask; // Continue until window is empty.
        new.push((rules >> win) & 1 == 1);
    }

    while new.trailing_zeros() > 0 {
        new.pop();
    }

    (new, base)
}

fn score(state: &BitVec, base: i16) -> u64 {
    state.iter_ones().map(|x| x as u64 + base as u64).sum()
}

#[allow(unused_assignments)]
pub fn combi(raw: &[AsciiString]) -> (u64, u64) {
    let (mut prev, rules) = parse(raw);
    let mut base = 0;
    let mut i = 0;
    let (mut part1, mut part2) = (0, 0);

    loop {
        let (now, new_base) = step(&prev, rules, base);
        i += 1;

        match i.cmp(&20) {
            Ordering::Less => (),
            Ordering::Equal => part1 = score(&now, new_base),
            Ordering::Greater => {
                if prev == now {
                    part2 = score(&now, new_base);
                    part2 += (part2 - score(&prev, base)) * (50_000_000_000 - i as u64);
                    break;
                }
            }
        }

        base = new_base;
        prev = now;
    }
    (part1, part2)
}

mod tests {
    use super::*;
    use crate::utils::*;

    #[test]
    fn test_combi() {
        assert_eq!(combi(&read_ascii("day12.txt")), (2444, 750000000697));
    }
}
