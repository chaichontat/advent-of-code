use std::cmp::Ordering;

use crate::utils::*;

type Parsed = usize;

pub fn parse(raw: &str) -> Vec<Parsed> {
    let line = raw.split('\n').next().unwrap();
    line.split(',').map(|x| x.parse()).flatten().collect()
}

fn simple_intcode(mem: &[Parsed], noun: usize, verb: usize) -> usize {
    let mut mem = mem.to_owned();
    let mut ptr = 0;
    mem[1] = noun;
    mem[2] = verb;

    loop {
        let loc = (mem[ptr + 1], mem[ptr + 2], mem[ptr + 3]);
        match mem[ptr] {
            1 => mem[loc.2] = mem[loc.0] + mem[loc.1],
            2 => mem[loc.2] = mem[loc.0] * mem[loc.1],
            99 => break,
            _ => unreachable!(),
        }
        ptr += 4;
    }
    mem[0]
}

unsafe fn simple_intcode_unchecked(mem: &[Parsed], noun: usize, verb: usize) -> usize {
    let mut mem = mem.to_owned();
    let mut ptr = 0;

    mem[2] = verb;
    mem[1] = noun;

    loop {
        unsafe {
            let op = *mem.get_unchecked(ptr);
            if op == 99 {
                break;
            }

            let ops = (
                *mem.get_unchecked(*mem.get_unchecked(ptr + 1)),
                *mem.get_unchecked(*mem.get_unchecked(ptr + 2)),
                *mem.get_unchecked(ptr + 3),
            );

            *mem.get_unchecked_mut(ops.2) = match op {
                1 => ops.0 + ops.1,
                2 => ops.0 * ops.1,
                _ => unreachable!(),
            };
        }

        ptr += 4;
    }
    mem[0]
}

pub fn run(parsed: &[Parsed]) -> Ans {
    // f(x) = ax₁ + b + x₂
    // Solve for a, b.
    // Then, find x₁ and x₂ that results in target.
    let x = (12, 2);
    let part1 = simple_intcode(parsed, x.0, x.1);

    let b = simple_intcode(parsed, 0, 0);
    let a = (part1 - b - x.1) / x.0;

    let target = 19690720;
    let u = target - b;
    let noun = u / a; // Divide and drop remainders.
    let verb = u - (noun * a); // Fine-tune remainder.
    assert_eq!(target, noun * a + verb + b);

    Some((part1, (100 * noun + verb)))

    // match fn_binary_search(|x| run_ic(&raw[0], x, 0), target, 0, 100) {
    //     Ok(r) => 100 * r as usize,
    //     Err(r) => match fn_binary_search(|x| run_ic(&raw[0], r - 1, x), target, 0, 100) {
    //         Ok(t) => (100 * (r - 1) + t) as usize,
    //         Err(_) => panic!("Doesn't work."),
    //     },
    // }
}

/// Initial 10×10 grid search indicates monotonicity of the intcode function.
/// Higher gradient w.r.t mem[1]. Binary search on mem[1] first then mem[2].
#[allow(dead_code)]
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(run(&parse(&read(2019, "day02.txt"))), Some((5110675, 4847)));
    }
}
