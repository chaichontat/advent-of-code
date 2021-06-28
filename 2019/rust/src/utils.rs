use ahash::{AHashMap, AHashSet};
use num_complex::Complex;
use regex::Regex;
use std::fmt::Debug;
use std::fs;
use std::hash::Hash;
use strum_macros::EnumString;

pub type Coord = Complex<isize>;

pub fn read(path: &str) -> Vec<String> {
    let p = format!("../data/{}", path);
    let res = fs::read_to_string(p).unwrap();
    let mut vec: Vec<String> = res.split('\n').map(str::to_string).collect();
    vec.truncate(vec.len() - 1);
    vec
}

pub fn str_idx(s: &str, i: usize) -> char {
    s.chars().nth(i).unwrap()
}

pub fn printt(x: &impl Debug) {
    println!("{:#?}", x);
}

pub fn gen_re(r: &str) -> Regex {
    Regex::new(r).unwrap()
}

pub fn int(x: &str) -> usize {
    x.parse::<usize>().unwrap()
}

pub trait Set {
    type Item;
    fn keys_to(&self) -> Self::Item;
}

impl<T: Clone + Eq + Hash, R> Set for AHashMap<T, R> {
    type Item = AHashSet<T>;
    fn keys_to(&self) -> Self::Item {
        self.keys().cloned().collect::<Self::Item>()
    }
}

#[derive(Debug, PartialEq, EnumString, Clone, Copy)]
pub enum Dir {
    U,
    D,
    L,
    R,
}

impl Dir {
    // pub fn turn(&mut self, turn: Turn) {
    //     std::mem::swap(
    //         self,
    //         &mut Dir::from(Complex::from(*self) * Complex::from(turn)),
    //     );
    // }

    pub fn turn(&self, turn: Turn) -> Self {
        Dir::from(Complex::from(*self) * Complex::from(turn))
    }
}

impl From<Complex<isize>> for Dir {
    fn from(x: Complex<isize>) -> Self {
        if x == Complex::new(0, 1) {
            Self::U
        } else if x == Complex::new(0, -1) {
            Self::D
        } else if x == Complex::new(-1, 0) {
            Self::L
        } else if x == Complex::new(1, 0) {
            Self::R
        } else {
            unreachable!("Invalid Dir");
        }
    }
}

impl From<Dir> for Complex<isize> {
    fn from(x: Dir) -> Self {
        match x {
            Dir::U => Complex::new(0, 1),
            Dir::D => Complex::new(0, -1),
            Dir::L => Complex::new(-1, 0),
            Dir::R => Complex::new(1, 0),
        }
    }
}

#[derive(Debug, PartialEq, EnumString, Clone, Copy)]
pub enum Turn {
    L,
    R,
}

impl From<Turn> for Complex<isize> {
    fn from(x: Turn) -> Self {
        match x {
            Turn::L => Complex::new(0, 1),
            Turn::R => Complex::new(0, -1),
        }
    }
}
