use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::str::from_utf8;

use itertools::Itertools;
use regex::Regex;
#[cfg(target_arch = "x86_64")]
use safe_arch::*;

type Parsed = (u8, u8);

pub fn parse(raw: &str) -> Vec<Parsed> {
    let re = Regex::new(r"[A-Z]").unwrap();
    raw.split('\n')
        .map(|x| {
            re.find_iter(x)
                .skip(1)
                .map(|m| m.as_str().as_bytes()[0] - b'A')
                .collect_tuple::<(_, _)>()
                .unwrap()
        })
        .collect_vec()
}

/// Build adjacency matrix with `m256i`s.
#[cfg(target_arch = "x86_64")]
fn build_adj_matrix(nodes: &[Parsed]) -> [m256i; 26] {
    let mut temp = [[0u8; 32]; 26];
    for &node in nodes {
        temp[node.0 as usize][node.1 as usize] += 1;
    }

    let mut outdeg = [zeroed_m256i(); 26];
    outdeg.iter_mut().zip(temp).for_each(|(c, t)| *c = m256i::from(t));
    debug_assert!(outdeg
        .iter()
        .all(|&x| cmp_gt_mask_i8_m256i(x, set_splat_i8_m256i(2)) == zeroed_m256i()));

    outdeg
}

#[cfg(target_arch = "x86_64")]
fn topological_sort(outdeg: &[m256i; 26], indeg: m256i) -> [u8; 26] {
    // Find node with cumulative in-degree == 0, pop out in alphabetical order and subtract their
    // contributions to other nodes' in-degrees.
    let mut sorted = [0u8; 26];
    let mut it = sorted.iter_mut();
    let mut indeg_rem = indeg;
    let mut todo = 0x03ff_ffff; // First 26 bits set to 1.
    loop {
        let no_incoming = todo & move_mask_i8_m256i(cmp_eq_mask_i8_m256i(indeg_rem, zeroed_m256i())); // Ignore finished node.
        if no_incoming == 0 {
            break;
        }

        let node = no_incoming.trailing_zeros();
        *it.next().unwrap() = b'A' + node as u8;
        indeg_rem = sub_i8_m256i(indeg_rem, outdeg[node as usize]);

        todo ^= 1 << node; // Mark this node as done.
    }
    sorted
}

#[cfg(target_arch = "x86_64")]
pub fn combi(nodes: &[(u8, u8)]) -> (String, u32) {
    let outdeg = build_adj_matrix(nodes);

    // Each vector represents "from" node
    // Big-endian so that A-Z corresponds to the first 26 bytes, but Intel is little-endian, so mask needs to be at the front.
    let indeg = outdeg.iter().fold(set_i64_m256i(0xffff_0000, 0, 0, 0), |acc, c| {
        add_i8_m256i(acc, *c)
    });

    let sorted = topological_sort(&outdeg, indeg);

    let mut labor = BinaryHeap::new();
    // Find node with cumulative in-degree == 0, pop out in alphabetical order and subtract their
    // contributions to other nodes' in-degrees.
    let mut indeg_rem = indeg;
    let mut todo = 0x03ff_ffff; // First 26 bits set to 1.
    let mut t = 0;
    let mut avail;
    loop {
        // Assign work on nodes w/o in-degrees, aka, pre-reqs satisfied.
        while labor.len() < 5 {
            avail = todo & move_mask_i8_m256i(cmp_eq_mask_i8_m256i(indeg_rem, zeroed_m256i())); // Ignore finished node.
            if avail == 0 {
                break;
            }
            let node = avail.trailing_zeros();
            labor.push(Reverse((t + 61 + node, node))); // t_finish
            todo ^= 1 << node; // Mark this node as assigned.
        }

        if labor.is_empty() {
            break;
        }

        // Working area.
        // Jump to the next finish time. Clear all nodes with the same finish time.
        t = labor.peek().unwrap().0 .0;
        loop {
            let (_, node) = labor.pop().unwrap().0;
            indeg_rem = sub_i8_m256i(indeg_rem, outdeg[node as usize]);

            if labor.peek().unwrap_or(&Reverse((u32::MAX, 0))).0 .0 > t {
                break;
            }
        }
    }

    (from_utf8(&sorted).unwrap().to_string(), t)
}

#[cfg(target_arch = "aarch64")]
pub fn combi(nodes: &[(u8, u8)]) -> (String, u32) {
    (String::from("o"), 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::*;

    #[cfg(target_arch = "x86_64")]
    #[test]
    fn test_combi() {
        assert_eq!(
            combi(&parse(&read(2018, "day07.txt"))),
            (String::from("ADEFKLBVJQWUXCNGORTMYSIHPZ"), 1120)
        );
    }
}
