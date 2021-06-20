use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::fs;
use std::hash::Hash;

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

impl<T: Clone + Eq + Hash, R> Set for HashMap<T, R> {
    type Item = HashSet<T>;
    fn keys_to(&self) -> Self::Item {
        self.keys().cloned().collect::<Self::Item>()
    }
}

// pub fn to_tuple<T>(v: Vec<&T>) -> (&T,) {
//     v.iter().collect_tuple().unwrap()
// }
