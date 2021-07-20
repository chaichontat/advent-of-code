use nohash_hasher::IntSet;

type Parsed = String;
pub fn parse(raw: &str) -> Vec<Parsed> {
    raw.split('\n').map(|x| x.to_string()).collect()
}

fn parse_c(raw: &[String]) -> u32 {
    raw.concat()
        .chars()
        .enumerate()
        .map(|(i, c)| match c {
            '#' => 1 << i,
            '.' => 0,
            _ => unreachable!(),
        })
        .sum()
}

pub fn part1(parsed: &[Parsed]) -> u32 {
    fn run(arr: u32, dim: (u32, u32)) -> u32 {
        let mut new = 0;
        for i in 0..(dim.0 * dim.1) {
            // Mask
            let mut mask = 0;
            if let Some(res) = i.checked_sub(dim.0) {
                mask |= 1 << res
            }
            if i % dim.1 != 0 {
                mask |= 1 << (i - 1); // Not left
            }
            if i % dim.1 != 4 {
                mask |= 1 << (i + 1); // Not right
            }
            if i + dim.0 < dim.0 * dim.1 {
                mask |= 1 << (i + dim.0);
            }

            // Count and gen new state.
            let c = (mask & arr).count_ones();
            let r = (1 << i) & arr; // Check if alive.
            if c == 1 || (r == 0 && c == 2) {
                new |= 1 << i;
            }
        }
        new
    }

    let dim = (parsed[0].len() as u32, parsed.len() as u32);
    let mut arr = parse_c(parsed);
    let mut set = IntSet::default();
    loop {
        if !set.insert(arr) {
            return arr;
        }
        arr = run(arr, dim)
    }
}

pub fn part2(parsed: &[Parsed]) -> u32 {
    fn run(arr: u32, top: u32, bottom: u32) -> u32 {
        let mut new = 0;
        let center = 12u32;
        let dim = 5u32;

        for i in 0..25 {
            if i == center {
                continue;
            }

            // Mask
            let mut mask = 0;
            let mut c = 0;

            if let Some(res) = i.checked_sub(dim) {
                mask |= 1 << res
            } else {
                c += (top & (1 << 7)) >> 7;
            }

            if i % dim != 0 {
                mask |= 1 << (i - 1); // Not left
            } else {
                c += (top & (1 << 11)) >> 11;
            }

            if i % dim != 4 {
                mask |= 1 << (i + 1); // Not right
            } else {
                c += (top & (1 << 13)) >> 13;
            }

            if i + dim < 25 {
                mask |= 1 << (i + dim);
            } else {
                c += (top & (1 << 17)) >> 17;
            }

            if i == 7 {
                c += (bottom & 31).count_ones(); // Top mask
            } else if i == 11 {
                c += (bottom & 1082401).count_ones(); // Left mask
            } else if i == 13 {
                c += (bottom & 17318416).count_ones(); // Right mask
            } else if i == 17 {
                c += (bottom & 32505856).count_ones(); // Bottom mask
            }

            // Count and gen new state.
            c += (mask & arr).count_ones();
            let r = (1 << i) & arr; // Check if alive.
            if c == 1 || (r == 0 && c == 2) {
                new |= 1 << i;
            }
        }
        new
    }

    let arr = parse_c(parsed);
    let mut arrs = [0; 2 * 200 + 3]; // Iter
    let mut new = [0; 2 * 200 + 3];
    let center = 201;
    arrs[center] = arr;

    for i in 1..=200 {
        for n in (center - i)..=(center + i) {
            if arrs[n - 1] | arrs[n] | arrs[n + 1] != 0 {
                new[n] = run(arrs[n], arrs[n - 1], arrs[n + 1]);
            }
        }
        arrs[(center - i)..=(center + i)].clone_from_slice(&new[(center - i)..=(center + i)]);
    }

    arrs.iter().map(|&i| i.count_ones()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::*;
    #[test]
    fn test1() {
        assert_eq!(part1(&parse(&read(2019, "day24.txt"))), 18350099);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(&parse(&read(2019, "day24.txt"))), 2037);
    }
}
