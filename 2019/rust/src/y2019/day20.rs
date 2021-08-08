use std::cmp::Reverse;
use std::collections::BinaryHeap;

use ascii::{AsAsciiStr, AsciiChar, AsciiString};
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use pathfinding::prelude::{bfs, dijkstra};

use crate::hashmap;
type Pos = (usize, usize);
type Portal = (AsciiChar, AsciiChar);

fn scan(raw: &[AsciiString], x: usize, y: usize) -> Option<(Portal, Pos)> {
    if raw[y][x + 1] == AsciiChar::Dot {
        return Some(((raw[y][x - 1], raw[y][x]), (x + 1, y))); // Path on right
    }
    if raw[y][x - 1] == AsciiChar::Dot {
        return Some(((raw[y][x], raw[y][x + 1]), (x - 1, y))); // Path on left
    }
    if raw[y + 1][x] == AsciiChar::Dot {
        return Some(((raw[y - 1][x], raw[y][x]), (x, y + 1))); // Path on bottom
    }
    if raw[y - 1][x] == AsciiChar::Dot {
        return Some(((raw[y][x], raw[y + 1][x]), (x, y - 1))); // Path on top
    }
    None
}

struct Board {
    walkable: HashSet<Pos>,
    portals:  HashMap<Pos, (Pos, i32)>,
    start:    Pos,
    finish:   Pos,
    posport:  HashMap<Pos, Portal>,
}

impl Board {
    fn new(raw: &[AsciiString]) -> Self {
        let mut temp = HashMap::new();
        let mut portals = HashMap::new();
        let mut posport = HashMap::new();
        let mut walkable = HashSet::new();

        let (mut start, mut finish) = (None, None);

        let dim = (raw.len(), raw[0].len());
        for (y, line) in raw.iter().enumerate() {
            for (x, c) in line.into_iter().enumerate() {
                match c {
                    AsciiChar::Dot => {
                        walkable.insert((x, y));
                        continue;
                    }
                    d if d.is_uppercase() => (),
                    _ => continue,
                }

                if (1..dim.0 - 1).contains(&y) && (1..dim.1 - 1).contains(&x) {
                    if let Some((port, pos)) = scan(raw, x, y) {
                        posport.insert(pos, port);

                        if port == (AsciiChar::A, AsciiChar::A) {
                            start = Some(pos);
                            continue;
                        }

                        if port == (AsciiChar::Z, AsciiChar::Z) {
                            finish = Some(pos);
                            continue;
                        }

                        if let Some(another) = temp.insert(port, pos) {
                            if y == 1 || y == dim.0 - 2 || x == 1 || x == dim.1 - 2 {
                                portals.insert(another, (pos, 1));
                                portals.insert(pos, (another, -1));
                            } else {
                                portals.insert(another, (pos, -1));
                                portals.insert(pos, (another, 1));
                            }
                        }
                    }
                }
            }
        }
        Board {
            walkable,
            portals,
            start: start.unwrap(),
            finish: finish.unwrap(),
            posport,
        }
    }

