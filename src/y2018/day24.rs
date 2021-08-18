use std::cmp::Reverse;
use std::fmt::Debug;
use std::mem;
use std::str::{from_utf8, FromStr};

use itertools::Itertools;
use rayon::prelude::*;
use regex::Regex;
use safe_arch::bit_lowest_set_reset_u32;
use strum_macros::EnumString;

const N_UNITS: usize = 20;
const N_ATTACKS: usize = 5; // Modes of attack.

const IMM: bool = false;
const VIR: bool = true;

type Bits = u32;
type Team = bool;

#[allow(non_camel_case_types)]
#[derive(Debug, EnumString, Clone, Copy)]
enum Attack {
    cold        = 0,
    radiation   = 1,
    slashing    = 2,
    bludgeoning = 3,
    fire        = 4,
}

impl Default for Attack {
    fn default() -> Self {
        Attack::cold
    }
}

// Initiative is unique.
#[derive(Clone, Copy, Default)]
pub struct Group {
    team:       Team,
    units:      u32,
    initiative: u8,
    target:     Option<u8>,
    attack:     Attack,
    hp:         u32,
    dp:         u32,
    ep:         u32,
    weak:       Bits,
    immune:     Bits,
}

impl Debug for Group {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}: Group {} contains {:4} units",
            self.team, self.initiative, self.units
        )
    }
}

impl Group {
    pub fn calc_ep(&self) -> u32 {
        self.units * self.dp
    }

    pub fn set_units(&mut self, units: u32) {
        if self.units == units {
            return;
        }
        debug_assert!(units < self.units);
        self.units = units;
        self.ep = self.calc_ep();
    }

    pub fn set_dp(&mut self, dp: u32) {
        self.dp = dp;
        self.ep = self.calc_ep();
    }

    pub fn set_target(&mut self, target: Option<u8>) {
        self.target = target;
    }
}

type Groups = [Group; N_UNITS];

#[repr(align(32))]
#[derive(Clone, Copy)]
pub struct Universe {
    groups:     Groups,
    weak:       [Bits; N_ATTACKS],
    not_immune: [Bits; N_ATTACKS],
    select_ord: [usize; N_UNITS],
    alive:      [Bits; 2],
}

impl Universe {
    fn new(mut groups: Groups) -> Self {
        let mut not_immune = [(0..N_UNITS).fold(0, |acc, i| acc | 1 << i); N_ATTACKS];
        let mut weak = [0; N_ATTACKS];
        let mut alive = [0; 2];

        // Sorted by ascending initiative. (init is flipped).
        groups.sort_unstable_by_key(|g| g.initiative);

        for mut g in groups {
            while g.immune > 0 {
                not_immune[g.immune.trailing_zeros() as usize] &= !(1 << g.initiative);
                g.immune = bit_lowest_set_reset_u32(g.immune);
            }
            while g.weak > 0 {
                weak[g.weak.trailing_zeros() as usize] |= 1 << g.initiative;
                g.weak = bit_lowest_set_reset_u32(g.weak);
            }

            alive[g.team as usize] |= 1 << g.initiative
        }

        let mut select_ord = [0; N_UNITS];
        select_ord.iter_mut().enumerate().for_each(|(i, x)| *x = i); // Just filling with (0..N_UNITS).
        select_ord.sort_unstable_by_key(|i| fn_select_ord(&groups, i));

        Universe {
            groups,
            weak,
            not_immune,
            select_ord,
            alive,
        }
    }

    fn sum(&self) -> u32 {
        self.groups.iter().map(|g| g.units).sum()
    }

    fn set_units(&mut self, initiative: u8, units: u32) {
        if units == 0 {
            self.alive[self.groups[initiative as usize].team as usize] &= !(1 << initiative);
        }
        self.groups.get_mut(initiative as usize).unwrap().set_units(units);
        self.rearrange_select_order(initiative);
        debug_assert!(self.is_sound());
    }

    fn rearrange_select_order(&mut self, changed: u8) {
        let defender = self.groups[changed as usize];
        let mut i = self
            .select_ord
            .iter()
            .find_position(|&&ini| ini == defender.initiative as usize)
            .unwrap()
            .0;

        while i + 1 < N_UNITS
            && (defender.ep < self.groups[self.select_ord[i + 1]].ep
                || (defender.ep == self.groups[self.select_ord[i + 1]].ep
                    && defender.initiative as usize > self.select_ord[i + 1]))
        {
            let (fst, snd) = self.select_ord.split_at_mut(i + 1);
            mem::swap(&mut fst[i], &mut snd[0]);
            i += 1;
        }

        debug_assert!(self.is_sound());
    }

    fn sort_select_order(&mut self) {
        let mut temp = self.select_ord;
        temp.sort_unstable_by_key(|i| fn_select_ord(&self.groups, i));
        self.select_ord = temp;
        debug_assert!(self.is_sound());
    }

    fn boost_immunity(&self, val: u32) -> Self {
        let mut out = *self;
        let mut im = out.alive[IMM as usize];
        while im > 0 {
            let old_dp = out.groups[im.trailing_zeros() as usize].dp;
            out.groups[im.trailing_zeros() as usize].set_dp(old_dp + val);
            im = bit_lowest_set_reset_u32(im);
        }
        out.sort_select_order();
        debug_assert!(out.is_sound());
        out
    }

    /// Check if invariants within the struct is broken.
    /// Check if selection order is sorted, effective power = units * dp, consistent alive state.
    fn is_sound(&self) -> bool {
        assert!(self
            .select_ord
            .is_sorted_by_key(|x| fn_select_ord(&self.groups, x)));
        assert!(self.groups.iter().all(|&g| g.ep == g.calc_ep()));
        let alive = self.alive[0] | self.alive[1];
        for (u, g) in self.groups.iter().enumerate() {
            if alive & (1 << u) > 0 {
                assert!(g.units > 0);
            } else {
                assert!(g.units == 0);
            }
        }
        true
    }
}

