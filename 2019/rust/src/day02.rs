use std::cmp::Ordering;

use super::intcode::*;

pub fn part1(raw: &[String]) -> usize {
    let mut ic = IntCode::from(&raw[0]);
    ic.mem[1] = 12;
    ic.mem[2] = 2;
    ic.run();
    ic.mem[0] as usize
}

/// Initial 10×10 grid search indicates monotonicity of the intcode function.
/// Higher gradient w.r.t mem[1]. Binary search on mem[1] first then mem[2].

fn fn_binary_search(
    f: impl Fn(isize) -> isize,
    target: isize,
    mut lo: isize,
    mut hi: isize,
) -> Result<isize, isize> {
    while lo < hi {
        let mid = (hi + lo) / 2;
        let fmid = f(mid);
        match target.cmp(&fmid) {
            Ordering::Equal => return Ok(mid),
            Ordering::Greater => lo = mid + 1,
            Ordering::Less => hi = mid - 1,
        }
    }
    if f(lo) == target {
        Ok(lo)
    } else {
        Err(lo)
    }
}

fn run_ic(mem: &[isize], m1: isize, m2: isize) -> isize {
    let mut ic = IntCode::from(mem);
    ic.mem[1] = m1;
    ic.mem[2] = m2;
    ic.run();
    ic.mem[0]
}

pub fn part2(raw: &[String]) -> usize {
    let target = 19690720;
    let mem: Vec<isize> = raw[0]
        .split(',')
        .map(|x| x.parse::<isize>().unwrap())
        .collect();

    match fn_binary_search(|x| run_ic(&mem, x, 0), target, 0, 100) {
        Ok(r) => 100 * r as usize,
        Err(r) => match fn_binary_search(|x| run_ic(&mem, r - 1, x), target, 0, 100) {
            Ok(t) => (100 * (r - 1) + t) as usize,
            Err(_) => panic!("Doesn't work."),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::*;
    #[test]
    fn test1() {
        assert_eq!(part1(&read("day02.txt")), 5110675);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(&read("day02.txt")), 4847);
    }
}
