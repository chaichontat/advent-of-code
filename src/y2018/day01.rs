use std::{iter, num::ParseIntError};

use itertools::Itertools;
use num::Integer;

type Parsed = i32;

pub fn parse(raw: &str) -> Result<Vec<Parsed>, ParseIntError> {
    raw.split('\n').map(|x| x.parse()).try_collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Div {
    md:  i32,
    qt:  i32,
    idx: i16,
}

pub fn combi(parsed: &[Parsed]) -> Option<(u32, u32)> {
    let mut freq = iter::once(0).chain(parsed.iter().copied()).collect_vec();

    freq.iter_mut().fold(0, |acc, i| {
        *i += acc;
        *i
    });

    let sum = freq.pop()?;

    // Assuming that the answer is not in the first iteration.
    // Otherwise,
    // let mut unique = HashSet::new();
    // if freq.iter().any(|x| !unique.insert(*x)) {
    //     panic!("Answer in first iteration.");
    // }

    // https://www.reddit.com/r/adventofcode/comments/a20646/2018_day_1_solutions/
    // Things to note:
    // - The infinite sequence is the cumulative sum + n × total_sum.
    // - The first repeat must be in the first cumulative sum.
    //       Suppose an element in the n-th cumulative sum is first repeated. However,
    //       the n-th cumulative sum is the first + n × shift. Reversing the shift to
    //       both elements and we get a better repeat. Contradiction.
    //
    // Therefore, the solution is the cumsum that, for others to reach it, requires the lowest
    // `n × shift`.

    // Step 1. We partition cumsums that have the same remainder (modulo shift) into
    // "cosets" (we also save its quotient).
    //
    // Then, out of all cosets, find a pair of cumsums that has the lowest difference in their quotients.
    // We need to compare quotients beacuse each quotient represents the "starting point" of each cumsum.
    // If there are multiple candidates, pick the one with the lower index, since that will be reached first.

    // Assuming that the sum is strictly positive. Need to reverse some sorting if not.

    // Step 2
    // (diff, idx of cumsum whose upon adding (qt × shift) = goal, idx of said goal)
    // Need to sort using idx_pursuer because the cumsum seq is not monotonic.
    // That is, idx_goal could be < idx_pursuer.

    let divs = freq
        .iter()
        .enumerate()
        // Step 1
        .map(|(idx, &x)| {
            let (qt, md) = x.div_mod_floor(&sum);
            Div { md, qt, idx: idx as i16 }
        });

    let idx = divs
        .sorted_unstable()
        .tuple_windows::<(_, _)>()
        .filter_map(|(prev, now)| {
            if now.md == prev.md {
                Some((now.qt - prev.qt, prev.idx, now.idx))
            } else {
                None
            }
        })
        .min()?;

    Some((sum as u32, freq[idx.2 as usize] as u32))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::*;
    #[test]
    fn test() -> GenericResult<()> {
        assert_eq!(combi(&parse(&read(2018, "day01.txt"))?), Some((454, 566)));
        Ok(())
    }
}
