fn fuel(x: usize) -> Option<usize> {
    (x / 3).checked_sub(2)
}

fn recurse(x: usize) -> usize {
    match fuel(x) {
        Some(y) => recurse(y) + y,
        _ => 0,
    }
}

pub fn run(raw: &[String]) -> (usize, usize) {
    (
        raw.iter()
            .map(|x| x.parse::<usize>().ok().and_then(fuel))
            .flatten()
            .sum(),
        raw.iter()
            .map(|x| x.parse::<usize>().ok().map(recurse))
            .flatten()
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::*;

    #[test]
    fn test() {
        assert_eq!(run(&read("day01.txt")), (3184233, 4773483));
    }
}
