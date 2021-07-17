use regex::Regex;
use std::fmt::Debug;
use std::fs;

pub fn read(path: &str) -> Vec<String> {
    let p = format!("../data/{}", path);
    let res = fs::read_to_string(p).unwrap();
    let mut vec: Vec<String> = res.split("\n").map(str::to_string).collect();
    vec.truncate(vec.len() - 1);
    vec
}

pub fn read_nosep(path: &str) -> String {
    let p = format!("../data/{}", path);
    let mut res = fs::read_to_string(p).unwrap();
    res.pop();
    res
}

pub fn str_idx(s: &String, i: usize) -> char {
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
