use std::cmp::Ordering;

use ahash::AHashMap;

type Name = String;
type Db = AHashMap<Name, Recipe>;

#[derive(Debug, Clone)]
struct NName {
    n:    usize,
    name: Name,
}

#[derive(Debug, Clone)]
struct Recipe {
    ins:   Vec<NName>,
    out:   NName,
    dfsed: bool,
}

fn parse(raw: &[String]) -> Db {
    let mut db = AHashMap::with_capacity(100);
    for line in raw.iter() {
        let mut ins = Vec::new();
        let mut n = 0_usize;
        let mut name = String::new();

        for s in line.chars() {
            if '0' <= s && s <= '9' {
                let c = s as usize - '0' as usize;
                n = 10 * n + c; // Build dec number.
            } else if 'A' <= s && s <= 'Z' {
                name.push(s); // Append name to building string.
            } else if s == ',' || s == '=' {
                // End of name, build struct, and reset.
                ins.push(NName {
                    n,
                    name: name.to_owned(),
                });
                n = 0;
                name.clear();
            } else if s == ' ' || s == '>' {
            } else {
                unreachable!(); // Safety check.
            }
        }
        let out = NName {
            n,
            name: name.to_owned(),
        };

        db.insert(out.name.to_owned(), Recipe {
            ins,
            out,
            dfsed: false,
        });
    }
    db
}

fn topo_sort(db: &mut Db, top: &str) -> Vec<Name> {
    fn inner(sorted: &mut Vec<Name>, db: &mut Db, name: Name) {
        if let Some(mut r) = db.get_mut(&name) {
            if r.dfsed {
                return;
            }
            r.dfsed = true;

            let r = r.ins.clone();
            for p in r.iter() {
                inner(sorted, db, p.name.to_owned())
            }
            sorted.push(name);
        }
    }

    let mut sorted = Vec::<Name>::new();
    inner(&mut sorted, db, top.to_string());
    sorted.reverse();
    sorted
}

#[derive(Debug, Clone, Copy)]
struct OreFuel {
    fuel: usize,
    ore:  usize,
}

struct Cost {
    sorted: Vec<Name>,
    db:     Db,
    goal:   String,
}

impl Cost {
    fn cost(&self, fuel: usize) -> OreFuel {
        let mut need = AHashMap::new();
        need.insert(&self.sorted[0], fuel);
        for chem in self.sorted.iter() {
            if chem == &self.goal {
                break;
            }
            let rec = self.db.get(chem).unwrap();
            let n = (need.get(chem).unwrap() + rec.out.n - 1) / rec.out.n; // Round-up
            for nname in rec.ins.iter() {
                need.entry(&nname.name)
                    .and_modify(|v| *v += n * nname.n)
                    .or_insert(n * nname.n);
            }
        }

        OreFuel {
            fuel,
            ore: *need.get(&self.goal).unwrap(),
        }
    }
}

fn binary_search(mut lo: OreFuel, mut hi: OreFuel, avail: usize, cost: &Cost) -> OreFuel {
    while lo.fuel < hi.fuel - 1 {
        let deriv = (avail as f64 - lo.ore as f64) / (hi.ore as f64 - lo.ore as f64);

        // Estimate and constrain target.
        let mut fuel_est = lo.fuel + (deriv * (hi.fuel - lo.fuel) as f64) as usize;
        fuel_est = fuel_est.min(hi.fuel - 1);
        fuel_est = fuel_est.max(lo.fuel + 1);

        let curr = cost.cost(fuel_est);
        match curr.ore.cmp(&avail) {
            Ordering::Equal | Ordering::Less => lo = curr,
            Ordering::Greater => hi = curr,
        }
    }
    lo
}

pub fn run(raw: &[String]) -> (usize, usize) {
    // Part 1
    let mut db = parse(raw);
    let sorted = topo_sort(&mut db, &"FUEL");
    let cost = Cost {
        sorted,
        db,
        goal: "ORE".to_string(),
    };
    let cost_per_fuel = cost.cost(1);

    // Part 2: Get lower and upper bound then binary search.
    // Setting upper bound to be 4× the linear approximation.
    let avail = 1e12 as usize;
    let lo = cost_per_fuel;
    let hi = cost.cost(4 * avail / cost_per_fuel.ore);

    let max_with_avail = binary_search(lo, hi, avail, &cost);
    (cost_per_fuel.ore, max_with_avail.fuel)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::*;
    #[test]
    fn test1() {
        assert_eq!(run(&read("day14.txt")), (892207, 1935265));
    }
}
