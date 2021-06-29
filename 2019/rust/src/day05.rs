use super::intcode::*;

fn run_ic(raw: &[String], code: isize) -> IntCode {
    let mut ic = IntCode::from(&raw[0]);
    ic.input.push_front(code);
    ic.run();
    ic
}

pub fn part1(raw: &[String]) -> usize { run_ic(raw, 1).output.pop_back().unwrap() as usize }

pub fn part2(raw: &[String]) -> usize { run_ic(raw, 5).output.pop_back().unwrap() as usize }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::*;
    #[test]
    fn test1() {
        assert_eq!(part1(&read("day05.txt")), 15426686);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(&read("day05.txt")), 11430197);
    }
}
