use std::ops::Range;

use crate::intcode::*;

fn call(ic: &mut IntCode, cmd: isize) -> isize {
    ic.push(cmd + 1); // Shifting by 1 makes bitwise manipulation simple.
    ic.run_pause();
    ic.pop().unwrap()
}

// Assuming that the maze is a tree. DFS returns shortest distance.
fn dfs(ic: &mut IntCode, from: isize, mut dist: isize) -> (isize, isize) {
    let mut max_branch = 0;
    let mut max_spread = 0;

    for dir in (Range::<isize> { start: 0, end: 4 }) {
        if dir == from || call(ic, dir) == 0 {
            continue;
        }
        // Prevent backtracking and stop if stuck.
        // Else already one step ahead in dir.

        let new_o = call(ic, dir) & 2; // 2 if returns 2 else 0. Continue searching.
        let (o, m) = dfs(ic, dir ^ 1, new_o); // dir ^ 1 is the opposite direction.
        if o == 0 {
            // Haven't found yet.
            max_branch = max_branch.max(m + 2); // From all directions.
        } else {
            dist = o + 2; // Feed distance back up.
            max_spread = m;
        }

        call(ic, dir ^ 1); // Step back 2 times.
        call(ic, dir ^ 1);
    }

    (dist, max_spread.max(dist + max_branch)) // Sum farthest ever reached with dist to tank.
}

// Simply DFS until ic returns 2.
#[allow(dead_code)]
fn simple_dfs(ic: &mut IntCode, from: isize, dist: isize) -> Option<isize> {
    for dir in (Range::<isize> { start: 0, end: 4 }) {
        if dir == from {
            continue; // Prevent backtracking.
        }

        match call(ic, dir) {
            0 => continue, // Stuck. No movement.
            1 => (),
            2 => return Some(dist + 1), // Found tank. Stop.
            _ => unreachable!(),
        }

        match simple_dfs(ic, dir ^ 1, dist + 1) {
            Some(x) => return Some(x),
            None => call(ic, dir ^ 1), // Dead-end, step back.
        };
    }
    None
}

pub fn part1(raw: &[String]) -> usize {
    let mut ic = IntCode::from(&raw[0]);
    ic.run_wait_input();
    let (p1, p2) = dfs(&mut ic, -1, 0);
    (p1 - 2) as usize
}

// Need to find max number of steps from tank.
pub fn part2(raw: &[String]) -> usize {
    let mut ic = IntCode::from(&raw[0]);
    ic.run_wait_input();
    let (p1, p2) = dfs(&mut ic, -1, 0);
    (p2 - 2) as usize
}

pub fn run_both(raw: &[String]) -> (usize, usize) {
    let mut ic = IntCode::from(&raw[0]);
    ic.run_wait_input();
    let (p1, p2) = dfs(&mut ic, -1, 0);
    ((p1 - 2) as usize, (p2 - 2) as usize)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::*;
    #[test]
    fn test1() {
        assert_eq!(part1(&read("day15.txt")), 216);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(&read("day15.txt")), 326);
    }
}
