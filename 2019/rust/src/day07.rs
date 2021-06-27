use super::intcode::*;
use super::utils::*;
use itertools::Itertools;

fn run_ic(raw: &[String], code: [isize; 2]) -> isize {
    let mem: Vec<isize> = raw[0]
        .split(',')
        .map(|x| x.parse::<isize>().unwrap())
        .collect();
    let mut ic = IntCode::from(mem.as_slice());
    ic.input.push_back(code[0]);
    ic.input.push_back(code[1]);
    ic.run();
    ic.output.pop_front().unwrap()
}

pub fn part1(raw: &[String]) -> usize {
    let mut res = [0_isize; 4_usize.pow(5)];
    for (i, phase) in (0..5).permutations(5).enumerate() {
        res[i] = phase.iter().fold(0, |out, ph| run_ic(raw, [*ph, out]));
    }
    *res.iter().max().unwrap() as usize
}

fn gen_ic(raw: &[String], phase: isize) -> IntCode {
    let mem: Vec<isize> = raw[0]
        .split(',')
        .map(|x| x.parse::<isize>().unwrap())
        .collect();
    let mut ic = IntCode::from(mem.as_slice());
    ic.input.push_back(phase);
    ic
}

fn run_loop(raw: &[String], phases: &[isize]) -> isize {
    let mut ics = phases
        .iter()
        .map(|ph| gen_ic(raw, *ph))
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

pub fn part2(raw: &[String]) -> usize {
    let mut res = [0_isize; 4_usize.pow(5)];
    for (i, phases) in (5..=9).permutations(5).enumerate() {
        res[i] = run_loop(raw, &phases);
    }
    *res.iter().max().unwrap() as usize
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(part1(&read("day07.txt")), 77500);
    }

    #[test]
    fn test2() {
        // assert_eq!(
        //     run_loop(&[String::from("3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5")], &[9,8,7,6,5]),
        //     139629729
        // );
        assert_eq!(part2(&read("day07.txt")), 22476942);
    }
}
