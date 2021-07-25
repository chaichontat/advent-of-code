use itertools::Itertools;
use regex::Regex;

use crate::utils::printt;

type Parsed = u8;
type Keys = [i16; 16];

pub fn parse(raw: &str) -> (Vec<Parsed>, Vec<Parsed>) {
    let re = Regex::new(r"\d+").unwrap();
    let blocks = raw.split("\n\n").collect_vec();
    let v = blocks[..blocks.len() - 1]
        .iter()
        .flat_map(|&x| re.find_iter(x).map(|m| m.as_str().parse::<u8>().unwrap()))
        .collect_vec();

    let codes = re
        .find_iter(blocks[blocks.len() - 1])
        .map(|m| m.as_str().parse::<u8>().unwrap())
        .collect_vec();

    (v, codes)
}

// Bipartite perfect matching problem.
pub fn match_keys(mut keys: Keys) -> [u8; 16] {
    // Yet to be mapped.
    let mut dom = -1i16; // Domain: current opcode.
    let mut rng = -1i16; // Range : real opcode.

    while dom != 0 {
        let mut curr_dom = dom;
        // Need for initial cycling through elements without a unique mapping.
        // Remove found codes and cycle until found current opcode with a unique mapping.
        while curr_dom != 0 {
            let r = keys.get_mut(curr_dom.trailing_zeros() as usize).unwrap();
            *r &= rng;
            if r.count_ones() == 1 {
                rng ^= *r; // Remove real op from candidates.
                dom ^= curr_dom & curr_dom.wrapping_neg(); // BLSI Remove curr op.
            }
            curr_dom &= curr_dom.wrapping_sub(1); // BLSR Set rightmost bit to 0.
        }
    }

    let mut out = [0; 16];
    for (o, k) in out.iter_mut().zip(keys) {
        *o = k.trailing_zeros() as u8;
    }
    out
}

// Considered using separate functions for each opcode.
// Would be too messy.
#[rustfmt::skip]
#[allow(clippy::identity_op)]
pub unsafe fn combi_unchecked((ins, codes): &(Vec<Parsed>, Vec<Parsed>)) -> (usize, u16) {
    let mut part1 = 0;
    let mut keys = [-1i16; 16];
    ins.chunks_exact(12).for_each(|block| {
        let bef = &block[..4];
        let ins = &block[4..8];
        let aft = &block[8..];

        unsafe {
            let ia = ins[1];
            let ib = ins[2];
            let ra = *bef.get_unchecked(ia as usize);
            let rb = *bef.get_unchecked(ib as usize);
            let ans = *aft.get_unchecked(ins[3] as usize);
    
            let mut m = 0i16;
            m |= ((ans ==  ra + rb)         as i16) << 0;
            m |= ((ans ==  ra + ib)         as i16) << 1;
            m |= ((ans ==  ra * rb)         as i16) << 2;
            m |= ((ans ==  ra * ib)         as i16) << 3;
            m |= ((ans ==  ra & rb)         as i16) << 4;
            m |= ((ans ==  ra & ib)         as i16) << 5;
            m |= ((ans ==  ra | rb)         as i16) << 6;
            m |= ((ans ==  ra | ib)         as i16) << 7;
            m |= ((ans ==  ra     )         as i16) << 8;
            m |= ((ans ==  ia     )         as i16) << 9;
            m |= ((ans == (ia > rb)  as u8) as i16) << 10;
            m |= ((ans == (ra > ib)  as u8) as i16) << 11;
            m |= ((ans == (ra > rb)  as u8) as i16) << 12;
            m |= ((ans == (ia == rb) as u8) as i16) << 13;
            m |= ((ans == (ra == ib) as u8) as i16) << 14;
            m |= ((ans == (ra == rb) as u8) as i16) << 15;
    
            let op = ins[0] as usize;
            *keys.get_unchecked_mut(op) &= m;
            if m.count_ones() >= 3 {
                part1 += 1;
            }
        }
    });

    let keys = match_keys(keys);
    let mut reg = [0u16;4];
    codes.chunks_exact(4).for_each(|ins| {
        let ia = ins[1] as u16;
        let ib = ins[2] as u16;
        let ic = ins[3] as u16;

        unsafe {
            let op = keys.get_unchecked(ins[0] as usize);
            let ra = *reg.get_unchecked(ia as usize);
            let rb = *reg.get_unchecked(ib as usize);
            let rc = reg.get_unchecked_mut(ic as usize);
            match op {
                 0 => *rc = ra + rb,
                 1 => *rc = ra + ib,
                 2 => *rc = ra * rb,
                 3 => *rc = ra * ib,
                 4 => *rc = ra & rb,
                 5 => *rc = ra & ib,
                 6 => *rc = ra | rb,
                 7 => *rc = ra | ib,
                 8 => *rc = ra,
                 9 => *rc = ia,
                10 => *rc = (ia > rb) as u16,
                11 => *rc = (ra > ib) as u16,
                12 => *rc = (ra > rb) as u16,
                13 => *rc = (ia == rb) as u16,
                14 => *rc = (ra == ib) as u16,
                15 => *rc = (ra == rb) as u16,
                _ => unreachable!()
            }
        }
    });

    (part1, reg[0])
}

