use super::intcode::*;

fn run_ic(raw: &[String], input: isize) -> usize {
    let mut ic = IntCode::from(&raw[0]);
    ic.push(input);
    ic.run();
    ic.pop().unwrap() as usize
}

pub fn part1(raw: &[String]) -> usize {
    run_ic(raw, 1)
}

pub fn part2(raw: &[String]) -> usize {
    run_ic(raw, 2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::*;
    #[test]
    fn test1() {
        assert_eq!(part1(&read("day09.txt")), 2789104029);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(&read("day09.txt")), 32869);
    }
}
