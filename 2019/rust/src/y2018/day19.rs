use std::{iter::once, ops::Add};

use itertools::Itertools;
use num::{integer::Roots, Integer};

use crate::utils::printt;

type Parsed = (usize, Vec<[u8; 4]>);

pub fn parse(raw: &str) -> Parsed {
    let bind = (raw.chars().nth(4).unwrap() as u8 - b'0') as usize;

    let ins = raw
        .split_ascii_whitespace()
        .skip(2)
        .chunks(4)
        .into_iter()
        .map(|mut it| {
            let mut arr = [0u8; 4];
            arr[0] = match it.next().unwrap() {
                "addr" => 0,
                "addi" => 1,
                "mulr" => 2,
                "muli" => 3,
                "banr" => 4,
                "bani" => 5,
                "borr" => 6,
                "bori" => 7,
                "setr" => 8,
                "seti" => 9,
                "gtir" => 10,
                "gtri" => 11,
                "gtrr" => 12,
                "eqir" => 13,
                "equi" => 14,
                "eqrr" => 15,
                what => unreachable!("{:?}", what),
            };
            arr[1..].copy_from_slice(&it.map(|x| x.parse::<u8>().unwrap()).collect_vec());
            arr
        })
        .collect_vec();

    (bind, ins)
}

fn run_elf((bind, inss): &Parsed, r0: u32) -> u32 {
    let mut r = [0u32; 6];
    r[0] = r0;

    while r[*bind] != 1 {
        let curr = inss[r[*bind] as usize];

        let op = curr[0];
        let ia = curr[1] as u32;
        let ib = curr[2] as u32;
        let ic = curr[3] as usize;

        let ra = *r.get(ia as usize).unwrap_or(&u32::MAX);
        let rb = *r.get(ib as usize).unwrap_or(&u32::MAX);

        r[ic] = match op {
            0 => ra + rb,
            1 => ra + ib,
            2 => ra * rb,
            3 => ra * ib,
            8 => ra,
            9 => ia,
            what => unreachable!("{:?}", what),
        };
        r[*bind] += 1;
    }
    r[1]
}

unsafe fn run_elf_unchecked((bind, inss): &Parsed, r0: u32) -> u32 {
    let mut r = [0u32; 6];
    r[0] = r0;
    unsafe {
        while *r.get_unchecked(*bind) != 1 {
            let curr = *inss.get_unchecked(r[*bind] as usize);

            let op = curr[0];
            let ia = curr[1] as u32;
            let ib = curr[2] as u32;
            let ic = curr[3] as usize;

            let ra = *r.get_unchecked(ia as usize);
            let rb = *r.get_unchecked(ib as usize);

            r[ic] = match op {
                0 => ra + rb,
                1 => ra + ib,
                2 => ra * rb,
                3 => ra * ib,
                8 => ra,
                9 => ia,
                what => unreachable!("{:?}", what),
            };
            *r.get_unchecked_mut(*bind) += 1;
        }
    }
    r[1]
}

fn sum_factors(tg: u32) -> u32 {
    // https://math.stackexchange.com/a/22723
    fn add_factors(curr_sum: u32, mut tg: u32, p: u32) -> u32 {
        let mut m = 1;
        let mut f = 1;
        loop {
            tg /= p;
            f *= p;
            m += f;
            if tg % p != 0 {
                break;
            }
        }
        curr_sum * m
    }

    let mut out = 1;

    [2u32, 3, 5].iter().for_each(|cand| {
        if tg.is_multiple_of(cand) {
            out = add_factors(out, tg, *cand)
        }
    });

    let mut spacings = 0x62642424 as u32; // Spacing of numbers that not divisible by {2, 3, 5}.
    let mut cand = 7;
    while cand * cand <= tg {
        cand += spacings & (0xf - 1);
        if tg.is_multiple_of(&cand) {
            out = add_factors(out, tg, cand)
        }
        spacings = spacings.rotate_right(4); // Cycle, so spacing is (4, 2, 4, 2, 4, 6...)
    }

    out
}

/// Sum of divisors of r[1] aka sum {x: ℕ | x ≤ r[1] & r[1] | x}.
pub fn combi(parsed: &Parsed) -> (u32, u32) {
    (
        sum_factors(run_elf(parsed, 0)),
        sum_factors(run_elf(parsed, 1)),
    )
}

mod tests {
    use super::*;
    use crate::utils::*;

    #[test]
    fn test_combi() {
        assert_eq!(combi(&parse(&read(2018, "day19.txt"))), (1228, 15285504));
    }
}
