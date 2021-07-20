use std::collections::VecDeque;

use super::intcode::*;

#[allow(dead_code)]
#[derive(Clone, Copy)]
enum RReg {
    A = 65,
    B = 66,
    C = 67,
    D = 68,
    E = 69,
    F = 70,
    G = 71,
    H = 72,
    I = 73,
    T = 84,
    J = 74,
}

#[derive(Clone, Copy)]
enum WReg {
    T = 84,
    J = 74,
}

const END: isize = 10;
const SP: isize = 32;

const RUN: [isize; 4] = [82, 85, 78, END];
const WALK: [isize; 5] = [87, 65, 76, 75, END];

enum Ins {
    And(RReg, WReg),
    Or(RReg, WReg),
    Not(RReg, WReg),
}

impl Ins {
    fn to_vec(&self) -> VecDeque<isize> {
        let v = match self {
            Ins::And(r, w) => vec![65, 78, 68, SP, *r as isize, SP, *w as isize, END],
            Ins::Or(r, w) => vec![79, 82, SP, *r as isize, SP, *w as isize, END],
            Ins::Not(r, w) => vec![78, 79, 84, SP, *r as isize, SP, *w as isize, END],
        };
        VecDeque::from(v)
    }
}

fn send_ins(ic: &mut IntCode, cmds: &[Ins], fin: &[isize]) {
    cmds.iter().for_each(|x| ic.input.append(&mut x.to_vec()));
    fin.iter().for_each(|x| ic.input.push_back(*x));
}

fn run_ic(raw: &[String], cmds: &[Ins], fin: &[isize]) -> usize {
    let mut ic = IntCode::from(&raw[0]);
    send_ins(&mut ic, cmds, fin);
    ic.run();
    ic.output.pop_back().unwrap() as usize
}

pub fn part1(raw: &[String]) -> usize {
    // (~A ∨ ~B ∨ ~C) ∧ D
    let cmds = [
        Ins::Or(RReg::A, WReg::J),
        Ins::And(RReg::B, WReg::J),
        Ins::And(RReg::C, WReg::J),
        Ins::Not(RReg::J, WReg::J),
        Ins::And(RReg::D, WReg::J),
    ];
    run_ic(raw, &cmds, &WALK)
}

pub fn part2(raw: &[String]) -> usize {
    // (~A ∨ ~B ∨ ~C) ∧ D ∧ (E ∨ H)
    let cmds = [
        // Same as part 1
        Ins::Or(RReg::A, WReg::J),
        Ins::And(RReg::B, WReg::J),
        Ins::And(RReg::C, WReg::J),
        Ins::Not(RReg::J, WReg::J),
        Ins::And(RReg::D, WReg::J),
        //
        Ins::Or(RReg::E, WReg::T),
        Ins::Or(RReg::H, WReg::T),
        Ins::And(RReg::T, WReg::J),
    ];
    run_ic(raw, &cmds, &RUN)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::*;
    #[test]
    fn test1() {
        assert_eq!(part1(&read("day21.txt")), 19357507);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(&read("day21.txt")), 1142830249);
    }
}
