use std::fs;

pub fn read(path: &str) -> Vec<String> {
    let p = format!("../data/{}", path);
    let res = match fs::read_to_string(p) {
        Ok(s) => s,
        Err(e) => panic!("{}", e),
    };
    let mut vec: Vec<String> = res.split("\n").map(str::to_string).collect();
    vec.truncate(vec.len() - 1);
    vec
}
