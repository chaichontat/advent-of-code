use std::arch::x86_64::*;
use std::convert::TryFrom;
use std::mem;

use ahash::AHashMap;
use itertools::Itertools;
use num_enum::TryFromPrimitive;

#[repr(u8)]
#[derive(Debug, Copy, Clone, TryFromPrimitive)]
pub enum Block {
    Gnd  = 0x00,
    Tree = 0x01,
    Yard = 0x10,
}

const BATCH: usize = 32;
// Conway Game of Life.

const DIM: usize = 50;
const GND: u8 = 0;
const N_SUM: usize = DIM * (PAD / BATCH);
const PAD: usize = 64;
const TREE: u8 = 0x10;
const YARD: u8 = 0x01;

type Map = [u8; (DIM + 2) * (PAD)];
#[repr(C, align(32))] // AVX
#[derive(Debug, Clone)]
pub struct MapAlign(Map);

// impl PartialOrd for MapAlign {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         unsafe {
//             let ptr_s = self.0.get_unchecked(PAD) as *const u8 as *const __m256i;
//             let ptr_o = other.0.get_unchecked(PAD) as *const u8 as *const __m256i;
//             for y in 0..N_SUM {
//                 let eq: i32 = _mm256_movemask_epi8(_mm256_cmpeq_epi8(*ptr_s.add(y), *ptr_o.add(y)));
//                 let gt: i32 = _mm256_movemask_epi8(_mm256_cmpgt_epi8(*ptr_s.add(y), *ptr_o.add(y)));
//                 if !eq != 0 {
//                     return match (eq.wrapping_add(gt << 1)) > gt {
//                         true => Some(Ordering::Greater),
//                         false => Some(Ordering::Less),
//                     };
//                 }
//             }
//             Some(Ordering::Equal)
//         }
//         // Some(self.0.cmp(&other.0))
//     }
// }

// impl Ord for MapAlign {
//     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//         self.partial_cmp(other).unwrap()
//     }
// }

// impl PartialEq for MapAlign {
//     fn eq(&self, other: &Self) -> bool {
//         self.partial_cmp(other) == Some(Ordering::Equal)
//     }
// }

// impl Eq for MapAlign {}

impl MapAlign {
    pub fn print_map(&self) {
        for i in 0..DIM + 2 {
            for j in 0..PAD {
                let x = match self.0[i * PAD + j] {
                    GND => ".",
                    TREE => "|",
                    YARD => "#",
                    _ => panic!(),
                };
                print!("{}", x);
            }
            println!();
        }
        println!();
    }
}

pub fn parse(raw: &str) -> MapAlign {
    let sp = raw.split('\n').collect_vec();
    let mut out = [0; (DIM + 2) * (PAD)];
    for (i, &line) in sp.iter().enumerate() {
        out[PAD * (i + 1)..PAD * (i + 1) + DIM]
            .iter_mut()
            .zip(line.as_bytes())
            .for_each(|(b, &n)| *b = Block::try_from(n & 0x11).unwrap() as u8);
    }
    MapAlign(out)
}

