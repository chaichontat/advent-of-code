use std::fmt::Debug;
use std::hash::Hash;
use std::{error, fs};

use ascii::{AsciiStr, AsciiString};
use colored::*;
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use regex::Regex;

pub type Ans = Option<(usize, usize)>;
pub type GenericResult<T> = std::result::Result<T, Box<dyn error::Error + Send + Sync + 'static>>;

pub fn read(year: u16, path: &str) -> String {
    let p = format!("./data/{}/{}", year, path);
    let mut res = fs::read_to_string(p).unwrap();
    res.pop();
    res
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
macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

#[macro_export]
macro_rules! hashset {
    ($( $key: expr ),*) => {{
         let mut set = HashSet::new();
         $( set.insert($key); )*
         set
    }}
}

/// https://stackoverflow.com/a/45792463
#[macro_export]
macro_rules! compose {
    ( $last:expr ) => { $last };
    ( $head:expr, $($tail:expr), +) => {
        compose_two($head, compose!($($tail),+))
    };
}

pub fn compose_two<A, B, C, G, F>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    move |x| g(f(x))
}

pub fn str_idx(s: &str, i: usize) -> char {
    s.chars().nth(i).unwrap()
}

pub fn printt(x: &impl Debug) {
    println!("{:?}", x);
}

pub fn printarr(x: &[impl Debug]) {
    print!("[");
    for i in x {
        print!("{:3?},", i);
    }
    println!("]");
}

pub fn printc(x: &impl Debug, c: Color) {
    println!("{}", ColoredString::from(&format!("{:?}", x)[..]).color(c));
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

impl<T: Clone + Eq + Hash, R> Set for HashMap<T, R> {
    type Item = HashSet<T>;

    fn keys_to(&self) -> Self::Item {
        self.keys().cloned().collect::<Self::Item>()
    }
}

pub trait ModAdd {
    fn mod_add(self, x: Self, m: Self) -> Self;
    fn mod_down(self, m: Self) -> Self;
}

macro_rules! impl_mod_add {
    ( $( $t:ident)* ) => {
        $(
            impl ModAdd for $t {
                fn mod_add(self, x: Self, m: Self) -> Self {
                    (self + x).mod_down(m)
                }

                fn mod_down(self, m: Self) -> Self {
                    if self >= m {
                        return self - m;
                    }
                    return self;
                }
            }
        )*
    }
}

impl_mod_add!(u8 u16 u32 u64 usize);
