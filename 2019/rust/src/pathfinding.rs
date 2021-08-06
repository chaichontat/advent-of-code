use std::cmp::Reverse;
use std::hash::Hash;
use std::{cmp::Ordering, collections::BinaryHeap};

use hashbrown::{HashMap, HashSet};
use num::PrimInt;

#[derive(Clone)]
struct DistWrap<T: Eq + Clone, D: PrimInt> {
    t: T,
    d: D,
}

impl<T: Eq + Clone, D: PrimInt> PartialOrd for DistWrap<T, D> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.d.cmp(&other.d))
    }
}

impl<T: Eq + Clone, D: PrimInt> Ord for DistWrap<T, D> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl<T: Eq + Clone, D: PrimInt> PartialEq for DistWrap<T, D> {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal) && self.t == other.t
    }
}

impl<T: Eq + Clone, D: PrimInt> Eq for DistWrap<T, D> {}

pub fn bfs_bucket<T, D, FS, FT, const N: usize>(
    start: T,
    mut successors: FS,
    mut target: FT,
    buc_size: usize,
) -> Option<T>
where
    T: Eq + Copy + Hash + Default,
    D: PrimInt + Into<usize>,
    FS: FnMut(&T) -> [Option<(T, D)>; N],
    FT: FnMut(&T) -> bool,
{
    let mut q = Vec::new();
    for _ in 0..buc_size {
        q.push(Vec::with_capacity(1024));
    }
    q[0].push(start);
    loop {
        while let Some(t) = q[0].pop() {
            if target(&t) {
                return Some(t);
            }

            let mut it = IntoIterator::into_iter(successors(&t));
            while let Some(Some((nbr, Δ))) = it.next() {
                q[Δ.into()].push(nbr);
            }
        }
        q.rotate_left(1);
    }
}

pub fn bfs<T, D, FS>(start: T, mut successors: FS, target: T) -> Option<D>
where
    T: Eq + Copy + Hash,
    D: PrimInt,
    FS: FnMut(&T) -> Vec<(D, T)>,
{
    let mut q = BinaryHeap::new();
    let mut visited = HashSet::new();
    q.push(Reverse(DistWrap { d: D::zero(), t: start }));

    while let Some(Reverse(DistWrap { d, t })) = q.pop() {
        if !visited.insert(t) {
            continue;
        }

        if t == target {
            return Some(d);
        }

        for (Δ, nbr) in successors(&t) {
            q.push(Reverse(DistWrap { t: nbr, d: d + Δ }));
        }
    }
    None
}

pub fn bfs_heuristics<T, D, FS, FH>(start: T, mut successors: FS, mut heuristics: FH, target: T) -> Option<D>
where
    T: Eq + Copy + Hash,
    D: PrimInt,
    FS: FnMut(&T) -> Vec<(D, T)>,
    FH: FnMut(&T) -> D,
{
    let mut q = BinaryHeap::new();
    let mut visited = HashSet::new();
    q.push(Reverse(DistWrap {
        d: D::zero(),
        t: DistWrap { d: D::zero(), t: start },
    }));

    while let Some(Reverse(DistWrap {
        d: _,
        t: DistWrap { d, t },
    })) = q.pop()
    {
        if !visited.insert(t) {
            continue;
        }

        if t == target {
            return Some(d);
        }

        for (Δ, nbr) in successors(&t) {
            q.push(Reverse(DistWrap {
                d: d + Δ + heuristics(&nbr),
                t: DistWrap { t: nbr, d: d + Δ },
            }));
        }
    }
    None
}

pub fn dijkstra<T, D, FS, FH>(start: T, mut successors: FS, target: T) -> Option<D>
where
    T: Eq + Copy + Hash,
    D: PrimInt,
    FS: FnMut(&T) -> Vec<(D, T)>,
    FH: FnMut(&T) -> D,
{
    let mut q = BinaryHeap::new();
    let mut dists = HashMap::new();
    let mut visited = HashSet::new();
    q.push(Reverse(DistWrap { d: D::zero(), t: start }));

    while let Some(Reverse(DistWrap { d, t })) = q.pop() {
        if !visited.insert(t) {
            continue;
        }

        for (Δ, nbr) in successors(&t) {
            let new_d = d + Δ;
            let is_shorter = dists.get(&nbr).map_or(true, |&curr_d| new_d < curr_d);
            if is_shorter {
                dists.insert(nbr, new_d);
                q.push(Reverse(DistWrap { t: nbr, d: new_d }));
            }
        }
    }

    dists.get(&target).copied()
}
