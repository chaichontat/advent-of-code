use std::arch::x86_64::{__m256i, _mm256_loadu_si256};
use std::convert::TryFrom;
use std::fmt;
use std::ops;
use std::str;

use bitvec::prelude::*;
use itertools::izip;
use safe_arch::*;

#[repr(C, align(32))]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct BitMap<const N: usize> {
    padf: m256i,
    m:    [m256i; N],
    padb: m256i,
    yx:   (u16, u16),
}

impl<const N: usize> fmt::Display for BitMap<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = [b'.'; 256 * N];
        for (x, offset) in self.m.iter().zip((0..256 * N).step_by(256)) {
            let bits = BitArray::<Lsb0, _>::new(<[u8; 32]>::from(*x));
            for b in bits.iter_ones() {
                out[offset + b] = b'#';
            }
        }
        let s = out
            .chunks_exact(self.yx.1 as usize)
            .map(str::from_utf8)
            .flatten()
            .join("\n");
        write!(f, "{}", s)
    }
}

impl<const N: usize> BitMap<N> {
    fn new(seq: &[u8], c: u8) -> Self {
        let mask = set_splat_i8_m256i(c as i8);

        let mut ms = [m256i::default(); N];
        for (m, chunk) in ms.iter_mut().zip(seq.chunks_exact(32 * 8)) {
            let mut packed = [0i32; 8];
            for (p, c) in packed.iter_mut().zip(chunk.chunks_exact(32)) {
                *p = move_mask_i8_m256i(cmp_eq_mask_i8_m256i(
                    m256i::from(*<&[u8; 32]>::try_from(c).unwrap()),
                    mask,
                ));
            }
            *m = m256i::from(packed);
        }
        BitMap { m: ms, ..Default::default() }
    }
}

impl<const N: usize> ops::BitAnd for &BitMap<N> {
    type Output = BitMap<N>;

    fn bitand(self, rhs: Self) -> Self::Output {
        let mut m = [m256i::default(); N];
        for (o, x, y) in izip!(m.iter_mut(), self.m.iter(), rhs.m.iter()) {
            *o = bitand_m256i(load_m256i(x), load_m256i(y));
        }
        BitMap { m, ..Default::default() }
    }
}
