use ascii::{AsciiStr, AsciiString};
use itertools::Itertools;
use std::error;
use std::fmt::Debug;
use std::fs;

pub type SomeResult<T> = std::result::Result<T, Box<dyn error::Error>>;

pub fn read(path: &str) -> Vec<String> {
    let p = format!("../data/{}", path);
    let res = fs::read_to_string(p).unwrap();
    let mut vec: Vec<String> = res.split('\n').map(str::to_string).collect();
    vec.truncate(vec.len() - 1);
    vec
}

pub fn read_nosep(path: &str) -> String {
    let p = format!("../data/{}", path);
    let mut res = fs::read_to_string(p).unwrap();
    res.pop();
    res
}

pub fn read_ascii_sep(path: &str) -> Vec<AsciiString> {
    let p = format!("../data/{}", path);
    let res = AsciiString::from_ascii(fs::read(p).unwrap()).unwrap();
    let mut vec = res
        .split(ascii::AsciiChar::LineFeed)
        .map(AsciiStr::to_owned)
        .collect_vec();
    vec.truncate(vec.len() - 1);
    vec
}

pub fn read_ascii(path: &str) -> AsciiString {
    let p = format!("../data/{}", path);
    let mut res = AsciiString::from_ascii(fs::read(p).unwrap()).unwrap();
    res.pop();
    res
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

pub fn printt(x: &impl Debug) {
    println!("{:?}", x);
}
