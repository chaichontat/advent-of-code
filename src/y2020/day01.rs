use bitvec::prelude::*;
use itertools::Itertools;

pub fn parse(raw: &str) -> Vec<usize> {
    raw.split('\n')
        .map(|line| line.parse::<usize>().unwrap())
        .collect_vec()
}

fn build_bit(parsed: &[usize]) -> BitArray<Lsb0, [usize; 32]> {
    let mut s = bitarr![0; 2048];
    for &n in parsed {
        s.set(n, true);
    }
    s
}

pub fn combi(parsed: &[usize]) -> (Option<usize>, Option<usize>) {
    let nums = build_bit(parsed);
    let have = nums.iter_ones().collect_vec();

    let (mut part1, mut part2) = (None, None);

    for &n in &have {
        if *nums.get(2020 - n).unwrap() {
            part1 = Some(n * (2020 - n));
            break;
        }
    }

    'outer: for i in 0..have.len() {
        for j in (i + 1)..have.len() {
            let o = 2020 - have[i] - have[j];
            if o < have[j] {
                break;
            }

            if *nums.get(o).unwrap() {
                part2 = Some(o * have[i] * have[j]);
                break 'outer;
            }
        }
    }

    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::*;

    #[test]
    fn test_combi() {
        assert_eq!(
            combi(&parse(&read(2020, "day01.txt"))),
            (Some(605364), Some(128397680))
        );
    }
}
