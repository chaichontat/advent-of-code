use ahash::AHashMap;

type Parsed = u8;

#[derive(Clone, Copy, Debug, Default)]
struct State {
    x:    i8,
    y:    i8,
    dist: u16,
}

pub fn parse(raw: &str) -> Vec<u8> {
    raw.as_bytes().to_vec()
}

pub fn combi(parsed: &[Parsed]) -> Option<(u32, u32)> {
    let mut stack = Vec::<State>::with_capacity(500);
    let mut seen = AHashMap::with_capacity(10000);
    let mut curr = State::default();

    let mut part1 = 0;
    let mut part2 = 0;

    for &c in parsed {
        match c {
            b'N' => curr.y += 1,
            b'E' => curr.x += 1,
            b'S' => curr.y -= 1,
            b'W' => curr.x -= 1,
            //
            b'(' => stack.push(curr),              // Save state before OR.
            b'|' => curr = *stack.last().unwrap(), // Get said state.
            b')' => curr = stack.pop().unwrap(),   // Done alternative exploration, go back.
            b'^' | b'$' => (),
            _ => panic!("Invalid input."),
        }

        if let b'N' | b'E' | b'S' | b'W' = c {
            if seen.insert((curr.x, curr.y), curr).is_none() {
                curr.dist += 1;
                part1 = part1.max(curr.dist);
                part2 += (curr.dist >= 1000) as u32;
            }
        }
    }

    Some((part1.into(), part2))
}

mod tests {
    use super::{combi, parse};
    use crate::utils::read;

    #[test]
    fn test_combi() {
        assert_eq!(combi(&parse(&read(2018, "day20.txt"))), Some((4108, 8366)));
    }
}
