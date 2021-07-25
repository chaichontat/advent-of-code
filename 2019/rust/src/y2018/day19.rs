use std::iter::once;

use itertools::Itertools;
use num::integer::Roots;

use crate::utils::printt;

type Parsed = (usize, Vec<[u8; 4]>);

fn factorize(n: u32) {
    let mut sum = 1;
}

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

        let ra = || r[ia as usize];
        let rb = || r[ib as usize];

        r[ic] = match op {
            0 => ra() + rb(),
            1 => ra() + ib,
            2 => ra() * rb(),
            3 => ra() * ib,
            8 => ra(),
            9 => ia,
            what => unreachable!("{:?}", what),
        };
        r[*bind] += 1;
    }
    r[1]
}

fn sum_factors(n: u32) -> u32 {
    // let primes = (1..n.sqrt()).filter(|&i| n % i == 0).collect_vec();
    // let factors = vec![];
    // for i in 0..primes.len() {
    //     for j in i+1..primes.len() {
    //         factors.push(primes[i] * primes[j]);
    //     }
    // }
    (1..n + 1).filter(|&i| n % i == 0).sum()
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
