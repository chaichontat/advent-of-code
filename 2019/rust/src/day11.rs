use super::intcode::*;
use super::utils::*;
use ahash::AHashMap;
use num_complex::Complex;

#[derive(Debug, Copy, Clone, FromPrimitive)]
enum Color {
    Black = 0,
    White = 1,
}

struct Board {
    board: AHashMap<Complex<isize>, Color>,
    pos: Complex<isize>,
    dir: Dir,
}

impl Board {
    pub fn get(&mut self) -> Color {
        *self.board.get(&self.pos).unwrap_or(&Color::Black)
    }
    pub fn set(&mut self, c: Color) {
        self.board.insert(self.pos, c);
    }
    pub fn step(&mut self, d: Turn) {
        self.dir.turn(d);
        self.pos += Complex::from(self.dir);
    }
}

impl Default for Board {
    fn default() -> Self {
        Board {
            board: AHashMap::new(),
            pos: Complex::new(0, 0),
            dir: Dir::U,
        }
    }
}

fn run(ic: &mut IntCode, board: &mut Board) {
    loop {
        ic.input.push_back(board.get() as isize);
        ic.run_pause();
        if ic.done {
            break;
        }
        board.set(
            ic.output
                .pop_front()
                .and_then(num::FromPrimitive::from_isize)
                .unwrap(),
        ); // Color to paint.
        ic.run_pause();
        board.step(match ic.output.pop_front() {
            Some(0) => Turn::L,
            Some(1) => Turn::R,
            _ => unreachable!(),
        })
    }
}

pub fn part1(raw: &[String]) -> usize {
    let mut ic = IntCode::from(&raw[0]);
    let mut board = Board {
        ..Default::default()
    };
    run(&mut ic, &mut board);
    board.board.len()
}

pub fn part2(raw: &[String]) -> usize {
    let mut ic = IntCode::from(&raw[0]);
    let mut board = Board {
        ..Default::default()
    };
    board.board.insert(Complex::new(0, 0), Color::White);
    run(&mut ic, &mut board);
    // Get boundaries.
    let (xmin, xmax, ymin, ymax) = board.board.keys().fold((0, 0, 0, 0), |mut sum, x| {
        if x.re < sum.0 {
            sum.0 = x.re
        } else if x.re > sum.1 {
            sum.1 = x.re
        }
        if x.im < sum.2 {
            sum.2 = x.im
        } else if x.im > sum.3 {
            sum.3 = x.im
        }
        sum
    });

    for y in (0..=(ymax - ymin)).rev() {
        for x in 0..=(xmax - xmin) {
            match board.board.get(&Complex::new(x + xmin, y + ymin)) {
                Some(Color::White) => 2, //print!("#"),
                _ => 1,                  //print!(" "),
            };
        }
        // println!();
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(part1(&read("day11.txt")), 2082);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(&read("day11.txt")), 0);
    }
}
