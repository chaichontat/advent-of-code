use rayon::iter::{IntoParallelIterator, ParallelIterator};

type Parsed = u8;

pub fn parse(raw: &str) -> Vec<Parsed> {
    raw.as_bytes().to_vec()
}

pub fn combi(p: &[Parsed]) -> (usize, usize) {
    let reacted = react(p, 0);

    let part2 = (1u8..=26)
        .into_par_iter()
        .map(|x| react(&reacted, x).len())
        .min()
        .unwrap();

    (reacted.len(), part2)
}

fn react(p: &[u8], skip: u8) -> Vec<u8> {
    let mut buf = Vec::with_capacity(p.len());
    for &c in p {
        if c & 31 == skip {
            continue;
        }
        // ASCII differentiates lowercases and uppercases using the 6th bit.
        if c ^ buf.last().unwrap_or(&0) == 32 {
            buf.pop();
        } else {
            buf.push(c)
        }
    }
    buf
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::*;

    #[test]
    fn test() {
        assert_eq!(combi(&parse(&read(2018, "day05.txt"))), (11298, 5148));
    }
}
