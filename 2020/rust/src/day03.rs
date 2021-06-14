use super::utils::*;

fn run(raw: &Vec<String>, modx: usize, mody: usize) -> usize {
    let block_size = raw[0].len();
    let mut count: usize = 0;
    for i in 0..raw.len() {
        let y = mody * i;
        if y < raw.len() && str_idx(&raw[y], (modx * i) % block_size) == '#' {
            count += 1;
        }
    }
    count
}

pub fn part1(input: &Vec<String>) -> usize {
    run(input, 3, 1)
}

pub fn part2(input: &Vec<String>) -> usize {
    run(input, 1, 2) * (1..8).step_by(2).map(|i| run(&input, i, 1)).product::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]

    fn test1() {
        assert_eq!(part1(&read("day03.txt")), 265);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(&read("day03.txt")), 3154761400);
    }
}
