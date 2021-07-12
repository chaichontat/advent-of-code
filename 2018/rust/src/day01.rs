use itertools::Itertools;
use std::iter;

pub fn bench(raw: &[String]) -> (u32, u32) {
    let mut freq = iter::once(0)
        .chain(raw.iter().map(|x| x.parse::<i32>().unwrap()))
        .collect_vec();

    freq.iter_mut().fold(0, |acc, i| {
        *i += acc;
        *i
    });

    let sum = freq.pop().unwrap();

    // Assuming that the answer is not in the first iteration.
    // Otherwise,
    // let mut unique = AHashSet::new();
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

    let all_in_one = freq
        .iter()
        .enumerate()
        // Step 1
        .map(|(idx, &x)| {
            let mut div = x / sum;
            let mut md = x % sum;
            if md < 0 {
                md += sum;
                div -= 1;
            }
            (md, div, idx as i16)
        })
        .sorted_unstable()
        .tuple_windows::<(_, _)>()
        // Step 2
        // (diff, idx of cumsum whose upon adding (qt × shift) = goal, idx of said goal)
        // Need to sort using idx_pursuer because the cumsum seq is not monotonic.
        // That is, idx_goal could be < idx_pursuer.
        .filter_map(|(prev, (md, qt, idx))| {
            if md == prev.0 {
                Some((qt - prev.1, prev.2, idx))
            } else {
                None
            }
        })
        .min();

    (sum as u32, freq[all_in_one.unwrap().2 as usize] as u32)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::*;
    #[test]
    fn test() {
        assert_eq!(bench(&read("day01.txt")), (454, 566));
    }
}
