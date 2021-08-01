use std::{cmp::Reverse, collections::BinaryHeap};

use hashbrown::{HashMap, HashSet};
use nohash_hasher::IntMap;
use pathfinding::prelude::{bfs, dijkstra};

#[derive(Debug)]
struct KeyDistDoors {
    dist:  u32,
    doors: Bits,
}

// We need "keys" to open "doors".
type Bits = u32;
type Key = u32;
type Pos = (u32, u32);
type KeyPosMap = IntMap<Key, Pos>;

type ReachableKeysMap = IntMap<u32, KeyDistDoors>;
type AllReachableKeysMap = IntMap<u32, ReachableKeysMap>;

trait IntAscii {
    fn int_lower(&self) -> u32;
    fn int_upper(&self) -> u32;
}

impl IntAscii for char {
    fn int_lower(&self) -> u32 {
        *self as u32 - 'a' as u32
    }

    fn int_upper(&self) -> u32 {
        *self as u32 - 'A' as u32
    }
}

type Parsed = String;

struct Board {
    n_bot:  u8,
    board:  HashMap<Pos, u8>,
    keypos: KeyPosMap,
}

impl Board {
    fn new(raw: &[String]) -> Self {
        let mut n_bot = 0;
        let mut keypos = IntMap::default();
        let mut board = HashMap::new();
        for (y, line) in raw.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c.is_ascii_lowercase() {
                    keypos.insert(c.int_lower(), (x as u32, y as u32));
                    board.insert((x as u32, y as u32), c.int_lower() as u8);
                } else if c == '@' {
                    keypos.insert(27 + n_bot, (x as u32, y as u32));
                    board.insert((x as u32, y as u32), 27 + n_bot as u8);
                    n_bot += 1;
                } else if c == '.' {
                    board.insert((x as u32, y as u32), '.' as u8);
                }
            }
        }
        Board {
            n_bot: n_bot as u8,
            board,
            keypos,
        }
    }
}

/// Locate position of keys on the board.
fn locate_keys(raw: &[String]) -> (KeyPosMap, usize) {
    let mut n_bot = 0;
    let mut out = IntMap::default();
    for (y, line) in raw.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c.is_ascii_lowercase() {
                out.insert(c.int_lower(), (x as u32, y as u32));
            } else if c == '@' {
                out.insert(27 + n_bot, (x as u32, y as u32));
                n_bot += 1;
            }
        }
    }
    (out, n_bot as usize)
}

/// Given curr_key, find all reachable keys. Reachable keys are encoded with distance and
/// key requirements in a bitfield.
fn key_explore(raw: &[String], curr_key: u32, start: Pos) -> ReachableKeysMap {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    struct KeyPathProg {
        dist:  u32,
        pos:   Pos,
        doors: Bits,
    }

    fn steps((x, y): Pos) -> [Pos; 4] {
        [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
    }

    let bound = (raw[0].len(), raw.len());
    let mut q = BinaryHeap::new();
    q.push(Reverse(KeyPathProg {
        dist:  0,
        pos:   start,
        doors: 0,
    }));

    let mut traversed = HashSet::<Pos>::new();
    let mut reachablekeys = IntMap::default();

    while let Some(Reverse(KeyPathProg {
        dist,
        pos: (xc, yc),
        mut doors,
    })) = q.pop()
    {
        match raw[yc as usize].chars().nth(xc as usize).unwrap() {
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
            if (0..bound.0).contains(&(x_new as usize))
                && (0..bound.1).contains(&(y_new as usize))
                && !traversed.contains(&(x_new, y_new))
            {
                match raw[y_new as usize].chars().nth(x_new as usize) {
                    Some(cc) if cc.is_ascii_alphabetic() || cc == '.' || cc == '@' => {
                        q.push(Reverse(KeyPathProg {
                            dist: dist + 1,
                            pos: (x_new, y_new),
                            doors,
                        }));
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

fn main_search(allkeyspath: &AllReachableKeysMap, n_keys: u32, n_bot: usize) -> Option<u32> {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    struct MainPathProg {
        dist:      u32,
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

    let mut locs = [255_u32; 4];

    #[allow(clippy::needless_range_loop)]
    for n in 0..n_bot {
        locs[n] = 27 + n as u32;
    }

    q.push(Reverse(MainPathProg {
        dist: 0,
        locs, // @
        keys_done: 0,
    }));

    let mut seen = HashSet::<SearchState>::new();

    while let Some(Reverse(MainPathProg { dist, locs, keys_done })) = q.pop() {
        if seen.contains(&SearchState { locs, keys_done }) {
            continue;
        }
        if keys_done == goal {
            return Some(dist);
        }
        seen.insert(SearchState { locs, keys_done });

        for n in 0..n_bot {
            for (
                loc_next,
                KeyDistDoors {
                    dist: dist_to_next,
                    doors,
                },
            ) in allkeyspath.get(&locs[n]).unwrap().iter()
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

pub fn parse(raw: &str) -> Vec<Parsed> {
    raw.split('\n').map(|x| x.to_string()).collect()
}

pub fn run(raw: &[String]) -> u32 {
    let (keys_pos, n_bot) = locate_keys(raw);
    let all_keys_paths = map_key_explore(raw, &keys_pos);
    let n_keys = keys_pos.len() - n_bot;

    main_search(&all_keys_paths, n_keys as u32, n_bot).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::*;

    #[test]
    fn test1() {
        assert_eq!(run(&parse(&read(2019, "day18.txt"))), 5402);
    }
    // #[test]
    fn test2() {
        let mut raw = parse(&read(2019, "day18.txt"));
        let xmid = (raw[0].len() - 1) / 2;
        let ymid = (raw.len() - 1) / 2;
        raw[ymid - 1].replace_range(xmid - 1..=xmid + 1, "@#@");
        raw[ymid].replace_range(xmid - 1..=xmid + 1, "###");
        raw[ymid + 1].replace_range(xmid - 1..=xmid + 1, "@#@");
        assert_eq!(run(&raw), 2138);
    }
}