#[rustfmt::skip]
#[allow(clippy::identity_op)]
pub fn combi((ins, codes): &(Vec<Parsed>, Vec<Parsed>)) -> (usize, u16) {
    let mut part1 = 0;
    let mut keys = [-1i16; 16];
    ins.chunks_exact(12).for_each(|block| {
        let bef = &block[..4];
        let ins = &block[4..8];
        let aft = &block[8..];

        let ia = ins[1];
        let ib = ins[2];
        let ra = bef[ia as usize];
        let rb = bef[ib as usize];
        let ans = aft[ins[3] as usize];

        let mut m = 0i16;
        m |= ((ans ==  ra + rb)         as i16) << 0;
        m |= ((ans ==  ra + ib)         as i16) << 1;
        m |= ((ans ==  ra * rb)         as i16) << 2;
        m |= ((ans ==  ra * ib)         as i16) << 3;
        m |= ((ans ==  ra & rb)         as i16) << 4;
        m |= ((ans ==  ra & ib)         as i16) << 5;
        m |= ((ans ==  ra | rb)         as i16) << 6;
        m |= ((ans ==  ra | ib)         as i16) << 7;
        m |= ((ans ==  ra     )         as i16) << 8;
        m |= ((ans ==  ia     )         as i16) << 9;
        m |= ((ans == (ia > rb)  as u8) as i16) << 10;
        m |= ((ans == (ra > ib)  as u8) as i16) << 11;
        m |= ((ans == (ra > rb)  as u8) as i16) << 12;
        m |= ((ans == (ia == rb) as u8) as i16) << 13;
        m |= ((ans == (ra == ib) as u8) as i16) << 14;
        m |= ((ans == (ra == rb) as u8) as i16) << 15;

        let op = ins[0] as usize;
        *keys.get_mut(op).unwrap() &= m;
        if m.count_ones() >= 3 {
            part1 += 1;
        }
        
    });

    let keys = match_keys(keys);
    let mut reg = [0u16;4];
    codes.chunks_exact(4).for_each(|ins| {
        let ia = ins[1] as u16;
        let ib = ins[2] as u16;
        let ic = ins[3] as u16;
        let op = keys[ins[0] as usize];
        let ra = reg[ia as usize];
        let rb = reg[ib as usize];
        let rc = reg.get_mut(ic as usize).unwrap();
        match op {
            0  => *rc = ra + rb,
            1  => *rc = ra + ib,
            2  => *rc = ra * rb,
            3  => *rc = ra * ib,
            4  => *rc = ra & rb,
            5  => *rc = ra & ib,
            6  => *rc = ra | rb,
            7  => *rc = ra | ib,
            8  => *rc = ra,
            9  => *rc = ia,
            10 => *rc = (ia > rb) as u16,
            11 => *rc = (ra > ib) as u16,
            12 => *rc = (ra > rb) as u16,
            13 => *rc = (ia == rb) as u16,
            14 => *rc = (ra == ib) as u16,
            15 => *rc = (ra == rb) as u16,
            _ => unreachable!()
        }
    });

    (part1, reg[0])
}

mod tests {
    use super::*;
    use crate::utils::*;

    #[test]
    fn test_combi() {
        assert_eq!(combi(&parse(&read(2018, "day16.txt"))), (570, 503));
    }

    #[test]
    fn test_combi_unchecked() {
        unsafe {
            assert_eq!(
                combi_unchecked(&parse(&read(2018, "day16.txt"))),
                (570, 503)
            );
        }
    }
}