fn fn_select_ord(groups: &[Group], i: &usize) -> (Reverse<u32>, u8) {
    (Reverse(groups[*i].ep), groups[*i].initiative)
}

fn miniparse(groups: &mut [Group], lines: &[String], team: Team) -> Option<bool> {
    let re = [
        Regex::new(r"(\d+)").unwrap(),
        Regex::new(r"does \d+ ([a-z]+)").unwrap(),
        Regex::new(r"\((.*)\)").unwrap(),
    ];
    debug_assert!(lines.len() == N_UNITS / 2);
    for (i, line) in lines.iter().enumerate() {
        let (n, hp, dp, initiative) = re[0]
            .find_iter(line)
            .map(|m| m.as_str().parse::<u32>().unwrap())
            .collect_tuple()?;

        let attack = Attack::from_str(re[1].captures(line)?.get(1)?.as_str()).unwrap();

        let (mut weak, mut immune) = (0, 0);
        if let Some(wi) = re[2].captures(line) {
            // Have parenthesis.
            let wi = wi.get(1)?;
            for y in wi.as_str().split("; ") {
                if y.starts_with("weak to ") {
                    weak = from_utf8(&y.as_bytes()[8..])
                        .unwrap()
                        .split(", ")
                        .map(|a| 1 << Attack::from_str(a).unwrap() as Bits)
                        .sum();
                } else if y.starts_with("immune to ") {
                    immune = from_utf8(&y.as_bytes()[10..])
                        .unwrap()
                        .split(", ")
                        .map(|a| 1 << Attack::from_str(a).unwrap() as Bits)
                        .sum();
                }
            }
        }
        groups[i] = Group {
            units: n,
            hp,
            attack,
            dp,
            ep: n * dp,
            initiative: N_UNITS as u8 - initiative as u8,
            weak,
            immune,
            target: None,
            team,
        };
    }
    Some(true)
}

pub fn parse(raw: &str) -> Universe {
    let raw2 = raw
        .split("\n\n")
        .map(|s| s.split('\n').skip(1).map(String::from).collect_vec())
        .collect_vec();

    let mut groups = [Group::default(); N_UNITS];
    miniparse(&mut groups[..N_UNITS / 2], &raw2[0][..], IMM);
    miniparse(&mut groups[N_UNITS / 2..], &raw2[1][..], VIR);

    Universe::new(groups)
}

fn choose_target(uni: &Universe) -> (Groups, Bits) {
    debug_assert!(uni.is_sound());
    let mut groups = uni.groups;

    let mut attacking: Bits = 0;
    let mut attackable = uni.alive;

    for i in uni.select_ord {
        if (uni.alive[0] | uni.alive[1]) & (1 << i) == 0 {
            continue;
        }

        let g = &groups[i];
        let mut cands: Bits = attackable[!g.team as usize];
        cands &= if (cands & uni.weak[g.attack as usize]) > 0 {
            uni.weak[g.attack as usize]
        } else {
            uni.not_immune[g.attack as usize]
        };

        let mut target = None;
        let mut target_max_ep = 0;
        while cands != 0 {
            let idx = cands.trailing_zeros() as usize;
            if uni.groups[idx].ep > target_max_ep {
                target_max_ep = uni.groups[idx].ep;
                target = Some(idx as u8);
            }
            cands = bit_lowest_set_reset_u32(cands);
        }

        if let Some(target) = target {
            attacking |= 1 << g.initiative;
            attackable[!g.team as usize] ^= 1 << target;
        }

        groups[i].set_target(target);
    }

    (groups, attacking)
}

fn run(mut uni: Universe) -> Option<Universe> {
    while uni.alive[0] > 0 && uni.alive[1] > 0 {
        let (groups, mut attacking) = choose_target(&uni);
        uni.groups = groups;

        let mut is_infinite_loop = true;
        while attacking != 0 {
            let idx = attacking.trailing_zeros();
            attacking = bit_lowest_set_reset_u32(attacking); // Pop

            let attacker = uni.groups[idx as usize];
            let temp_defender = uni.groups[attacker.target.unwrap() as usize];

            debug_assert!(attacker.units > 0);
            debug_assert!(temp_defender.units > 0);

            let real_ep = if (1 << attacker.attack as u32) & temp_defender.weak != 0 {
                2 * attacker.ep
            } else {
                attacker.ep
            };

            let old_units = temp_defender.units;
            uni.set_units(
                attacker.target.unwrap(),
                temp_defender.units.saturating_sub(real_ep / temp_defender.hp),
            );

            let defender = &uni.groups[attacker.target.unwrap() as usize];
            if old_units != defender.units {
                is_infinite_loop = false;
                if defender.units == 0 {
                    attacking &= !(1 << defender.initiative);
                }
            }

            debug_assert!(uni.is_sound());
        }
        if is_infinite_loop {
            return None;
        }
    }
    debug_assert!((uni.alive[0] == 0) ^ (uni.alive[1] == 0));
    Some(uni)
}

pub fn combi(parsed: &Universe) -> (u32, u32) {
    let ori = *parsed;
    let part1 = run(ori).unwrap().sum();

    let part2 = (1..40)
        .into_par_iter()
        .find_map_first(|inc| {
            let boosted = ori.boost_immunity(inc);
            if let Some(res) = run(boosted) {
                if res.alive[IMM as usize] > 0 {
                    return Some(res.sum());
                }
            }
            None
        })
        .unwrap();

    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read;

    #[test]
    fn test_part1() {
        assert_eq!(combi(&parse(&read(2018, "day24.txt"))), (26277, 8812));
    }
}
