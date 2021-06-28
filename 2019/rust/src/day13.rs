use super::intcode::*;
use ahash::AHashSet;
use num_complex::Complex;
use std::cmp::Ordering;

#[derive(Debug, FromPrimitive, PartialEq)]
enum Tile {
    Empty = 0,
    Wall = 1,
    Block = 2,
    HorzPad = 3,
    Ball = 4,
}

struct Game {
    blocks: AHashSet<Complex<isize>>,
    pad: Complex<isize>,
    ball: Complex<isize>,
    score: isize,
}

fn init(raw: &[String]) -> (IntCode, Game) {
    let ic = IntCode::from(&raw[0]);
    let game = Game {
        blocks: AHashSet::with_capacity(200),
        pad: Complex::new(0, 0),
        ball: Complex::new(0, 0),
        score: 0,
    };
    (ic, game)
}

fn parse_game(ic: &mut IntCode, game: &mut Game) {
    ic.run_wait_input();
    while !ic.output.is_empty() {
        let pos = Complex::new(
            ic.output.pop_front().unwrap(),
            ic.output.pop_front().unwrap(),
        );
        if pos == Complex::new(-1, 0) {
            game.score = ic.output.pop_front().unwrap();
        } else {
            let tile = ic
                .output
                .pop_front()
                .and_then(num::FromPrimitive::from_isize)
                .unwrap();
            match tile {
                Tile::Block => {
                    game.blocks.insert(pos);
                }
                Tile::Empty => {
                    game.blocks.remove(&pos);
                }
                Tile::HorzPad => game.pad = pos,
                Tile::Ball => game.ball = pos,
                _ => (),
            }
        }
    }
}

pub fn part1(raw: &[String]) -> usize {
    let (mut ic, mut game) = init(raw);
    parse_game(&mut ic, &mut game);
    game.blocks.len()
}

pub fn part2(raw: &[String]) -> usize {
    let (mut ic, mut game) = init(raw);
    ic.mem[0] = 2;

    parse_game(&mut ic, &mut game);
    while game.blocks.len() > 0 {
        ic.input.push_back(match game.ball.re.cmp(&game.pad.re) {
            Ordering::Equal => 0,
            Ordering::Greater => 1,
            Ordering::Less => -1,
        });
        parse_game(&mut ic, &mut game);
    }
    game.score as usize
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::*;
    #[test]
    fn test1() {
        assert_eq!(part1(&read("day13.txt")), 180);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(&read("day13.txt")), 8777);
    }
}
