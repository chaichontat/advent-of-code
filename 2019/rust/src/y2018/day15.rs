use std::arch::x86_64::{__m256i, _mm256_loadu_si256};
use std::convert::TryFrom;
use std::fmt::Display;
use std::ops::{BitAnd, Not};
use std::str::from_utf8;

use bitvec::prelude::*;
use enum_map::{enum_map, Enum, EnumMap};
use itertools::{izip, Itertools};
use num::Integer;
use safe_arch::*;

const DIM: usize = 32;
const DIM_SHOW: usize = 7;

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
        // let out = bitarr![Msb0, u8; 0, 32]

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
        debug_assert!(pos < 1024);

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
            if mask != 0 {
                let idx = mask.trailing_zeros();
                unsafe {
                    return Some(
                        256 * i as u16
                            + 8 * idx as u16
                            + <[u8; 32]>::from(*x).get_unchecked(idx as usize).trailing_zeros() as u16,
                    );
                }
            }
        }
        None
    }

    fn swap(&mut self, pos: u16, pos_new: u16) {
        self.toggle(pos);
        self.toggle(pos_new);
    }

    fn toggle(&mut self, pos: u16) {
        let ptr = self.m.as_mut_ptr() as *mut i32;
        let (r, c) = pos.div_rem(&(DIM as u16));
        unsafe {
            *ptr.add(r as usize) ^= 1 << c;
        }
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
enum Id {
    Elf,
    Gob,
}

impl Not for Id {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Id::Elf => Id::Gob,
            Id::Gob => Id::Elf,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Unit {
    id:  Id,
    hp:  u8,
    pos: u16,
}

#[derive(Debug, Clone)]
pub struct Game {
    maps:     EnumMap<Id, Map>,
    cnts:     EnumMap<Id, u8>,
    can_walk: Map,
    idxs:     Vec<usize>,
    units:    Vec<Unit>,
    pos_idx:  [u16; DIM * DIM],
}

pub fn parse(raw: &str) -> Game {
    let raw = raw.split('\n').collect_vec();
    let len = raw[0].len();
    let raw = raw
        .iter()
        .map(|&x| format!("{}{}", x, (0..32 - len).map(|_| "#").collect::<String>()))
        .collect_vec();
    let raw = raw.concat();
    let raw = format!("{}{}", raw, (0..32 * 32 - len).map(|_| "#").collect::<String>());

    let raw = raw
        .as_bytes()
        .iter()
        .filter(|&&x| x != b'\n')
        .copied()
        .collect_vec();

    let maps = enum_map! {
        Id::Gob     => Map::new(&raw, b'G'),
        Id::Elf     => Map::new(&raw, b'E'),
    };

    let mut cnts = enum_map! {
        Id::Gob     => 0u8,
        Id::Elf     => 0,
    };

    let can_walk = Map::new(&raw, b'.');

    let mut units = Vec::new();
    // let mut hp = Vec::new();
    // let mut pos = Vec::new();
    let mut idxs = Vec::new();
    let mut pos_idx = [u16::MAX; DIM * DIM];

    for (p, x) in IntoIterator::into_iter(raw).enumerate() {
        let id = match x {
            b'E' => Id::Elf,
            b'G' => Id::Gob,
            _ => continue,
        };
        let n = units.len() as u16;
        pos_idx[p] = n;
        units.push(Unit { hp: 200, pos: p as u16, id });
        // hp.push(200);
        // pos.push(p as u16);
        // ids.push(id);
        cnts[id] += 1;
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

pub fn combi(game: &Game) -> (u32, u32) {
    let mut game = game.to_owned();
    let dp = 3;
    let mut round = 0;

    'outer: loop {
        let db = &game.units;
        game.idxs.sort_unstable_by_key(|&i| db[i].pos);
        while game.units[*game.idxs.last().unwrap() as usize].pos == u16::MAX {
            game.idxs.pop();
        }

        for &i in &game.idxs {
            let mut u = game.units[i];
            if u.hp == 0 {
                continue; // Killed.
            }

            let u_map = Map::from_pos(u.pos);
            let mut enemy_adj = game.maps[!u.id]._adjacent();

            if (&u_map & &enemy_adj).is_zero() {
                enemy_adj = &enemy_adj & &game.can_walk;
                if let Some(pos_new) = game.move_it(&u_map, &enemy_adj) {
                    game.pos_idx.swap(u.pos as usize, pos_new as usize);
                    game.maps[u.id].swap(u.pos, pos_new);
                    game.can_walk.swap(pos_new, u.pos);
                    u.pos = pos_new;
                }
            }

            // Attack
            // Find adjacent.
            let adjs = [
                u.pos.wrapping_sub(DIM as u16),
                u.pos + DIM as u16,
                u.pos - 1,
                u.pos + 1,
            ];
            if let Some((idx, pos, mut tg)) = adjs
                .iter()
                .filter_map(|&p| {
                    if p < 1024 && game.pos_idx[p as usize] < u16::MAX {
                        let idx = game.pos_idx[p as usize];
                        let target = game.units[idx as usize];
                        if target.id != u.id {
                            return Some((idx, p, target));
                        }
                    }
                    None
                })
                .min_by_key(|&(_, _, u)| u.hp)
            {
                if tg.hp > dp {
                    tg.hp -= dp;
                } else {
                    // Die.
                    tg.hp = 0;
                    tg.pos = u16::MAX;
                    game.pos_idx[pos as usize] = u16::MAX;
                    game.maps[!u.id].toggle(pos);
                    game.can_walk.toggle(pos);
                    game.cnts[!u.id] -= 1;
                }
                game.units[idx as usize] = tg;
                if game.cnts[!u.id] == 0 {
                    break 'outer;
                }
            }
            game.units[i] = u;
        }
        round += 1;
        // game.show();
        // printt(&game.hp);
    }
    // printt(&round);
    // printt(&game.units.iter().map(|&x| x.hp as u32).sum::<u32>());
    (round * game.units.iter().map(|&x| x.hp as u32).sum::<u32>(), 0)
}

impl Game {
    fn show(&self) {
        let mut out = [b'#'; 1024];
        let elf: &Map = &self.maps[Id::Elf];
        for (x, offset) in elf.m.iter().zip([0, 256, 512, 768]) {
            let bits: BitArray<Lsb0, _> = BitArray::new(<[u8; 32]>::from(*x));
            for b in bits.iter_ones() {
                out[offset + b] = b'E';
            }
        }

        let gob: &Map = &self.maps[Id::Gob];
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

    fn move_it(&self, ori: &Map, e_adj: &Map) -> Option<u16> {
        // BFS until overlap with adjacent to enemy.
        if let Some(picked) = self.bfs(ori, e_adj) {
            let ori_adj = self.adjacent(ori);
            return self.bfs(&Map::from_pos(picked), &ori_adj);
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read;

    #[test]
    fn test_combi() {
        assert_eq!(combi(&parse(&read(2018, "day15.txt"))), (213692, 1033));
    }
}
