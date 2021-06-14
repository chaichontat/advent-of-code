use super::utils::*;
use itertools::Itertools;

fn process(input: &Vec<String>, f: fn((Vec<usize>, char, String)) -> bool) -> usize {
    let mut valid = 0;
    for x in input {
        let (num, c, pwd) = x.splitn(3, " ").collect_tuple().unwrap();
        let num = num
            .split("-")
            .map(|i| i.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let c = str_idx(&c.to_string(), 0);

        if f((num, c, pwd.to_string())) {
            valid += 1;
        }
    }
    valid
}

pub fn part1(input: &Vec<String>) -> usize {
    let f = |(num, c, pwd): (Vec<usize>, char, String)| -> bool {
        (num[0]..=num[1]).contains(&(pwd.matches(c).count()))
    };
    process(input, f)
}

pub fn part2(input: &Vec<String>) -> usize {
    let f = |(num, c, pwd): (Vec<usize>, char, String)| -> bool {
        (str_idx(&pwd, num[0] - 1) == c) ^ (str_idx(&pwd, num[1] - 1) == c)
    };
    process(input, f)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(part1(&read("day02.txt")), 447);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(&read("day02.txt")), 249);
    }
}

