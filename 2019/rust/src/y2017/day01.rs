#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

use safe_arch::*;

use crate::utils::ModAdd;

const LEN: usize = 2136;
const PAD: usize = (2136 + 31) & !31;
const N_PACKED: usize = PAD / 32;

pub fn parse(raw: &str) -> [u8; LEN] {
    let mut out = [0u8; LEN];
    for (dst, &src) in out.iter_mut().zip(raw.as_bytes()) {
        *dst = src - b'0';
    }
    out
}

pub fn combi_ori(ns: &[u8; LEN]) -> (u16, u16) {
    let part1 = {
        let p1: u16 = ns
            .windows(2)
            .filter_map(|x| if x[0] == x[1] { Some(x[0] as u16) } else { None })
            .sum();
        p1 + if ns[0] == ns[ns.len() - 1] { ns[0] as u16 } else { 0 }
    };

    let part2 = ns
        .iter()
        .enumerate()
        .filter_map(|(i, &x)| if x == ns[i.mod_add(LEN / 2, LEN)] { Some(x as u16) } else { None })
        .sum();

    (part1, part2)
}

/// # Safety
/// Pointer must be aligned for the first argument.
#[inline(always)]
unsafe fn cmp_sum(mut ptr: *const __m256i, mut ptr_u: *const __m256i) -> u16 {
    let mut out = 0u16;
    let mut sum = zeroed_m256i();
    let mut i = 0;

    for _ in 0..N_PACKED {
        if i == 256 / 9 {
            out += <[u8; 32]>::from(sum).iter().map(|&x| x as u16).sum::<u16>();
            sum = zeroed_m256i();
            i = 0;
        }
        let curr = m256i(unsafe { _mm256_load_si256(ptr) });
        sum = add_i8_m256i(
            sum,
            bitand_m256i(
                cmp_eq_mask_i8_m256i(curr, m256i(unsafe { _mm256_loadu_si256(ptr_u) })),
                curr,
            ),
        );
        i += 1;
        ptr = unsafe { ptr.add(1) };
        ptr_u = unsafe { ptr_u.add(1) };
    }
    out + <[u8; 32]>::from(sum).iter().map(|&x| x as u16).sum::<u16>()
}

#[repr(align(32))]
#[derive(Clone)]
struct Line([u8; PAD + 1]);

#[rustfmt::skip]
pub fn combi(ns: &[u8; LEN]) -> (u16, u16) {
    let mut line = Line([0u8; PAD + 1]);
    line.0[1..LEN + 1].copy_from_slice(ns);
    line.0[LEN + 1] = ns[0];
    let line = line;

    let ptr = line.0.as_ptr() as *const __m256i;
    let ptr_u = unsafe { line.0.as_ptr().add(1) as *const __m256i };
    let part1 = unsafe { cmp_sum(ptr, ptr_u) };

    let mut split = Line([0u8; PAD + 1]);
    split.0[       ..LEN / 2].copy_from_slice(&ns[LEN / 2..       ]);
    split.0[LEN / 2..LEN    ].copy_from_slice(&ns[       ..LEN / 2]);
    let split = split;

    let ptr = split.0.as_ptr() as *const __m256i;
    let part2 = unsafe { cmp_sum(ptr, ptr_u) };

    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::{combi, combi_ori, parse};
    use crate::utils::read;

    #[test]
    fn test_combi_simd() {
        assert_eq!(combi(&parse(&read(2018, "day12.txt"))), (1253, 1278));
    }

    #[test]
    fn test_combi() {
        assert_eq!(combi_ori(&parse(&read(2018, "day12.txt"))), (1253, 1278));
    }
}
