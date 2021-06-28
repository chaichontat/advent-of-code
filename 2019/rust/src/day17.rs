use super::intcode::*;
use super::utils::*;
use num_complex::Complex;
use std::collections::HashSet;

struct Board {
    board: HashSet<Complex<isize>>,
    dir: Dir,
    loc: Complex<isize>,
}

fn get_data(raw: &[String]) -> Board {
    let mut ic = IntCode::from(&raw[0]);
    let mut board = HashSet::new();
    let mut curr = Complex::new(0, 0);

    let mut dir: Option<Dir> = None;
    let mut loc: Option<Complex<isize>> = None;

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
                dir = Some(Dir::U);
            }
            Some(118) => {
                loc = Some(curr); // v
                dir = Some(Dir::D);
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
    Board {
        board,
        dir: dir.unwrap(),
        loc: loc.unwrap(),
    }
}

pub fn part1(raw: &[String]) -> usize {
    let board = get_data(raw);
    let adj = [Dir::U, Dir::D, Dir::L, Dir::R];
    board.board.iter().fold(0, |sum, x| {
        if adj
            .iter()
            .all(|a| board.board.contains(&(x + Complex::from(*a))))
        {
            sum + x.re * x.im
        } else {
            sum
        }
    }) as usize
}

pub fn part2(raw: &[String]) -> usize {
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
        assert_eq!(part2(&read("day17.txt")), 0);
    }
}
