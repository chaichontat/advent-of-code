use std::convert::TryFrom;

use hashbrown::HashMap;
use itertools::{izip, Itertools};
use num_enum::TryFromPrimitive;
use safe_arch::*;

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

#[allow(unused_unsafe)]
unsafe fn step_unchecked(map: &mut Map) {
    let m;
    unsafe {
        m = &mut *(map.get_unchecked_mut(PAD) as *mut u8 as *mut [m256i; (DIM + 1) * (PAD / BATCH)]);
        // Transmute into m256i and truncate first two "fillers".
    }
    let mut sum = [zeroed_m256i(); (DIM + 1) * (PAD / BATCH)];

    // Compute sum of self + L/R.
    for (y, s) in sum.iter_mut().enumerate() {
        let idx_c = PAD + (BATCH * y);
        unsafe {
            let ptr = map.get_unchecked(idx_c) as *const u8; // Shift by one u8.
            *s = add_i8_m256i(
                *(ptr as *const m256i),
                add_i8_m256i(
                    load_unaligned_m256i(&*(ptr.sub(1) as *const [i8; 32])),
                    load_unaligned_m256i(&*(ptr.add(1) as *const [i8; 32])),
                ),
            );
        }
    }

    // Add sum of above and below and subtract self.
    let mut prev = [zeroed_m256i(), zeroed_m256i()];
    let mut curr;
    let mut total = [sum[0], sum[1]]; // Contains curr+prev sums. Loop invariant.
    for y in (0..N_SUM).step_by(2) {
        // Note end range -> skip end pad.
        curr = [sum[y], sum[y + 1]];
        for i in 0..2 {
            total[i] = add_i8_m256i(total[i], sum[y + 2 + i]); // _mm256_add_epi8(total[i], sum[y + 2 + i]); // Add next, now have everything.
            sum[y + i] = sub_i8_m256i(total[i], m[y + i]); // Remove self.
            total[i] = sub_i8_m256i(total[i], prev[i]); // Remove prev-prev.
        }
        prev = curr;
    }

    for (i, (&s, ori)) in izip!(&sum, m.iter_mut()).take(N_SUM).enumerate() {
        let zero = zeroed_m256i();
        let two = set_splat_i8_m256i(2);
        let yard_m = set_splat_i8_m256i(YARD as i8);
        let tree_m = set_splat_i8_m256i(TREE as i8);

        #[allow(overflowing_literals)]
        let tree = shr_imm_u16_m256i::<4>(bitand_m256i(s, set_splat_i8_m256i(0xf0)));
        let yard = bitand_m256i(s, set_splat_i8_m256i(0x0f));

        let tree3 = cmp_gt_mask_i8_m256i(tree, two);
        let yard3 = cmp_gt_mask_i8_m256i(yard, two);
        let notyard = bitor_m256i(cmp_eq_mask_i8_m256i(tree, zero), cmp_eq_mask_i8_m256i(yard, zero));

        let curr_gnd = cmp_eq_mask_i8_m256i(*ori, zero);
        let curr_tree = cmp_eq_mask_i8_m256i(*ori, tree_m);
        let curr_yard = cmp_eq_mask_i8_m256i(*ori, yard_m);

        // An open acre will become filled with trees if three or more adjacent acres contained trees.
        *ori = bitxor_m256i(*ori, bitand_m256i(curr_gnd, bitand_m256i(tree3, tree_m)));

        // An acre filled with trees will become a lumberyard if three or more adjacent acres were lumberyards.
        *ori = bitxor_m256i(
            *ori,
            bitand_m256i(curr_tree, bitand_m256i(yard3, set_splat_i8_m256i(0x11))),
        );

        // An acre containing a lumberyard will remain a lumberyard if it was adjacent to at least one other lumberyard and at least one acre containing trees.
        *ori = bitxor_m256i(*ori, bitand_m256i(curr_yard, bitand_m256i(notyard, yard_m)));

        if i % 2 == 1 {
            *ori = bitand_m256i(*ori, set_i64_m256i(0, 0x1111, -1, -1));
        }
    }
}

#[allow(unused_unsafe)]
unsafe fn calc_value_unchecked(map: &Map) -> u32 {
    let mut tree = 0;
    let mut yard = 0;

    let pmap;
    unsafe {
        pmap = (&*(map as *const u8 as *const [m256i; (DIM + 2) * 2])).get_unchecked(2..N_SUM + 2);
    }

    for &y in pmap {
        tree += move_mask_i8_m256i(shl_imm_u16_m256i::<7>(y)).count_ones();
        yard += move_mask_i8_m256i(shl_imm_u16_m256i::<3>(y)).count_ones();
    }

    tree * yard
}

pub fn combi(mapal: &MapAlign) -> Option<(u32, u32)> {
    let mut mapal = mapal.clone();

    let mut count = 1u32;
    let mut seen = HashMap::with_capacity(1000);
    let mut value = HashMap::with_capacity(1000);

    loop {
        unsafe { step_unchecked(&mut mapal.0) };
        if let Some(old) = seen.insert(mapal.0, count) {
            let period = count - old;
            let idx = old + (1000000000 - old) % period;
            return Some((value[&10], value[&idx]));
        }
        unsafe { value.insert(count, calc_value_unchecked(&mapal.0)) };

        count += 1;
    }
}

mod tests {
    use super::{combi, parse};
    use crate::utils::read;

    #[test]
    fn test_combi() {
        assert_eq!(combi(&parse(&read(2018, "day18.txt"))), Some((536370, 190512)));
    }
}
