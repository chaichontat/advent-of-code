use ascii::{AsciiStr, AsciiString};
use itertools::Itertools;
use std::fmt::Debug;
use std::fs;

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

pub fn printt(x: &impl Debug) {
    println!("{:?}", x);
}
