// use regex::Regex;  // Cannot use lookaheads.

fn val1(n: &str) -> bool {
    let mut ok = false;
    let n = n.as_bytes();
    for i in 0..(n.len() - 1) {
        if n[i] > n[i + 1] {
            // Going from left to right, the digits never decrease.
            return false;
        }
        if n[i] == n[i + 1] {
            // Two adjacent digits are the same.
            ok = true
        }
    }
    ok
}

pub fn part1(raw: &[String]) -> usize {
    let inp: Vec<isize> = raw.iter().map(|x| x.parse::<isize>().unwrap()).collect();
    (inp[0]..inp[1]).filter(|x| val1(&format!("{}", x))).count()
}

fn val2(n: &str) -> bool {
    let mut twosame = false;
    let mut onlytwosame = false;
    let mut lenmatch = 1;
    let n = n.as_bytes();

    for i in 0..(n.len() - 1) {
        if n[i] > n[i + 1] {
            return false;
        }
        if n[i] == n[i + 1] {
            twosame = true;
            lenmatch += 1;
        } else if lenmatch == 2 {
            onlytwosame = true;
        } else {
            lenmatch = 1;
        }
    }
    twosame && (onlytwosame || lenmatch == 2)
}

pub fn part2(raw: &[String]) -> usize {
    let inp: Vec<isize> = raw.iter().map(|x| x.parse::<isize>().unwrap()).collect();
    (inp[0]..inp[1]).filter(|x| val2(&format!("{}", x))).count()
}

#[cfg(test)]
mod tests {
    use super::super::utils::*;
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(part1(&read("day04.txt")), 1625);
        assert_eq!(val1("111111"), true);
        assert_eq!(val1("223450"), false);
        assert_eq!(val1("123789"), false);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(&read("day04.txt")), 1111);
        assert_eq!(val2("112233"), true);
        assert_eq!(val2("123444"), false);
        assert_eq!(val2("111122"), true);
    }
}
