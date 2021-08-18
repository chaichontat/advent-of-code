use super::intcode::*;

type Parsed = isize;

pub fn parse(raw: &str) -> Vec<Parsed> {
    parse_ic(raw)
}

fn run_ic(parsed: &[Parsed], code: isize) -> IntCode {
    let mut ic = IntCode::from(parsed);
    ic.input.push_front(code);
    ic.run();
    ic
}

pub fn part1(parsed: &[Parsed]) -> usize {
    run_ic(parsed, 1).output.pop_back().unwrap() as usize
}

pub fn part2(parsed: &[Parsed]) -> usize {
    run_ic(parsed, 5).output.pop_back().unwrap() as usize
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::*;
    #[test]
    fn test1() {
        assert_eq!(part1(&parse(&read(2019, "day05.txt"))), 15426686);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(&parse(&read(2019, "day05.txt"))), 11430197);
    }
}
