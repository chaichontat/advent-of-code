use std::cmp::Ordering;

use ascii::{AsciiChar, AsciiString};
use bitvec::prelude::*;

struct BitVecBase {
    state: BitVec,
    base: i16,
}

#[allow(bindings_with_variant_name)]
fn parse(raw: &[AsciiString]) -> (BitVecBase, u32) {
    let start = raw[0][15..]
        .into_iter()
        .map(|&c| match c {
            AsciiChar::Hash => true,
            AsciiChar::Dot => false,
            _ => unreachable!(),
        })
        .collect::<BitVec>();

    // let rules = raw[2..] Significantly slower.
    //     .iter()
    //     .filter(|&rule| rule[9] != AsciiChar::Dot)
    //     .fold(0u32, |rules, rule| {
    //         let r = rule[..5].into_iter().fold(0u8, |this, c| match *c {
    //             AsciiChar::Hash => this << 1 | 1,
    //             AsciiChar::Dot => this << 1,
    //             _ => unreachable!(),
    //         });
    //         rules | 1 << r
    //     });

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
    (
        BitVecBase {
            state: start,
            base: 0,
        },
        rules,
    )
}

fn step(start: &BitVecBase, rules: u32) -> BitVecBase {
    let mut new = BitVec::new();
    let mut i = 0;
    let mut base = start.base;
    while !start.state[i] {
        i += 1; // Skip empty bits.
        base += 1;
    }
    base -= 2; // Code structure. 2 leading zeros.

    let mut win = 0u8;
    let mask = 0b011111; // Keep only 5 most recent bits.
    while i < start.state.len() {
        let b = start.state[i] as u8;
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

    BitVecBase { state: new, base }
}

fn score(b: &BitVecBase) -> u64 {
    b.state.iter_ones().map(|x| x as u64 + b.base as u64).sum()
}

pub fn combi(raw: &[AsciiString]) -> (Option<u64>, Option<u64>) {
    let (input, rules) = parse(raw);
    let (mut part1, mut part2) = (None, None);

    let mut prev = input;
    for i in 1.. {
        let now = step(&prev, rules);
        match i.cmp(&20) {
            Ordering::Less => (),
            Ordering::Equal => part1 = Some(score(&now)),
            Ordering::Greater => {
                if prev.state == now.state {
                    let p2 = score(&now);
                    part2 = Some(p2 + (p2 - score(&prev)) * (50_000_000_000 - i as u64));
                    break;
                }
            }
        }
        prev = now;
    }

    (part1, part2)
}

mod tests {
    use super::*;
    use crate::utils::*;

    #[test]
    fn test_combi() {
        assert_eq!(
            combi(&read_ascii_sep("day12.txt")),
            (Some(2444), Some(750000000697))
        );
    }
}
