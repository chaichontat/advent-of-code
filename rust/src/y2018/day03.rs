use std::cmp::Reverse;

use itertools::Itertools;
use num::Integer;
use regex::Regex;

const DIM: usize = 1024;
const BASE: usize = 32;
type Base = u32;
const ARR: usize = DIM / BASE;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
enum Overlap {
    Yes,
    No,
}

#[derive(Debug, Clone, Copy)]
pub struct Claim {
    y:      u16,
    h:      u16,
    id:     u16,
    m_slot: u16,
    masks:  [Base; 2], // Claims never exceed 29.
    ol:     Option<Overlap>,
}

impl Claim {
    #[allow(clippy::many_single_char_names)]
    fn new(a: [u16; 5]) -> Claim {
        let (id, x, y, w, h) = (a[0], a[1], a[2], a[3], a[4]);
        let (m_slot, shift) = x.div_rem(&(BASE as u16));

        let m = (1 << w) - 1;
        let masks = [m << shift, if shift > 0 { m >> (BASE as u16 - shift) } else { 0 }];

        Claim { y, h, id, m_slot, masks, ol: None }
    }
}

pub fn parse(raw: &str) -> Vec<Claim> {
    let re = Regex::new(r"\d+").unwrap();
    raw.split('\n')
        .map(|line| {
            let mut temp = [0u16; 5];
            let matches = re.find_iter(line);
            temp.iter_mut()
                .zip(matches)
                .for_each(|(o, x)| *o = x.as_str().parse::<u16>().unwrap());
            Claim::new(temp)
        })
        .collect_vec()
}

pub fn combi(cs: &[Claim]) -> (u32, u32) {
    let mut cs = cs.to_owned();
    // Sort by row in descending order to enable efficient vector pop.
    cs.sort_unstable_by_key(|c| Reverse(c.y));

    let mut part1 = 0;
    let mut part2 = None;

    assert!(cs.iter().all(|c| c.m_slot <= ARR as u16)); // Safety check.
    while !cs.is_empty() {
        // Iteration corresponds to row.
        let curr_row = cs.last().unwrap().y;
        let overlap = unsafe { get_overlap(&cs, curr_row) };
        part1 += overlap.iter().map(|x| x.count_ones()).sum::<u32>();

        for i in (0..cs.len()).rev() {
            if cs[i].y != curr_row {
                break;
            }

            if part2.is_none() {
                // Check if current claim overlaps.
                let idx = cs[i].m_slot as usize;
                if overlap[idx] & cs[i].masks[0] != 0 || overlap[idx + 1] & cs[i].masks[1] != 0 {
                    cs[i].ol = Some(Overlap::Yes);
                }
            }

            cs[i].y += 1;
            cs[i].h -= 1;

            if cs[i].h == 0 {
                if part2.is_none() && cs[i].ol.is_none() {
                    part2 = Some(cs[i].id);
                }
                cs.swap_remove(i);
            }
        }
    }
    (part1, part2.unwrap() as u32)
}

/// # Safety
/// Expects all `Claim::idx` to be within bounds of the temp array.
///
/// Expects `cs` to be sorted by `y` in descending order.
/// Create two storage bitvecs, one lagging by a claim and one current.
/// If there are more than one claims,
unsafe fn get_overlap(cs: &[Claim], y: u16) -> [u32; ARR + 1] {
    let mut temp = [0; ARR + 1];
    let mut overlap = temp;

    for c in cs.iter().rev() {
        // Loop once if there's no overlap.
        if c.y != y {
            break;
        }

        let idx = c.m_slot as usize;
        #[rustfmt::skip]
        unsafe {
            *overlap.get_unchecked_mut(idx)     |= temp.get_unchecked(idx)     & c.masks[0];
            *overlap.get_unchecked_mut(idx + 1) |= temp.get_unchecked(idx + 1) & c.masks[1];
            *temp   .get_unchecked_mut(idx)     |= c.masks[0];
            *temp   .get_unchecked_mut(idx + 1) |= c.masks[1];
        }
    }
    overlap
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::*;

    #[test]
    fn test() {
        assert_eq!(combi(&parse(&read(2018, "day03.txt"))), (104126, 695));
    }
}
