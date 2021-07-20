use super::intcode::*;
type Parsed = isize;

pub fn parse(raw: &str) -> Vec<Parsed> {
    parse_ic(raw)
}

fn run_ic(parsed: &[Parsed], input: isize) -> usize {
    let mut ic = IntCode::from(parsed);
    ic.push(input);
    ic.run();
    ic.pop().unwrap() as usize
}

pub fn part1(parsed: &[Parsed]) -> usize {
    run_ic(parsed, 1)
}

pub fn part2(parsed: &[Parsed]) -> usize {
    run_ic(parsed, 2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::*;
    #[test]
    fn test1() {
        assert_eq!(part1(&parse(&read(2019, "day09.txt"))), 2789104029);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(&parse(&read(2019, "day09.txt"))), 32869);
    }
}
