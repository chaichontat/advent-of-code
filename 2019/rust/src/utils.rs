use std::fmt::Debug;
use std::fs;
use std::hash::Hash;

use ahash::{AHashMap, AHashSet};
use ascii::{AsciiStr, AsciiString};
use itertools::Itertools;
use num::Signed;
use num_complex::Complex;
use regex::Regex;
use strum_macros::EnumString;

pub type Coord = Complex<isize>;
pub type Ans = Option<(usize, usize)>;

pub fn read(path: &str) -> Vec<String> {
    let p = format!("../data/{}", path);
    let res = fs::read_to_string(p).unwrap();
    let mut vec: Vec<String> = res.split('\n').map(str::to_string).collect();
    vec.truncate(vec.len() - 1);
    vec
}

pub fn read_ascii(path: &str) -> Vec<AsciiString> {
    let p = format!("../data/{}", path);
    let res = AsciiString::from_ascii(fs::read(p).unwrap()).unwrap();
    let mut vec = res
        .split(ascii::AsciiChar::LineFeed)
        .map(AsciiStr::to_owned)
        .collect_vec();
    vec.truncate(vec.len() - 1);
    vec
}

#[macro_export]
macro_rules! ahashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = AHashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

#[macro_export]
macro_rules! ahashset {
    ($( $key: expr ),*) => {{
         let mut set = AHashSet::new();
         $( set.insert($key); )*
         set
    }}
}

pub fn str_idx(s: &str, i: usize) -> char {
    s.chars().nth(i).unwrap()
}

pub fn printt(x: &impl Debug) {
    println!("{:?}", x);
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
        Dir::from(Complex::<i8>::from(*self) * Complex::from(turn))
    }
}

impl<T: Signed> From<Complex<T>> for Dir {
    fn from(x: Complex<T>) -> Self {
        if x == Complex::new(T::zero(), T::one()) {
            Self::U
        } else if x == Complex::new(T::zero(), -T::one()) {
            Self::D
        } else if x == Complex::new(-T::one(), T::zero()) {
            Self::L
        } else if x == Complex::new(T::one(), T::zero()) {
            Self::R
        } else {
            unreachable!("Invalid Dir");
        }
    }
}

impl<T: Signed> From<Dir> for Complex<T> {
    fn from(x: Dir) -> Self {
        match x {
            Dir::U => Complex::new(T::zero(), T::one()),
            Dir::D => Complex::new(T::zero(), -T::one()),
            Dir::L => Complex::new(-T::one(), T::zero()),
            Dir::R => Complex::new(T::one(), T::zero()),
        }
    }
}

#[derive(Debug, PartialEq, EnumString, Clone, Copy)]
pub enum Turn {
    L,
    R,
    N,
}

impl<T: Signed> From<Turn> for Complex<T> {
    fn from(x: Turn) -> Self {
        match x {
            Turn::L => Complex::new(T::zero(), T::one()),
            Turn::R => Complex::new(T::zero(), -T::one()),
            Turn::N => Complex::new(T::one(), T::zero()),
        }
    }
}
