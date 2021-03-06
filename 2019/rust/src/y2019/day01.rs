type Parsed = usize;

pub fn parse(raw: &str) -> Vec<Parsed> {
    raw.split('\n').map(|x| x.parse().unwrap()).collect()
}

fn fuel(x: &usize) -> Option<usize> {
    (*x / 3).checked_sub(2)
}

fn recurse(x: &usize) -> usize {
    match fuel(x) {
        Some(y) => recurse(&y) + y,
        _ => 0,
    }
}

pub fn part1(parsed: &[Parsed]) -> usize {
    parsed.iter().map(fuel).flatten().sum()
}

pub fn part2(parsed: &[Parsed]) -> usize {
    parsed.iter().map(recurse).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::*;

    #[test]
    fn test1() {
        assert_eq!(part1(&parse(&read(2019, "day01.txt"))), 3184233);
    }
    #[test]
    fn test2() {
        assert_eq!(part2(&parse(&read(2019, "day01.txt"))), 4773483);
    }
}
