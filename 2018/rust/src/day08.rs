use itertools::Itertools;

pub fn parse(raw: String) -> Vec<u16> {
    raw.split_ascii_whitespace()
        .map(|x| x.parse::<u16>().unwrap())
        .collect_vec()
}

fn recurse(p: &[u16], mut idx: usize) -> (usize, u16, u16) {
    let n_child = p[idx];
    let n_met = p[idx + 1];

    if n_child == 0 {
        if n_met == 0 {
            return (idx + 2, 0, 0);
        } else {
            let v = p[idx + 2..idx + 2 + n_met as usize].iter().sum();
            return (idx + 2 + n_met as usize, v, v);
        }
    }
    let mut sum = (0, 0);
    let mut children = [0; 10]; // Max children node is 10.

    idx += 2;
    for i in 0..n_child {
        let r = recurse(p, idx);
        idx = r.0;
        sum.0 += r.1;
        children[i as usize] = r.2;
    }

    for mets in &p[idx..idx + n_met as usize] {
        sum.0 += *mets;
        if (1..=n_child).contains(mets) {
            sum.1 += children[*mets as usize - 1];
        }
    }

    (idx + n_met as usize, sum.0, sum.1)
}

pub fn combi(parsed: &[u16]) -> (u16, u16) {
    let out = recurse(parsed, 0);
    (out.1, out.2)
}

mod tests {
    use super::*;
    use crate::utils::*;

    #[test]
    fn test_combi() {
        assert_eq!(combi(&parse(read_nosep("day08.txt"))), (48260, 25981));
    }
}
