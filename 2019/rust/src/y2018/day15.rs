use std::arch::x86_64::{__m256i, _mm256_loadu_si256};
use std::convert::TryFrom;
use std::fmt::Display;
use std::ops::{BitAnd, Not};
use std::str::from_utf8;
use std::thread;

use bitvec::prelude::*;
use enum_map::{enum_map, Enum, EnumMap};
use itertools::{izip, Itertools};
use num::Integer;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use safe_arch::*;

const DIM: usize = 32;
const DIM_SHOW: usize = 32;

#[repr(C, align(32))]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Map {
    padf: m256i,
    m:    [m256i; 4],
    padb: u32,
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = [b'.'; 1024];
        for (x, offset) in self.m.iter().zip([0, 256, 512, 768]) {
            let bits: BitArray<Lsb0, _> = BitArray::new(<[u8; 32]>::from(*x));
            for b in bits.iter_ones() {
                out[offset + b] = b'#';
            }
        }
        let s = out
            .chunks_exact(32)
            .take(DIM_SHOW)
            .map(|x| from_utf8(&x[..DIM_SHOW]))
            .flatten()
            .join("\n");
        write!(f, "{}", s)
    }
}

impl Map {
    fn new(seq: &[u8], c: u8) -> Self {
        let mask = set_splat_i8_m256i(c as i8);
        let mut ms = [m256i::default(); 4];
        for (m, chunk) in ms.iter_mut().zip(seq.chunks_exact(32 * 8)) {
            let mut packed = [0i32; 8];
            for (p, c) in packed.iter_mut().zip(chunk.chunks_exact(32)) {
                *p = move_mask_i8_m256i(cmp_eq_mask_i8_m256i(
                    m256i::from(*<&[u8; 32]>::try_from(c).unwrap()),
                    mask,
                ));
            }
            *m = m256i::from(packed);
        }
        Map { m: ms, ..Default::default() }
    }

    fn from_pos(pos: u16) -> Self {
        debug_assert!(pos < (DIM * DIM) as u16);

        let (which_vec, r) = pos.div_rem(&256);
        let (which_i32, r) = r.div_rem(&32);

        let mut bits = [0u32; 8];
        bits[which_i32 as usize] |= 1 << r as u32;

        let mut out = [m256i::default(); 4];
        out[which_vec as usize] = m256i::from(bits);
        Map { m: out, ..Default::default() }
    }

    #[allow(clippy::many_single_char_names)]
    fn _adjacent(&self) -> Self {
        let mut m = [m256i::default(); 4];

        for (new, cur) in m.iter_mut().zip(&self.m) {
            let l = shl_imm_u32_m256i::<1>(*cur);
            let r = shr_imm_u32_m256i::<1>(*cur);
            let d = m256i(unsafe {
                _mm256_loadu_si256((cur as *const m256i as *const u32).sub(1) as *const __m256i)
            });
            let u = m256i(unsafe {
                _mm256_loadu_si256((cur as *const m256i as *const u32).add(1) as *const __m256i)
            });

            let combi = bitor_m256i(bitor_m256i(l, r), bitor_m256i(u, d));
            *new = bitor_m256i(combi, *cur);
        }

        Map { m, ..Default::default() }
    }

    fn is_zero(&self) -> bool {
        self.m
            .iter()
            .all(|&mm| !move_mask_i8_m256i(cmp_eq_mask_i8_m256i(mm, zeroed_m256i())) == 0)
    }

    fn get_first(&self) -> Option<u16> {
        for (i, x) in self.m.iter().enumerate() {
            let mask = !move_mask_i8_m256i(cmp_eq_mask_i8_m256i(*x, zeroed_m256i()));
            if mask == 0 {
                continue;
            }
            let idx = mask.trailing_zeros();
            return Some(
                256 * i as u16
                    + 8 * idx as u16
                    + unsafe { <[u8; 32]>::from(*x).get_unchecked(idx as usize).trailing_zeros() as u16 }, // Always safe.
            );
        }
        None
    }

    unsafe fn swap(&mut self, pos: u16, pos_new: u16) {
        self.toggle(pos);
        self.toggle(pos_new);
    }

    unsafe fn toggle(&mut self, pos: u16) {
        debug_assert!(pos < (DIM * DIM) as u16);
        let ptr = self.m.as_mut_ptr() as *mut i32;
        let (r, c) = pos.div_rem(&(DIM as u16));
        *ptr.add(r as usize) ^= 1 << c;
    }
}

impl BitAnd for &Map {
    type Output = Map;

    fn bitand(self, rhs: Self) -> Self::Output {
        let mut m = [m256i::default(); 4];
        for (o, x, y) in izip!(m.iter_mut(), self.m.iter(), rhs.m.iter()) {
            *o = bitand_m256i(load_m256i(x), load_m256i(y));
        }
        Map { m, ..Default::default() }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Enum)]
