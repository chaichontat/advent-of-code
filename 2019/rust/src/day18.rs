use std::{cmp::Reverse, collections::BinaryHeap};

use ahash::AHashSet;
use nohash_hasher::IntMap;

#[derive(Debug)]
struct KeyDistDoors {
    dist:  usize,
    doors: Bits,
}

// We need "keys" to open "doors".
type Bits = usize;
type Key = usize;
type Pos = (usize, usize);
type KeyPosMap = IntMap<Key, Pos>;

type ReachableKeysMap = IntMap<usize, KeyDistDoors>;
type AllReachableKeysMap = IntMap<usize, ReachableKeysMap>;

trait IntAscii {
    fn int_lower(&self) -> usize;
    fn int_upper(&self) -> usize;
}

impl IntAscii for char {
    fn int_lower(&self) -> usize {
        *self as usize - 'a' as usize
    }

    fn int_upper(&self) -> usize {
        *self as usize - 'A' as usize
    }
}

/// Locate position of keys on the board.
fn locate_keys(raw: &[String]) -> (KeyPosMap, usize) {
    let mut n_bot = 0;
    let mut out = IntMap::default();
    for (y, line) in raw.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c.is_ascii_lowercase() {
                out.insert(c.int_lower(), (x, y));
            } else if c == '@' {
                out.insert(27 + n_bot, (x, y));
                n_bot += 1;
            }
        }
    }
    (out, n_bot)
}

/// Given curr_key, find all reachable keys. Reachable keys are encoded with distance and
/// key requirements in a bitfield.
fn key_explore(raw: &[String], curr_key: usize, start: Pos) -> ReachableKeysMap {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    struct KeyPathProg {
        dist:  usize,
        pos:   Pos,
        doors: Bits,
    }

    fn steps((x, y): Pos) -> [Pos; 4] {
        [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
    }

    let bound = (raw[0].len(), raw.len());
    let mut q = BinaryHeap::new();
    q.push(Reverse(KeyPathProg { dist: 0, pos: start, doors: 0 }));

    let mut traversed = AHashSet::<Pos>::new();
    let mut reachablekeys = IntMap::default();

    while let Some(Reverse(KeyPathProg { dist, pos: (xc, yc), mut doors })) = q.pop() {
        match raw[yc].chars().nth(xc).unwrap() {
            c if c.is_ascii_lowercase() => {
                if c.int_lower() != curr_key {
                    reachablekeys.insert(c.int_lower(), KeyDistDoors { dist, doors });
                    continue;
                }
            }
            c if c.is_ascii_uppercase() => {
                doors |= 1 << c.int_upper();
            }
            '.' | '@' => (),
            _ => unreachable!(),
        };

        // Explore
        for (x_new, y_new) in steps((xc, yc)) {
            if (0..bound.0).contains(&x_new)
                && (0..bound.1).contains(&y_new)
                && !traversed.contains(&(x_new, y_new))
            {
                match raw[y_new].chars().nth(x_new) {
                    Some(cc) if cc.is_ascii_alphabetic() || cc == '.' || cc == '@' => {
                        q.push(Reverse(KeyPathProg { dist: dist + 1, pos: (x_new, y_new), doors }));
                        traversed.insert((x_new, y_new));
                    }
                    Some('#') => (),
                    _ => unreachable!(),
                }
            }
        }
    }
    reachablekeys
}

fn map_key_explore(raw: &[String], keyspos: &KeyPosMap) -> AllReachableKeysMap {
    let mut allkeys = IntMap::default();
    keyspos.iter().for_each(|(k, pos)| {
        allkeys.insert(*k, key_explore(raw, *k, *pos));
    });
    allkeys
}

fn main_search(allkeyspath: &AllReachableKeysMap, n_keys: usize, n_bot: usize) -> Option<usize> {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    struct MainPathProg {
        dist:      usize,
        locs:      [Key; 4], // Current location
        keys_done: Bits,
    }

    #[derive(Hash, PartialEq, Eq)]
    struct SearchState {
        locs:      [Key; 4],
        keys_done: Bits,
    }

    let goal = (0..n_keys).fold(0, |sum, i| sum | 1 << i);
    let mut q = BinaryHeap::new();

    let mut locs = [255; 4];

    #[allow(clippy::needless_range_loop)]
    for n in 0..n_bot {
        locs[n] = 27 + n;
    }

    q.push(Reverse(MainPathProg {
        dist: 0,
        locs, // @
        keys_done: 0,
    }));

    let mut seen = AHashSet::<SearchState>::new();

    while let Some(Reverse(MainPathProg { dist, locs, keys_done })) = q.pop() {
        if seen.contains(&SearchState { locs, keys_done }) {
            continue;
        }
        if keys_done == goal {
            return Some(dist);
        }
        seen.insert(SearchState { locs, keys_done });

        for n in 0..n_bot {
            for (loc_next, KeyDistDoors { dist: dist_to_next, doors }) in
                allkeyspath.get(&locs[n]).unwrap().iter()
            {
                let mut locs_new = locs;
                locs_new[n] = *loc_next;

                if (keys_done & doors) == *doors {
                    q.push(Reverse(MainPathProg {
                        dist:      dist + dist_to_next,
                        locs:      locs_new,
                        keys_done: keys_done | (1 << loc_next),
                    }))
                }
            }
        }
    }
    None
}

pub fn run(raw: &[String]) -> usize {
    let (keys_pos, n_bot) = locate_keys(raw);
    let all_keys_paths = map_key_explore(raw, &keys_pos);
    let n_keys = keys_pos.len() - n_bot;

    main_search(&all_keys_paths, n_keys, n_bot).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::*;

    #[test]
    fn test1() {
        assert_eq!(run(&read("day18.txt")), 5402);
    }
    #[test]
    fn test2() {
        let mut raw = read("day18.txt");
        let xmid = (raw[0].len() - 1) / 2;
        let ymid = (raw.len() - 1) / 2;
        raw[ymid - 1].replace_range(xmid - 1..=xmid + 1, "@#@");
        raw[ymid].replace_range(xmid - 1..=xmid + 1, "###");
        raw[ymid + 1].replace_range(xmid - 1..=xmid + 1, "@#@");
        assert_eq!(run(&raw), 2138);
    }
}
