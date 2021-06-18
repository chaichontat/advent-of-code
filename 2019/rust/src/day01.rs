use super::utils::*;

fn fuel(x: usize) -> Option<usize> {
    (x / 3).checked_sub(2)
}

pub fn part1(raw: &[String]) -> usize {
    raw.iter().map(|x| int(x)).map(fuel).flatten().sum()
}

fn recurse(x: usize) -> usize {
    match fuel(x) {
        Some(y) => recurse(y) + y,
        _ => 0,
    }
}

pub fn part2(raw: &[String]) -> usize {
    raw.iter().map(|x| int(x)).map(recurse).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(part1(&read("day01.txt")), 3184233);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(&read("day01.txt")), 4773483);
    }
}