enum Team {
    Elf,
    Gob,
}

impl Not for Team {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Team::Elf => Team::Gob,
            Team::Gob => Team::Elf,
        }
    }
}

type Idx = u16;
type Pos = u16;

#[derive(Debug, Clone, Copy)]
pub struct Unit {
    team: Team,
    hp:   u8,
    pos:  Option<Pos>, // Double as marker for dead/alive when sorting by pos.
}

#[derive(Debug, Clone)]
pub struct Game {
    maps:     EnumMap<Team, Map>,
    cnts:     EnumMap<Team, u8>,
    can_walk: Map,
    idxs:     Vec<usize>,
    units:    Vec<Unit>,
    pos_idx:  [Option<Idx>; DIM * DIM],
}

pub fn parse(raw: &str) -> Game {
    // Pad in case len  32.
    let raw = raw.split('\n').collect_vec();
    let len = raw[0].len();
    let raw = raw
        .iter()
        .map(|&x| format!("{}{}", x, (0..32 - len).map(|_| "#").collect::<String>()))
        .collect_vec();
    let raw = format!(
        "{}{}",
        raw.concat(),
        (0..32 * 32 - len).map(|_| "#").collect::<String>()
    );

    let raw = raw
        .as_bytes()
        .iter()
        .filter(|&&x| x != b'\n')
        .copied()
        .collect_vec();

    let maps = enum_map! {
        Team::Gob     => Map::new(&raw, b'G'),
        Team::Elf     => Map::new(&raw, b'E'),
    };
    let can_walk = Map::new(&raw, b'.');

    let mut cnts = enum_map! {
        Team::Gob     => 0u8,
        Team::Elf     => 0,
    };

    let mut units = Vec::new();
    let mut idxs = Vec::new();
    let mut pos_idx = [None; DIM * DIM];

    for (pos, x) in IntoIterator::into_iter(raw).enumerate() {
        let team = match x {
            b'E' => Team::Elf,
            b'G' => Team::Gob,
            _ => continue,
        };
        pos_idx[pos] = Some(units.len() as Idx);
        cnts[team] += 1;
        units.push(Unit { hp: 200, pos: Some(pos as Pos), team });
    }

    (0..units.len()).for_each(|x| idxs.push(x));

    Game {
        maps,
        can_walk,
        cnts,
        idxs,
        units,
        pos_idx,
    }
}

#[derive(PartialEq)]
pub enum Directive {
    StopWhenElfDies,
    Meh,
}

/// # Safety
/// See `combi` below.
unsafe fn run(game: &Game, elf_dp: u8, mode: Directive) -> Option<(u16, Vec<Unit>)> {
    let mut game = game.to_owned();
    let mut round = 0u16;

    loop {
        game.unqueue_dead();
        for j in 0..game.idxs.len() {
            let idx = *game.idxs.get_unchecked(j);
            let mut u = *game.units.get_unchecked(idx);

            if u.hp == 0 {
                continue; // Killed.
            }
            let u_map = Map::from_pos(u.pos.unwrap());
            // Only place that we do NOT automatically remove unwalkable tiles.
            let mut enemy_adj = game.maps[!u.team]._adjacent();
            if (&u_map & &enemy_adj).is_zero() {
                enemy_adj = &enemy_adj & &game.can_walk; // Remove unwalkable tile.
                if let Some(pos_new) = game.move_it(&u_map, &enemy_adj) {
                    game.swap_pos(u, pos_new);
                    game.units.get_unchecked_mut(idx).pos = Some(pos_new);
                    u.pos = Some(pos_new);
                }
            }

            // Attack
            if let Some((idx_tg, tg)) = game.choose_attack_target(u) {
                let dp = if u.team == Team::Elf { elf_dp } else { 3 };
                if tg.hp > dp {
                    game.units.get_unchecked_mut(idx_tg as usize).hp -= dp;
                } else {
                    // Die.
                    game.units.get_unchecked_mut(idx_tg as usize).hp = 0;
                    if tg.team == Team::Elf && mode == Directive::StopWhenElfDies {
                        return None;
                    }
                    game.finish_it(u, idx_tg, tg);
                    if game.cnts[!u.team] == 0 {
                        let round = if j == game.idxs.len() - 1 { round + 1 } else { round };
                        return Some((round, game.units));
                    }
                }
            }
        }
        round += 1;
    }
}

