fn fuel(x: usize) -> Option<usize> {
    (x / 3).checked_sub(2)
}

fn recurse(x: usize) -> usize {
    match fuel(x) {
        Some(y) => recurse(y) + y,
        _ => 0,
    }
}

pub fn part1(raw: &[String]) -> usize {
    raw.iter()
        .map(|x| x.parse::<usize>().ok().and_then(fuel))
        .flatten()
        .sum()
}

pub fn part2(raw: &[String]) -> usize {
    raw.iter()
        .map(|x| x.parse::<usize>().ok().map(recurse))
        .flatten()
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::*;

    #[test]
    fn test1() {
        assert_eq!(part1(&read("day01.txt")), 3184233);
    }
    #[test]
    fn test2() {
        assert_eq!(part2(&read("day01.txt")), 4773483);
    }
}
