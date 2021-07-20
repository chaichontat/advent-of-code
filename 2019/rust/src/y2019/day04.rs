type Parsed = isize;

pub fn parse(raw: &str) -> Vec<Parsed> {
    raw.split('\n').map(|x| x.parse().unwrap()).collect()
}

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

pub fn part1(parsed: &[Parsed]) -> usize {
    (parsed[0]..parsed[1])
        .filter(|x| val1(&format!("{}", x)))
        .count()
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

pub fn part2(parsed: &[Parsed]) -> usize {
    (parsed[0]..parsed[1])
        .filter(|x| val2(&format!("{}", x)))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::*;
    #[test]
    fn test1() {
        assert_eq!(part1(&parse(&read(2019, "day04.txt"))), 1625);
        assert!(val1("111111"));
        assert!(!val1("223450"));
        assert!(!val1("123789"));
    }

    #[test]
    fn test2() {
        assert_eq!(part2(&parse(&read(2019, "day04.txt"))), 1111);
        assert!(val2("112233"));
        assert!(!val2("123444"));
        assert!(val2("111122"));
    }
}
