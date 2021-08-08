use packed_simd::i16x16;
use rayon::prelude::*;

type Parsed = i32;

pub fn parse(raw: &str) -> Parsed {
    raw.parse().unwrap()
}

const DIM: usize = 300;

// https://stackoverflow.com/questions/1766535/bit-hack-round-off-to-multiple-of-8/1766565
const PAD: usize = ((DIM + 2) + 15) & !15;
type Arr = [[i16; PAD]; PAD];

/// f(x, y; s) = r(ry + s)
/// where r = x + 10;
///
/// Add along the y-axis.
/// ∂/∂y f = r².
#[allow(non_snake_case)]
pub fn build_arr(sn: i32) -> Arr {
    // Index starts at 1.
    let mut dY = [0i32; PAD];
    let mut arr = [[0i32; PAD]; PAD]; // [y][x]
    let mut out = [[0i16; PAD]; PAD]; // [y][x]

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
        snd[0].copy_from_slice(&fst[y]); // arr[y+1] = arr[y] + dY
        snd[0].iter_mut().zip(dY).for_each(|(xp, dy)| *xp += dy);

        out[y]
            .iter_mut()
            .zip(arr[y])
            .for_each(|(o, x)| *o = ((x / 100) % 10 - 5) as i16);
        out[y][0] = 0;
    }
    out
}

/// https://en.wikipedia.org/wiki/Summed-area_table
fn build_summed(mut arr: Arr) -> Arr {
    for y in 1..=DIM {
        // y-direction.
        let (fst, snd) = arr.split_at_mut(y + 1);
        for (xp, x) in snd[0].iter_mut().zip(fst[y]) {
            *xp = xp.wrapping_add(x); // arr[y + 1] += arr[y];
        }

        // x-direction.
        fst[y][1..=DIM].iter_mut().fold(0, |acc, x| {
            *x = x.wrapping_add(acc); // Cumulative sum.
            *x
        });
    }
    arr
}

/// Sum is (x₀, x₁]. To get the sum of [1:3], we need x₃ - x₀.
fn find_max(summed: &Arr, win: usize) -> (i16, (usize, usize)) {
    let len = DIM - win;

    let mut max = i16::MIN;
    let mut vmax = i16x16::splat(max);
    let mut ans = (0usize, 0usize);
    let mut sum;

    for y in 0..len {
        for i in (0..PAD - win - 16).step_by(16) {
            // from_slice_unaligned_unchecked is bad when len(slice) < len(lane). Cannot happen from loop structure.
            // Both get_unchecked are impossible to be out-of-bound due to constraints on y and i.
            unsafe {
                let _tl =
                    i16x16::from_slice_unaligned_unchecked(summed.get_unchecked(y).get_unchecked(i..i + 16));
                let _br = i16x16::from_slice_unaligned_unchecked(
                    summed.get_unchecked(y + win).get_unchecked(win + i..win + i + 16),
                );
                let _bl = i16x16::from_slice_unaligned_unchecked(
                    summed.get_unchecked(y + win).get_unchecked(i..i + 16),
                );
                let _tr = i16x16::from_slice_unaligned_unchecked(
                    summed.get_unchecked(y).get_unchecked(win + i..win + i + 16),
                );
                sum = (_tl + _br) - (_bl + _tr);
            }

            if sum.gt(vmax).any() {
                let sum: [i16; 16] = sum.into();
                for (j, &u) in sum.iter().enumerate() {
                    if i + j > len {
                        break;
                    }
                    if u > max {
                        max = u;
                        vmax = i16x16::splat(max);
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

#[cfg(test)]
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
//
// fn gen_power(x: i32, y: i32, sn: i32) -> i32 {
//     let r_id = x + 10;
//     let power = (r_id * y + sn) * r_id;
//     let power = (power / 100) % 10;
//     power - 5
// }
