use itertools::Itertools;
use nohash_hasher::IntSet;

use crate::intcode::*;

fn gen_ic(raw: &String) -> Vec<IntCode> {
    let ori = IntCode::from(raw);
    (0..50)
        .map(|i| {
            let mut ic = ori.clone();
            ic.push(i);
            ic.run_wait_input();
            ic
        })
        .collect_vec()
}

fn run_ic(ics: &mut Vec<IntCode>, nat: &mut (isize, isize)) {
    for i in 0..ics.len() {
        let ic = &mut ics[i];
        if ic.input.is_empty() {
            ic.push(-1);
        }
        ic.run_wait_input();
        if let Some(ins) = ic.pop() {
            if ins == 255 {
                *nat = (ic.pop().unwrap(), ic.pop().unwrap());
            } else if (0..50).contains(&ins) {
                let packet = (ic.pop().unwrap(), ic.pop().unwrap());
                ics[ins as usize].push(packet.0);
                ics[ins as usize].push(packet.1);
            } else {
                unreachable!();
            }
        }
    }
}

pub fn part1(raw: &[String]) -> usize {
    let mut ics = gen_ic(&raw[0]);
    let mut nat = (0, 0);
    while nat == (0, 0) {
        run_ic(&mut ics, &mut nat);
    }
    nat.1 as usize
}

pub fn part2(raw: &[String]) -> usize {
    let mut ics = gen_ic(&raw[0]);
    let mut nat = (0, 0);
    let mut delivered = IntSet::default();
    loop {
        run_ic(&mut ics, &mut nat);
        if ics
            .iter()
            .all(|ic| ic.output.is_empty() && ic.input.is_empty())
        {
            ics[0].push(nat.0);
            ics[0].push(nat.1);
            if !delivered.insert(nat.1) {
                return nat.1 as usize;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::*;
    #[test]
    fn test1() {
        assert_eq!(part1(&read("day23.txt")), 19937);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(&read("day23.txt")), 13758);
    }
}
