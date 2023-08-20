#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

use partitions::partition_vec;

type Parsed = i8;
const N_DATA: usize = 1500; // Exceeding this results in undefined behavior!
const LEN: usize = (4 * N_DATA + 31) & !31;

pub fn parse(raw: &str) -> Vec<Parsed> {
    let out = raw
        .split('\n')
        .flat_map(|line| line.split(',').map(|s| s.parse().unwrap()))
        .collect();
    assert!(Vec::<Parsed>::len(&out) < LEN);
    out
}

#[repr(align(32))]
struct Data([i8; LEN]);

/// # Safety
/// Number of elements in the input file must be less than N_DATA for data alignment.
#[cfg(target_arch = "x86_64")]
pub unsafe fn part1(parsed: &[Parsed]) -> usize {
    let len = parsed.len() / 4;
    let mut arr = Data([0; LEN]);
    arr.0[..4 * len].copy_from_slice(parsed);
    let ptr = arr.0.as_ptr() as *const i32; // 4 i8.

    let mut pv = partition_vec![0u8; len];

    'outer: for i in 0..len {
        unsafe {
            let to_cmp = _mm256_set1_epi32(*(ptr.add(i))); // Broadcast (point i) 8 times.
            for j in (0..len).step_by(8) {
                let d = _mm256_abs_epi8(_mm256_sub_epi8(
                    to_cmp,
                    _mm256_load_si256(ptr.add(j) as *const __m256i),
                ));
                let d = _mm256_add_epi32(d, _mm256_slli_si256(d, 2)); // Shift each point by 16 bits and sum to create cumumlative sum.
                let d = _mm256_add_epi32(d, _mm256_slli_si256(d, 1));
                let d = _mm256_cmpgt_epi32(_mm256_set1_epi32(0x04_00_00_00), d); // 0xFF for each i8 if 4 > sum.
                let mut m: i32 = _mm256_movemask_epi8(d); // Return value of MSB of each i8.

                if m == 0 {
                    continue;
                }

                m &= 0x11111111;
                let base = j;
                loop {
                    let idx = base + m.trailing_zeros() as usize / 4;
                    if idx >= i {
                        continue 'outer;
                    }
                    pv.union(i, idx);
                    m &= m - 1;
                    if m == 0 {
                        break;
                    }
                }
            }
        }
    }
    pv.amount_of_sets()
}

#[cfg(target_arch = "aarch64")]
pub fn part1(parsed: &[Parsed]) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::utils::read;

    #[cfg(target_arch = "x86_64")]
    #[test]
    fn test_part1() {
        assert_eq!(unsafe { part1(&parse(&read(2018, "day25.txt"))) }, 338);
    }
}
