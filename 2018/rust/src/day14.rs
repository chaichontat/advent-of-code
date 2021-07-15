use crate::utils::*;
use ascii::AsciiString;

fn build(v: &[u8]) -> u64 {
    v.iter().fold(0, |acc, &i| acc * 10 + i as u64)
}

pub fn combi(raw: &[AsciiString]) -> SomeResult<(u64, u64)> {
    let target = raw[0].as_str().parse::<usize>()?;
    let mut elfs = (0, 1);
    let mut rgened = 0;
    let mut part1 = 0;
    let part2;
    let mut v = vec![1u8; 40_000_000];

    v[0] = 3;
    v[1] = 7;

    loop {
        let new_recipe = v[elfs.0] + v[elfs.1];
        if new_recipe >= 10 {
            rgened += 2;
            v[rgened + 1] = new_recipe - 10;
        } else {
            rgened += 1;
            v[rgened + 1] = new_recipe;
        }
        let len = rgened + 2;

        elfs = (
            (elfs.0 + 1 + v[elfs.0] as usize) % len,
            (elfs.1 + 1 + v[elfs.1] as usize) % len,
        );

        if rgened > target + 11 {
            part1 = build(&v[target..target + 10]);
        }

        if len > 7 {
            if build(&v[len - 6..len]) == target as u64 {
                part2 = len - 6;
                break;
            } else if build(&v[len - 7..len - 1]) == target as u64 {
                part2 = len - 7;
                break;
            }
        }
    }
    Ok((part1, part2 as u64))
}

mod tests {
    use super::*;

    #[test]
    fn test_combi() {
        assert_eq!(
            combi(&read_ascii("day14.txt")).unwrap(),
            (9315164154, 20231866)
        );
    }
}
