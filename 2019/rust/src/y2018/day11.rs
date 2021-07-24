use itertools::{izip, Itertools};
use ndarray::prelude::*;
use packed_simd::i32x8;
use rayon::prelude::*;

use crate::utils::*;

type Parsed = i32;

pub fn parse(raw: &str) -> Parsed {
    raw.parse().unwrap()
}

const DIM: usize = 300;

// https://stackoverflow.com/questions/1766535/bit-hack-round-off-to-multiple-of-8/1766565
const PAD: usize = ((DIM + 2) + 7) & !7;
type Arr = [[i32; PAD]; PAD];

fn gen_power(x: i32, y: i32, sn: i32) -> i32 {
    let r_id = x + 10;
    let power = (r_id * y + sn) * r_id;
    let power = (power / 100) % 10;
    power - 5
}

/// f(x, y; s) = r(ry + s)
/// where r = x + 10;
///
/// Add along the y-axis.
/// ∂/∂y f = r².
#[allow(non_snake_case)]
fn build_arr(sn: i32) -> Arr {
    // Index starts at 1.

    let mut dY = [0i32; PAD];
    let mut arr = [[0i32; PAD]; PAD]; // [y][x]

    dY.iter_mut().enumerate().for_each(|(x, r)| {
        let xp = x as i32 + 10;
        *r = xp * xp;
    });
    dY[0] = 0;

    arr[1].iter_mut().enumerate().for_each(|(x, r)| {
        let xp = x as i32 + 10;
        *r = (sn + xp) * xp;
    });
    arr[1][0] = 0;

    for y in 1..=DIM {
        let (fst, snd) = arr.split_at_mut(y + 1);
        for (xp, x, dy) in izip!(snd[0].iter_mut(), fst[y], dY) {
            *xp = x + dy;
        }

        arr[y].iter_mut().for_each(|x| *x = (*x / 100) % 10 - 5);
        arr[y][0] = 0;
    }
    arr
}

/// https://en.wikipedia.org/wiki/Summed-area_table
fn build_summed(mut arr: Arr) -> Arr {
    for y in 1..=DIM {
        // y-direction.
        let (fst, snd) = arr.split_at_mut(y + 1);
        for (xp, x) in snd[0].iter_mut().zip(fst[y]) {
            *xp += x; // arr[y + 1] += arr[y];
        }

        // x-direction.
        fst[y][1..=DIM].iter_mut().fold(0, |acc, x| {
            *x += acc; // Cumulative sum.
            *x
        });
    }
    arr
}

/// Sum is (x₀, x₁]. To get the sum of [1:3], we need x₃ - x₀.
fn find_max(summed: &Arr, win: usize) -> (i32, (usize, usize)) {
    let len = DIM - win;

    let mut max = i32::MIN;
    let mut vmax = i32x8::splat(max);
    let mut ans = (0usize, 0usize);
    let mut sum = i32x8::splat(0);

    for y in 0..len {
        for i in (0..PAD - win - 8).step_by(8) {
            unsafe {
                let _tl = i32x8::from_slice_unaligned_unchecked(
                    &summed.get_unchecked(y).get_unchecked(i..i + 8),
                );
                let _br = i32x8::from_slice_unaligned_unchecked(
                    &summed
                        .get_unchecked(y + win)
                        .get_unchecked(win + i..win + i + 8),
                );
                let _bl = i32x8::from_slice_unaligned_unchecked(
                    &summed.get_unchecked(y + win).get_unchecked(i..i + 8),
                );
                let _tr = i32x8::from_slice_unaligned_unchecked(
                    &summed.get_unchecked(y).get_unchecked(win + i..win + i + 8),
                );
                sum = (_tl + _br) - (_bl + _tr);
            }

            if sum.gt(vmax).any() {
                let sum: [i32; 8] = sum.into();
                for (j, &u) in sum.iter().enumerate() {
                    if i + j > len {
                        break;
                    }
                    if u > max {
                        max = u;
                        vmax = i32x8::splat(max);
                        ans.0 = i + j + 1;
                        ans.1 = y + 1;
                    }
                }
            }
        }
    }
    (max, ans)
}

pub fn combi(parsed: &Parsed) -> ((usize, usize), (usize, usize, usize)) {
    let summed = build_summed(build_arr(*parsed));

    // https://github.com/Voltara/advent2018-fast/blob/master/README.md#day-11
    // Presumably from experiments with randomized serial numbers.
    let out = (1..35)
        .into_par_iter()
        .map(|i| {
            let (max, loc) = find_max(&summed, i);
            (max, loc, i)
        })
        .collect::<Vec<_>>();

    let part1 = out[2].1;
    let part2 = *out.iter().max().unwrap();
    let part2 = ((part2.1).0, (part2.1).1, part2.2);

    (part1, part2)
}

mod tests {
    use super::*;
    use crate::utils::*;

    #[test]
    fn test_combi() {
        assert_eq!(
            combi(&parse(&read(2018, "day11.txt"))),
            ((235, 22), (231, 135, 8))
        );
    }

    #[test]
    fn test_integral() {
        assert_eq!(build_arr(57)[79][122], -5);
        assert_eq!(build_arr(39)[196][217], 0);
        assert_eq!(build_arr(71)[153][101], 4);
    }

    #[test]
    fn test_max() {
        assert_eq!(find_max(&build_summed(build_arr(18)), 16), (113, (90, 269)));
        assert_eq!(
            find_max(&build_summed(build_arr(42)), 12),
            (119, (232, 251))
        )
    }
}

// Initial version before SIMD.
// fn find_max_nonsimd(summed: &Arr, win: usize) -> (i32, (usize, usize)) {
//     let mut max = i32::MIN;
//     let mut ans = [0usize; 2];
//     let mut tmp = [0; PAD];
//     let len = DIM - win;

//     for y in 0..len {
//         let tl = &summed[y][..len];
//         let br = &summed[y + win][win..DIM];
//         let bl = &summed[y + win][..len];
//         let tr = &summed[y][win..DIM];

//         for (x, i) in tmp.iter_mut().zip(0..len) {
//             *x = (tl[i] + br[i]) - (bl[i] + tr[i]);
//         }

//         for (&u, x) in tmp.iter().zip(0..len) {
//             if u > max {
//                 max = u;
//                 ans[0] = x + 1;
//                 ans[1] = y + 1;
//             }
//         }
//     }
//     (max, (ans[0], ans[1]))
// }
