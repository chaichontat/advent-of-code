use itertools::Itertools;
use nohash_hasher::IntMap;

#[derive(Debug, Clone, Copy)]
struct Orbit {
    parent:    u32,
    depth:     Option<u32>,
    traversed: bool, // for part 2.
}

type Parsed = (u32, u32);
type OrbitMap = IntMap<u32, Orbit>;
const COM: u32 = 203230;

fn encode(s: &str) -> u32 {
    s.chars().fold(0, |sum, c| 100 * sum + (c as u32 - 47))
}

pub fn parse(raw: &str) -> Vec<Parsed> {
    raw.split('\n')
        .map(|x| x.to_string())
        .map(|line| {
            let (parent, remaining) = line.split_at(3);
            let parent = encode(parent); // '0' turns into 1.
            let child = encode(&remaining[1..]);
            (parent, child)
        })
        .collect()
}

fn parse_map(parsed: &[Parsed]) -> OrbitMap {
    let mut map = IntMap::default();
    for (parent, child) in parsed {
        if map
            .insert(*child, Orbit {
                parent:    *parent,
                depth:     None,
                traversed: false,
            })
            .is_some()
        {
            panic!("Child has two parents.");
        }
    }
    map
}

fn depth(map: &mut OrbitMap, idx: u32) -> Option<u32> {
    if idx == COM {
        return Some(0); // COM
    }

    let d = map.get(&idx).expect("Key not found.");
    let mut x = d.depth;
    if x.is_none() {
        let p = d.parent;
        x = depth(map, p).map(|x| x + 1);
        let d = map.get_mut(&idx).unwrap();
        d.depth = x;
    }
    x
}

pub fn part1(parsed: &[Parsed]) -> u32 {
    let mut map = parse_map(parsed);
    let keys = map.keys().copied().collect_vec();
    keys.iter().map(|&idx| depth(&mut map, idx).unwrap()).sum()
}

pub fn part2(parsed: &[Parsed]) -> u32 {
    let mut map = parse_map(parsed);
    let keys = map.keys().copied().collect_vec();
    keys.iter().map(|&idx| depth(&mut map, idx).unwrap()).sum::<u32>();

    // Find depth of shared parent.
    // Mark all traversed nodes.
    let mut orb = map.get_mut(&encode("YOU")).unwrap();
    let mut depths = orb.depth.unwrap() - 2; // Doesn't count closest transfers.
    orb.traversed = true;
    let mut p = orb.parent;
    while p != COM {
        orb = map.get_mut(&p).unwrap();
        orb.traversed = true;
        p = orb.parent;
    }

    // Find first previously traversed node.
    let mut orb = map.get(&encode("SAN")).unwrap();
    depths += orb.depth.unwrap();
    while !orb.traversed {
        orb = map.get(&orb.parent).unwrap();
    }
    depths -= 2 * orb.depth.unwrap();
    depths
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::*;
    #[test]
    fn test1() {
        assert_eq!(part1(&parse(&read(2019, "day06.txt"))), 402879);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(&parse(&read(2019, "day06.txt"))), 484);
    }
}
