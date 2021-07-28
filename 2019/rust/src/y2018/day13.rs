use num::Complex;

use crate::utils::{Dir, Turn};

const DIM: usize = 150;
type Pos = Complex<i16>;
type Map = [[u8; DIM]; DIM];

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cart {
    pos:  Pos,
    dir:  Dir,
    turn: Turn,
    dead: bool,
}

#[derive(Debug, Clone)]
pub struct Board {
    map:   Map,
    carts: Vec<Cart>,
    part1: Option<Pos>,
    part2: Option<Pos>,
}

pub fn parse(raw: &str) -> Board {
    let mut carts = Vec::new();
    let mut map = [[0; DIM]; DIM];

    for (y, line) in raw.split('\n').enumerate() {
        let line = line.as_bytes();
        map[y][..line.len()].copy_from_slice(line);

        for (x, &c) in line.iter().enumerate() {
            let dir = match c {
                b'>' => Dir::R,
                b'v' => Dir::D,
                b'^' => Dir::U,
                b'<' => Dir::L,
                _ => continue,
            };
            carts.push(Cart {
                pos: Complex::new(x as i16, -(y as i16)),
                dir,
                turn: Turn::L,
                dead: false,
            });
            map[y][x] |= 0x80; // Marker that ∃cart here. All char in here < 128.
        }
    }

    Board {
        map,
        carts,
        part1: None,
        part2: None,
    }
}

impl Board {
    fn step(&mut self) {
        self.carts
            .sort_unstable_by_key(|c| (c.dead, -c.pos.im, c.pos.re));

        // Problem: Since we're screening for death at the beginning of each inner loop, it's
        // possible for one cart to get cleared in one step and another to get cleared in the next.
        // At the end, we would be left with two carts, having no idea which one is the zombie.
        // In that case, we need to check right after the death screen and before stepping to prevent
        // overstepping. This should be the only scenario where we have two carts in the vector.

        while self.carts.last().unwrap().dead {
            self.carts.pop();
        }
        let cart_len = self.carts.len();
        if cart_len == 1 {
            self.part2 = Some(self.carts[0].pos);
            return;
        }

        for cart in self.carts.iter_mut() {
            let curr;
            unsafe {
                curr = self
                    .map
                    .get_unchecked_mut(-cart.pos.im as usize)
                    .get_unchecked_mut(cart.pos.re as usize);
            }

            if *curr & 0x80 == 0 {
                cart.dead = true; // If cart is here but map says otherwise => dead.
                continue;
            }

            if cart_len == 2 && !cart.dead {
                self.part2 = Some(cart.pos);
                return;
            }
            *curr &= 0x7f; // Remove cart from old position.

            cart.pos += Complex::from(cart.dir);
            let next;
            unsafe {
                next = self
                    .map
                    .get_unchecked_mut(-cart.pos.im as usize)
                    .get_unchecked_mut(cart.pos.re as usize);
            }

            if *next & 0x80 != 0 {
                // Overlap and crash.
                if self.part1.is_none() {
                    self.part1 = Some(cart.pos);
                }
                cart.dead = true;
                *next &= 0x7f; // Mark site as crashed to remove another cart.
                continue;
            }

            match next {
                b'/' => {
                    cart.dir = match cart.dir {
                        Dir::R => Dir::U,
                        Dir::L => Dir::D,
                        Dir::U => Dir::R,
                        Dir::D => Dir::L,
                    }
                }
                b'\\' => {
                    cart.dir = match cart.dir {
                        Dir::R => Dir::D,
                        Dir::L => Dir::U,
                        Dir::U => Dir::L,
                        Dir::D => Dir::R,
                    }
                }
                b'+' => {
                    cart.dir = cart.dir.turn(cart.turn);
                    cart.turn = match cart.turn {
                        Turn::L => Turn::N,
                        Turn::N => Turn::R,
                        Turn::R => Turn::L,
                    }
                }
                _ => (),
            }
            *next |= 0x80;
        }
    }
}

pub fn combi(board: &Board) -> ((i16, i16), (i16, i16)) {
    let mut board = board.to_owned();
    while board.part2.is_none() {
        board.step();
    }
    (
        (board.part1.unwrap().re, -board.part1.unwrap().im),
        (board.part2.unwrap().re, -board.part2.unwrap().im),
    )
}

mod tests {
    use super::{combi, parse};
    use crate::utils::read;

    #[test]
    fn test_combi() {
        assert_eq!(
            combi(&parse(&read(2018, "day13.txt"))),
            ((136, 36), (53, 111))
        );
    }
}
