extern crate num_integer;
use std::{fmt::Debug, str::FromStr};

use num::Signed;

// https://github.com/simon-andrews/rust-modinverse
#[allow(clippy::many_single_char_names)]
fn egcd<T: Copy + Signed>(a: T, b: T) -> (T, T, T) {
    if a == T::zero() {
        (b, T::zero(), T::one())
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

pub fn mod_inv<T: Copy + Signed>(a: T, m: T) -> T {
    let (g, x, _) = egcd(a, m);
    if g != T::one() {
        panic!();
    } else {
        (x % m + m) % m
    }
}

#[allow(dead_code)]
enum Op<T: Signed> {
    Id,
    Stack,
    Cut(T),
    Deal(T),
}

impl<T: Signed + Copy> Op<T> {
    fn to_mod(&self, base: T) -> Mod<T> {
        let (mul, add) = match self {
            Op::Id => (T::one(), T::zero()),
            Op::Stack => (-T::one(), -T::one()),
            Op::Cut(n) => (T::one(), *n),
            Op::Deal(n) => (mod_inv(*n, base), T::zero()),
        };
        Mod { mul, add, base }
    }
}

#[derive(Debug, Clone, Copy)]
struct Mod<T: Signed + Copy> {
    mul:  T,
    add:  T,
    base: T,
}

impl<T: Signed + Copy + Debug> Mod<T> {
    fn compose(&self, other: &Self) -> Self {
        debug_assert_eq!(self.base, other.base);
        let add = (self.add + self.mul * other.add) % self.base;
        let mul = self.mul * other.mul % self.base;
        Mod { mul, add, base: self.base }
    }

    fn curr_pos_from_ori_pos(&self, tar: T) -> T {
        // a⁻¹(target - b) ≡ x mod N  if N prime.
        (mod_inv(self.mul, self.base) * (tar - self.add)) % self.base
    }

    fn value_of_curr_pos(&self, x: T) -> T {
        (x * self.mul + self.add) % self.base
    }
}

fn parse<T>(raw: &[String]) -> Vec<Op<T>>
where
    T: Signed + FromStr,
    <T as FromStr>::Err: Debug,
{
    let mut vec = Vec::new();
    for line in raw.iter() {
        if line.starts_with("cut") {
            let n = line.split(' ').nth(1).unwrap().parse::<T>().unwrap();
            vec.push(Op::Cut(n));
        } else if line.contains("increment") {
            let n = line.split(' ').last().unwrap().parse::<T>().unwrap();
            vec.push(Op::Deal(n));
        } else if line.contains("stack") {
            vec.push(Op::Stack);
        } else {
            unreachable!();
        }
    }
    vec
}

// ℤₚ is a field when p is prime, which by the Bézout's identity,
// implies the existence and uniqueness of the multiplicative modular inverse.

pub fn part1(raw: &[String]) -> usize {
    let ops = parse::<i32>(raw);
    let base = 10007_i32;
    let shuffle =
        ops.iter().fold(Mod { mul: 1, add: 0, base }, |prog, x| prog.compose(&x.to_mod(base)));
    shuffle.curr_pos_from_ori_pos(2019) as usize
}

fn pow(mut m: Mod<i128>, mut k: i128) -> Mod<i128> {
    // Binary exponentiation.
    let mut g = Mod { mul: 1, add: 0, base: m.base };
    while k > 0 {
        // Check odd.
        if k & 1 == 1 {
            g = g.compose(&m);
        }
        k >>= 1;
        m = m.compose(&m);
    }
    g
}

pub fn part2(raw: &[String]) -> usize {
    let ops = parse::<i128>(raw);
    let base = 119315717514047_i128;
    let shuffle =
        ops.iter().fold(Mod { mul: 1, add: 0, base }, |prog, x| prog.compose(&x.to_mod(base)));

    let res = pow(shuffle, 101741582076661);
    res.value_of_curr_pos(2020) as usize
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::*;
    #[test]
    fn test1() {
        assert_eq!(part1(&read("day22.txt")), 8775);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(&read("day22.txt")), 47141544607176);
    }
}
