// TODO: Finish part 2. Implement wheel factorization.

use hashbrown::HashSet;
use itertools::Itertools;

type Parsed = (usize, Vec<[u64; 4]>);

use crate::utils::printt;

pub fn parse(raw: &str) -> Parsed {
    let bind = (raw.chars().nth(4).unwrap() as u8 - b'0') as usize;

    let ins = raw
        .split_ascii_whitespace()
        .skip(2)
        .chunks(4)
        .into_iter()
        .map(|mut it| {
            let mut arr = [0u64; 4];
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
                "eqri" => 14,
                "eqrr" => 15,
                what => unreachable!("{:?}", what),
            };
            arr[1..].copy_from_slice(&it.map(|x| x.parse::<u64>().unwrap()).collect_vec());
            arr
        })
        .collect_vec();

    (bind, ins)
}

fn run_elf((bind, inss): &Parsed) -> (u64, u64) {
    let mut r = [0u64; 6];
    let mut set = HashSet::new();
    let mut last = -1i64;
    let mut part1 = 0;
    let mut part2 = 0;

    loop {
        let curr = inss[r[*bind] as usize];

        let op = curr[0];
        let ia = curr[1] as u64;
        let ib = curr[2] as u64;
        let ic = curr[3] as usize;

        let ra = *r.get(ia as usize).unwrap_or(&u64::MAX);
        let rb = *r.get(ib as usize).unwrap_or(&u64::MAX);

        r[ic] = match op {
            0 => ra + rb,
            1 => ra + ib,
            2 => ra * rb,
            3 => ra * ib,
            4 => ra & rb,
            5 => ra & ib,
            6 => ra | rb,
            7 => ra | ib,
            8 => ra,
            9 => ia,
            10 => (ia > rb) as u64,
            11 => (ra > ib) as u64,
            12 => (ra > rb) as u64,
            13 => (ia == rb) as u64,
            14 => (ra == ib) as u64,
            15 => (ra == rb) as u64,
            what => unreachable!("{:?}", what),
        };

        if r[*bind] == 28 {
            if last == -1 {
                part1 = last as u64;
            }
            if !set.insert(r[1]) {
                part2 = last as u64;
                break;
            }
            last = r[1] as i64;
        }

        r[*bind] += 1;
    }
    (part1, part2)
}

/// Sum of divisors of r[1] aka sum {x: ℕ | x ≤ r[1] & r[1] | x}.
pub fn combi(parsed: &Parsed) -> (u64, u64) {
    run_elf(parsed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::*;

    // #[test]
    fn test_combi() {
        assert_eq!(combi(&parse(&read(2018, "day21.txt"))), (2525738, 11316540));
    }
}