impl Game {
    #[allow(dead_code)]
    fn show(&self, bold: Option<i16>) {
        let mut out = [b'#'; 1024];
        let elf: &Map = &self.maps[Team::Elf];
        for (x, offset) in elf.m.iter().zip([0, 256, 512, 768]) {
            let bits: BitArray<Lsb0, _> = BitArray::new(<[u8; 32]>::from(*x));
            for b in bits.iter_ones() {
                out[offset + b] = b'E';
            }
        }

        let gob: &Map = &self.maps[Team::Gob];
        for (x, offset) in gob.m.iter().zip([0, 256, 512, 768]) {
            let bits: BitArray<Lsb0, _> = BitArray::new(<[u8; 32]>::from(*x));
            for b in bits.iter_ones() {
                debug_assert!(out[offset + b] == b'#');
                out[offset + b] = b'G';
            }
        }

        let gob: &Map = &self.can_walk;
        for (x, offset) in gob.m.iter().zip([0, 256, 512, 768]) {
            let bits: BitArray<Lsb0, _> = BitArray::new(<[u8; 32]>::from(*x));
            for b in bits.iter_ones() {
                debug_assert!(out[offset + b] == b'#');
                out[offset + b] = b'.';
            }
        }

        if let Some(tb) = bold {
            out[tb as usize] = b'X';
        }

        let s = out
            .chunks_exact(32)
            .take(DIM_SHOW)
            .map(|x| from_utf8(&x[..DIM_SHOW]))
            .flatten()
            .join("\n");
        println!("{}", s);
    }

    fn bfs(&self, start: &Map, target_adj: &Map) -> Option<u16> {
        let mut path = start.clone();

        let mut overlap;
        loop {
            let new_path = &path._adjacent() & &self.can_walk;
            overlap = target_adj & &new_path;
            if !overlap.is_zero() {
                return overlap.get_first();
            }

            if new_path == path {
                return None;
            }
            path = new_path;
        }
    }

    fn adjacent(&self, map: &Map) -> Map {
        &map._adjacent() & &self.can_walk
    }

    unsafe fn swap_pos(&mut self, u: Unit, pos_new: Pos) {
        let upos = u.pos.unwrap();
        self.pos_idx.swap(upos as usize, pos_new as usize);
        self.maps[u.team].swap(upos, pos_new);
        self.can_walk.swap(pos_new, upos);
    }

    unsafe fn finish_it(&mut self, killer: Unit, idx_tg: Idx, mut tg: Unit) {
        tg.hp = 0;
        *self.pos_idx.get_unchecked_mut(tg.pos.unwrap() as usize) = None;

        self.maps[!killer.team].toggle(tg.pos.unwrap());
        self.can_walk.toggle(tg.pos.unwrap());

        tg.pos = None;
        self.cnts[!killer.team] -= 1;
        *self.units.get_unchecked_mut(idx_tg as usize) = tg;
    }

    fn move_it(&self, ori: &Map, e_adj: &Map) -> Option<Pos> {
        // BFS until overlap with adjacent to enemy.
        if let Some(picked) = self.bfs(ori, e_adj) {
            let ori_adj = self.adjacent(ori);
            return self.bfs(&Map::from_pos(picked), &ori_adj);
        }
        None
    }

    unsafe fn choose_attack_target(&self, u: Unit) -> Option<(Idx, Unit)> {
        let adjs = [
            u.pos.unwrap().wrapping_sub(DIM as u16),
            u.pos.unwrap() + DIM as u16,
            u.pos.unwrap().wrapping_sub(1),
            u.pos.unwrap() + 1,
        ];

        adjs.iter()
            .filter_map(|&p| {
                if p < 1024 && self.pos_idx.get_unchecked(p as usize).is_some() {
                    let idx = self.pos_idx.get_unchecked(p as usize).unwrap();
                    let target = *self.units.get_unchecked(idx as usize);
                    if target.team != u.team {
                        return Some((idx, target));
                    }
                }
                None
            })
            .min_by_key(|&(_, u)| (u.hp, u.pos))
    }

    unsafe fn unqueue_dead(&mut self) {
        let db = &self.units;
        self.idxs.sort_unstable_by_key(|&idx| db.get_unchecked(idx).pos);
        while self
            .units
            .get_unchecked(*self.idxs.last().unwrap() as usize)
            .pos
            .is_none()
        {
            self.idxs.pop();
        }
    }
}

fn score((round, units): (u16, Vec<Unit>)) -> u32 {
    round as u32 * units.iter().map(|&x| x.hp as u32).sum::<u32>()
}

/// # Safety
/// Game must be internally consistent.
/// No bounds-checking at indices with use of pointer arithmetics for shifting.
pub unsafe fn combi(game: &Game) -> (u32, u32) {
    // let game1 = game.to_owned();
    // let thr = thread::spawn(move || run(&game1, 3, Directive::Meh).unwrap());
    let part1 = run(game, 3, Directive::Meh).unwrap();
    let part2 = (4u8..50)
        .into_par_iter()
        .find_map_first(|dp| run(game, dp, Directive::StopWhenElfDies))
        .unwrap();

    // let part1 = thr.join().unwrap();

    (score(part1), score(part2))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read;

    #[test]
    fn test_combi() {
        unsafe { assert_eq!(combi(&parse(&read(2018, "day15.txt"))), (213692, 52688)) };
    }
}