    fn steps(&self, (x, y): Pos, teleport: bool) -> (Vec<Pos>, Option<Portal>) {
        let mut v = Vec::new();
        let mut c = None;
        [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
            .iter()
            .filter(|p| self.walkable.contains(p))
            .for_each(|&p| v.push(p));

        if let Some(&(u, _)) = self.portals.get(&(x, y)) {
            if teleport {
                v.push(u);
            }
        }

        if let Some(&u) = self.posport.get(&(x, y)) {
            c = Some(u);
        }
        (v, c)
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct ProgR {
    total_dist: u32,
    pos:        Pos,
    nodefrom:   Pos,
    nodedist:   u32,
}

// Standard BFS.
pub fn part1(raw: &[AsciiString]) -> Option<usize> {
    let board = Board::new(raw);
    let result = bfs(
        &board.start,
        |&pos| board.steps(pos, true).0,
        |&pos| pos == board.finish,
    )
    .expect("No path.");
    Some(result.len() - 1)
}

type Dists = HashMap<Pos, u32>;

// BFS
fn get_reachable(board: &Board, search_points: &HashMap<Pos, Portal>) -> HashMap<Pos, Dists> {
    // fn successors(board: &Board) -> Vec<Pos> {
    //     board.steps(pos, false)
    // }
    // let mut res = HashMap::new();
    // for (&start, &curr) in search_points.iter() {
    // let test = dijkstra_partial(start, successors, |p| )
    // }

    let mut pq = BinaryHeap::new();
    let mut traversed = HashSet::new();
    let mut res = HashMap::new();

    let mut nodes = HashMap::new();

    for (&start, &curr) in search_points.iter() {
        if start == board.finish || res.contains_key(&start) {
            continue;
        }

        let mut dists = HashMap::new();
        pq.push(Reverse(ProgR {
            total_dist: 0,
            pos:        start,
            nodefrom:   start,
            nodedist:   0,
        }));

        while let Some(Reverse(ProgR {
            total_dist: dist,
            pos,
            mut nodefrom,
            mut nodedist,
        })) = pq.pop()
        {
            let (steps, c) = board.steps(pos, false);
            if steps.len() > 1 || c.is_some() {
                let n = nodes.entry(nodefrom).or_insert_with(Vec::new);
                (*n).push((pos, nodedist));
                let n = nodes.entry(pos).or_insert_with(Vec::new);
                (*n).push((nodefrom, nodedist));
                nodefrom = pos;
                nodedist = 0;
            }

            if let Some(x) = c {
                if x != curr && pos != board.start {
                    dists.insert(pos, dist);
                    continue;
                }
            }

            for pos_new in steps {
                if pos_new == board.start {
                    continue;
                }
                if traversed.insert(pos_new) {
                    pq.push(Reverse(ProgR {
                        total_dist: dist + 1,
                        pos: pos_new,
                        nodefrom,
                        nodedist: nodedist + 1,
                    }));
                }
            }
        }

        if dists.len() == 1 {
            let (pos, d) = dists.iter().next().unwrap(); // Symmetrical.
            res.insert(*pos, hashmap![start => *d]);
        } // else {
          //     let mut distsadd = dists.clone();
          //     distsadd.insert(start, 0);
          //     for ((&st, _), (&fi, &oridist)) in distsadd.iter().cartesian_product(distsadd.iter()) {
          //         if st == start || st == fi {
          //             continue;
          //         }
          //         if fi == start {
          //             let m = res.entry(st).or_insert_with(HashMap::new);
          //             m.insert(fi, oridist);
          //             continue;
          //         }

        //         let t = dijkstra(&st, |p| nodes.get(p).unwrap().to_owned(), |&p| p == fi);
        //         let m = res.entry(st).or_insert_with(HashMap::new);
        //         m.insert(fi, t.unwrap().1);
        //         // printt(&t.unwrap().1);
        //     }
        // }

        res.insert(start, dists);
        pq.clear();
        traversed.clear();
    }
    res
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Prog {
    dist: u32,
    pos:  Pos,
}

type Parsed = AsciiString;
pub fn parse(raw: &str) -> Vec<Parsed> {
    raw.split('\n')
        .map(|x| x.as_ascii_str().unwrap().to_owned())
        .collect()
}

type Reachable = HashMap<Pos, HashMap<Pos, u32>>;
// Dijkstra
pub fn part1a(parsed: &[Parsed]) -> Option<u32> {
    fn successors(board: &Board, reach: &Reachable, pos: Pos) -> Vec<(Pos, u32)> {
        let nbrs = reach.get(&pos).unwrap().iter();
        nbrs.filter_map(|(&p, &d)| match p {
            _ if p == board.start => None,
            _ if p == board.finish => Some((p, d)),
            _ => Some((board.portals.get(&p).unwrap().0, d + 1)),
        })
        .collect_vec()
    }

    let board = Board::new(parsed);
    let reach = get_reachable(&board, &board.posport);
    let res = dijkstra(
        &board.start,
        |&p| successors(&board, &reach, p),
        |&pos| pos == board.finish,
    );
    Some(res.expect("No path").1)
}

pub fn part2(parsed: &[Parsed]) -> Option<u32> {
    fn successors(board: &Board, reach: &Reachable, (pos, level): (Pos, i32)) -> Vec<((Pos, i32), u32)> {
        let mut out = Vec::new();
        for (&pos_new, &new_dist) in reach.get(&pos).unwrap() {
            let (pos_new2, level_new);

            if level == 0 && pos_new == board.finish {
                pos_new2 = pos_new; // Only outermost layer has goal.
                level_new = 0;
            } else if pos_new == board.finish || pos_new == board.start {
                continue;
            } else {
                let (a, b) = *board.portals.get(&pos_new).unwrap();
                pos_new2 = a;
                level_new = b;
                if level + level_new < 0 {
                    continue;
                }
            }
            out.push(((pos_new2, level + level_new), new_dist + 1));
        }
        out
    }

    let board = Board::new(parsed);
    let reach = get_reachable(&board, &board.posport);

    let result = dijkstra(
        &(board.start, 0),
        |&pos| successors(&board, &reach, pos),
        |&pos| pos == (board.finish, 0),
    );
    Some(result.expect("No path.").1 - 1)
}

pub fn bench(raw: &[AsciiString]) -> (u32, u32) {
    fn successors1(board: &Board, reach: &Reachable, pos: Pos) -> Vec<(Pos, u32)> {
        let nbrs = reach.get(&pos).unwrap().iter();
        nbrs.filter_map(|(&p, &d)| match p {
            _ if p == board.start => None,
            _ if p == board.finish => Some((p, d)),
            _ => Some((board.portals.get(&p).unwrap().0, d + 1)),
        })
        .collect_vec()
    }

    let board = Board::new(raw);
    let reach = get_reachable(&board, &board.posport);
    let res = dijkstra(
        &board.start,
        |&p| successors1(&board, &reach, p),
        |&pos| pos == board.finish,
    );
    let part1 = res.expect("No path").1;

    fn successors2(board: &Board, reach: &Reachable, (pos, level): (Pos, i32)) -> Vec<((Pos, i32), u32)> {
        let mut out = Vec::new();
        for (&pos_new, &new_dist) in reach.get(&pos).unwrap() {
            let (pos_new2, level_new);

            if level == 0 && pos_new == board.finish {
                pos_new2 = pos_new; // Only outermost layer has goal.
                level_new = 0;
            } else if pos_new == board.finish || pos_new == board.start {
                continue;
            } else {
                let (a, b) = *board.portals.get(&pos_new).unwrap();
                pos_new2 = a;
                level_new = b;
                if level + level_new < 0 {
                    continue;
                }
            }
            out.push(((pos_new2, level + level_new), new_dist + 1));
        }
        out
    }

    // let board = Board::new(raw);
    // let reach = get_reachable(&board, &board.posport);

    let result = dijkstra(
        &(board.start, 0),
        |&pos| successors2(&board, &reach, pos),
        |&pos| pos == (board.finish, 0),
    );
    let part2 = result.expect("No path.").1 - 1;

    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::*;
    #[test]
    fn test1() {
        assert_eq!(part1(&parse(&read(2019, "day20.txt"))), Some(658));
    }

    #[test]
    fn test2() {
        assert_eq!(part2(&parse(&read(2019, "day20.txt"))), Some(7612));
    }
}