fn step(map: &mut Map) {
    let mut sum: [__m256i; (DIM + 1) * (PAD / BATCH)];
    unsafe {
        sum = mem::zeroed();
        // Compute sum of self + L/R.
        for (y, s) in sum.iter_mut().enumerate() {
            let idx_c = PAD + (BATCH * y);
            let ptr = map.get(idx_c).unwrap() as *const u8; // Shift by one u8.
            *s = _mm256_add_epi8(
                *(ptr as *const __m256i),
                _mm256_add_epi8(
                    _mm256_loadu_si256(ptr.sub(1) as *const __m256i),
                    _mm256_loadu_si256(ptr.add(1) as *const __m256i),
                ),
            );
        }

        // Add sum of above and below and subtract self.
        let mut prev = [_mm256_setzero_si256(), _mm256_setzero_si256()];
        let mut curr;
        let mut total = [sum[0], sum[1]]; // Contains curr+prev sums. Loop invariant.
        for y in (0..N_SUM).step_by(2) {
            // Note end range -> skip end pad.
            curr = [sum[y], sum[y + 1]];
            let ptr = &map[PAD + (BATCH * y)] as *const u8 as *const __m256i;
            for i in 0..2 {
                total[i] = _mm256_add_epi8(total[i], sum[y + 2 + i]); // Add next, now have everything.
                sum[y + i] = _mm256_sub_epi8(total[i], *ptr.add(i)); // Remove self.
                total[i] = _mm256_sub_epi8(total[i], prev[i]); // Remove prev-prev.
            }
            prev = curr;
        }
    }

    for y in 0..N_SUM {
        let idx_c = PAD + (BATCH * y);

        #[allow(overflowing_literals)]
        unsafe {
            let zero = _mm256_setzero_si256();
            let two = _mm256_set1_epi8(2);
            let yard_m = _mm256_set1_epi8(YARD as i8);
            let tree_m = _mm256_set1_epi8(TREE as i8);

            let s = *sum.get_unchecked(y);
            let tree = _mm256_srli_epi16(_mm256_and_si256(s, _mm256_set1_epi8(0xf0)), 4);
            let yard = _mm256_and_si256(s, _mm256_set1_epi8(0x0f));

            let tree3 = _mm256_cmpgt_epi8(tree, two);
            let yard3 = _mm256_cmpgt_epi8(yard, two);
            let notyard = _mm256_or_si256(_mm256_cmpeq_epi8(tree, zero), _mm256_cmpeq_epi8(yard, zero));

            let mut m = *(map.get_unchecked(idx_c) as *const u8 as *const __m256i);
            let curr_gnd = _mm256_cmpeq_epi8(m, zero);
            let curr_tree = _mm256_cmpeq_epi8(m, tree_m);
            let curr_yard = _mm256_cmpeq_epi8(m, yard_m);

            // An open acre will become filled with trees if three or more adjacent acres contained trees.
            m = _mm256_xor_si256(m, _mm256_and_si256(curr_gnd, _mm256_and_si256(tree3, tree_m)));

            // An acre filled with trees will become a lumberyard if three or more adjacent acres were lumberyards.
            m = _mm256_xor_si256(
                m,
                _mm256_and_si256(curr_tree, _mm256_and_si256(yard3, _mm256_set1_epi8(0x11))),
            );

            // An acre containing a lumberyard will remain a lumberyard if it was adjacent to at least one other lumberyard and at least one acre containing trees.
            m = _mm256_xor_si256(m, _mm256_and_si256(curr_yard, _mm256_and_si256(notyard, yard_m)));

            if y % 2 == 1 {
                m = _mm256_and_si256(m, _mm256_set_epi64x(0, 0x1111, -1, -1));
            }

            let out: [u8; 32] = mem::transmute(m);
            map.get_unchecked_mut(idx_c..idx_c + BATCH).copy_from_slice(&out);
        }
    }
}

fn calc_value(map: &Map) -> u32 {
    let mut tree = 0;
    let mut yard = 0;
    unsafe {
        let ptr = map.get_unchecked(PAD) as *const u8 as *const __m256i;
        for y in 0..N_SUM {
            tree += _mm256_movemask_epi8(_mm256_slli_epi64(*ptr.add(y), 3)).count_ones();
            yard += _mm256_movemask_epi8(_mm256_slli_epi64(*ptr.add(y), 7)).count_ones();
        }
    }
    tree * yard
}

pub fn combi(mapal: &MapAlign) -> (u32, u32) {
    let mut mapal = mapal.clone();

    let mut count = 1u32;
    let mut seen = AHashMap::with_capacity(1000);
    let mut value = AHashMap::with_capacity(1000);

    loop {
        step(&mut mapal.0);
        if let Some(old) = seen.insert(mapal.0, count) {
            let period = count - old;
            let idx = old + (1000000000 - old) % period;
            return (value[&10], value[&idx]);
        }
        value.insert(count, calc_value(&mapal.0));

        count += 1;
    }
}

mod tests {
    use super::{combi, parse};
    use crate::utils::read;

    #[test]
    fn test_combi() {
        assert_eq!(combi(&parse(&read(2018, "day18.txt"))), (536370, 190512));
    }
}
