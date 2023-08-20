use itertools::Itertools;

use super::intcode::*;
type Parsed = isize;

pub fn parse(raw: &str) -> Vec<Parsed> {
    parse_ic(raw)
}

fn run_ic(parsed: &[isize], code: [isize; 2]) -> isize {
    let mut ic = IntCode::from(parsed);
    ic.input.push_back(code[0]);
    ic.input.push_back(code[1]);
    ic.run();
    ic.output.pop_front().unwrap()
}

pub fn part1(parsed: &[isize]) -> usize {
    let mut res = [0_isize; 4_usize.pow(5)];
    for (i, phase) in (0..5).permutations(5).enumerate() {
        res[i] = phase.iter().fold(0, |out, ph| run_ic(parsed, [*ph, out]));
    }
    *res.iter().max().unwrap() as usize
}

fn gen_ic(parsed: &[isize], phase: isize) -> IntCode {
    let mut ic = IntCode::from(parsed);
    ic.input.push_back(phase);
    ic
}

fn run_loop(parsed: &[isize], phases: &[isize]) -> isize {
    let mut ics = phases
        .iter()
        .map(|ph| gen_ic(parsed, *ph))
        .collect::<Vec<IntCode>>();

    let mut out = 0;
    let mut i = 0;
    while !ics[i].done {
        ics[i].input.push_back(out);
        ics[i].run_pause();
        out = ics[i].output.pop_front().unwrap_or(out);
        i = (i + 1) % 5;
    }
    out
}

pub fn part2(parsed: &[isize]) -> usize {
    let mut res = [0_isize; 4_usize.pow(5)];
    for (i, phases) in (5..=9).permutations(5).enumerate() {
        res[i] = run_loop(parsed, &phases);
    }
    *res.iter().max().unwrap() as usize
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::*;
    #[test]
    fn test1() {
        assert_eq!(part1(&parse(&read(2019, "day07.txt"))), 77500);
    }

    #[test]
    fn test2() {
        // assert_eq!(
        //     run_loop(&[String::from("3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5")], &[9,8,7,6,5]),
        //     139629729
        // );
        assert_eq!(part2(&parse(&read(2019, "day07.txt"))), 22476942);
    }
}
