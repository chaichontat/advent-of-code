use std::collections::VecDeque;
use std::usize;

use ahash::AHashSet;
use itertools::Itertools;
use num_complex::Complex;

use super::intcode::*;
use super::utils::*;

type Board = AHashSet<Coord>;

#[derive(Clone, Copy)]
struct Pos {
    loc: Complex<isize>,
    dir: Dir,
}

impl Pos {
    fn turn_step(&self, t: Turn) -> Self {
        Pos {
            loc: self.loc + Complex::from(self.dir.turn(t)),
            dir: self.dir.turn(t),
        }
    }
}

fn get_data(ic: &mut IntCode) -> (Board, Pos) {
    let mut board = AHashSet::new();
    let mut curr = Complex::new(0, 0);

    let mut dir: Option<Dir> = None;
    let mut loc: Option<Coord> = None;

    while !ic.done {
        ic.run_pause();
        let out = ic.output.pop_front();
        match out {
            Some(10) => {} // \n
            Some(35) => {
                board.insert(curr); // #
            }
            Some(46) => {} // .
            Some(94) => {
                loc = Some(curr); // ^
                dir = Some(Dir::D); // Swapped due to coordinate system.
            }
            Some(118) => {
                loc = Some(curr); // v
                dir = Some(Dir::U);
            }
            Some(60) => {
                loc = Some(curr); // <
                dir = Some(Dir::L);
            }
            Some(62) => {
                loc = Some(curr); // >
                dir = Some(Dir::R);
            }
            Some(_) => unreachable!("Invalid IntCode Output."),
            None => {} // Done.
        }

        if out == Some(10) {
            curr = Complex::new(0, curr.im + 1);
        } else {
            curr += Complex::new(1, 0);
        }
    }
    (board, Pos {
        dir: dir.unwrap(),
        loc: loc.unwrap(),
    })
}

pub fn part1(raw: &[String]) -> usize {
    let mut ic = IntCode::from(&raw[0]);
    let (board, _) = get_data(&mut ic);
    let adj = [Dir::U, Dir::D, Dir::L, Dir::R];
    board.iter().fold(0, |sum, x| {
        if adj.iter().all(|a| board.contains(&(x + Complex::from(*a)))) {
            sum + x.re * x.im
        } else {
            sum
        }
    }) as usize
}

fn path_finder(p: &Pos, board: Board) -> Vec<String> {
    let mut out = Vec::with_capacity(100);
    let mut acc = 1;
    let mut pos = *p;
    let mut prev_dir;

    if board.contains(&pos.turn_step(Turn::L).loc) {
        prev_dir = 'R'; // Swapped due to coordinate system.
        pos = pos.turn_step(Turn::L);
    } else if board.contains(&pos.turn_step(Turn::R).loc) {
        prev_dir = 'L';
        pos = pos.turn_step(Turn::R);
    } else {
        unreachable!();
    }

    loop {
        if board.contains(&pos.turn_step(Turn::N).loc) {
            acc += 1;
            pos = pos.turn_step(Turn::N);
        } else if board.contains(&pos.turn_step(Turn::L).loc) {
            out.push(format!("{},{},", prev_dir, acc));
            acc = 1;
            prev_dir = 'R';
            pos = pos.turn_step(Turn::L);
        } else if board.contains(&pos.turn_step(Turn::R).loc) {
            out.push(format!("{},{},", prev_dir, acc));
            acc = 1;
            prev_dir = 'L';
            pos = pos.turn_step(Turn::R);
        } else {
            out.push(format!("{},{},", prev_dir, acc));
            return out;
        }
    }
}

fn get_subseq(v: &[String], n: usize) -> Vec<String> {
    ((n - 2)..=n)
        .rev()
        .flat_map(|sl| {
            (0..v.len() - sl)
                .map(|i| (&v[i..i + sl]).to_vec().concat())
                .unique()
                .collect_vec()
        })
        .collect_vec()
}

fn space_saved(full: &str, cand: &str) -> usize { full.matches(cand).count() * cand.len() }

fn subs_test(full: &String, cand: &Vec<&String>) -> Option<String> {
    let keys = ["A,", "B,", "C,"];
    let res = cand
        .iter()
        .enumerate()
        .fold(full.clone(), |pass, (i, &this)| pass.replace(this, keys[i]));

    if !(res.contains("L") || res.contains("R")) {
        Some(res)
    } else {
        None
    }
}

fn compress(cmds: &[String]) -> Option<Vec<isize>> {
    let full = cmds.concat();
    let subs = get_subseq(cmds, 5);
    let subs_len = subs
        .iter()
        .map(|x| (x, space_saved(&full, &x)))
        .collect_vec();

    let test: Vec<Vec<&String>> = subs_len
        .iter()
        .combinations(3)
        .filter(|x| x.iter().map(|y| y.1).sum::<usize>() == full.len())
        .map(|x| x.iter().map(|y| y.0).collect_vec())
        .collect_vec();

    for x in test.iter() {
        if let Some(res) = subs_test(&full, &x) {
            let mut res = res.chars();
            res.next_back();
            let mut fuck = vec![res.collect::<String>()];
            fuck.append(
                &mut x
                    .iter()
                    .map(|&y| (&y[..y.len() - 1]).to_owned())
                    .collect_vec(),
            );

            let fin = fuck.join("\n");
            let g = fin.chars();
            // g.next_back();
            let u = g.map(|x| x as isize).collect_vec();
            return Some(u);
        }
    }
    None
}

// "In general, the scaffold forms a path, but it sometimes loops back onto itself."
// Just go straight as much as possible.
pub fn part2(raw: &[String]) -> usize {
    let mut ic = IntCode::from(&raw[0]);
    let (board, pos) = get_data(&mut ic);
    let seq = path_finder(&pos, board);
    let ans = compress(&seq).unwrap();
    let mut ic = IntCode::from(&raw[0]);
    ic.mem[0] = 2;
    ic.input.append(&mut VecDeque::from(ans));
    ic.input.push_back(10);
    ic.input.push_back('n' as isize);
    ic.input.push_back(10);
    ic.run();

    for x in ic.output {
        if x > 255 {
            return x as usize;
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(part1(&read("day17.txt")), 5056);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(&read("day17.txt")), 942367);
    }
}
