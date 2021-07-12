#%%
from pathlib import Path


def get_active_rs():
    return [rs for rs in (path / "src").iterdir() if rs.name.startswith("day")]


path = Path(".")


sols = {
    1: (3184233, 4773483),
    2: (5110675, 4847),
    3: (217, 3454),
    4: (1625, 1111),
    5: (15426686, 11430197),
    6: (402879, 484),
    7: (77500, 22476942),
    8: (1950, 0),  # FKAHL,
    9: (2789104029, 32869),
    10: (269, 612),
    11: (2082, 0),  # FARBCFJK
    12: (8287, 528250271633772),
    13: (180, 8777),
    14: (892207, 1935265),
    15: (216, 326),
    16: (27229269, 26857164),
}


def gen_test(day: int) -> str:
    sol = sols.get(day, (0, 0))
    return f"""use super::utils::*;

pub fn part1(raw: &[String]) -> usize {
    0
}

pub fn part2(raw: &[String]) -> usize {
    0
}

#[cfg(test)]
mod tests {{
    use super::*;
    #[test]
    fn test1() {{
        assert_eq!(part1(&read("day{day:02d}.txt")), {sol[0]});
    }}
    
    #[test]
    fn test2() {{
        assert_eq!(part2(&read("day{day:02d}.txt")), {sol[1]});
    }}

}}"""


def gen_bench(day: int) -> str:
    return f"""
    let day{day:02d} = &read("day{day:02d}.txt");
    c.bench_function("day{day:02d}a", |b| {{b.iter(|| day{day:02d}::part1(black_box(day{day:02d})))}});
    c.bench_function("day{day:02d}b", |b| {{b.iter(|| day{day:02d}::part2(black_box(day{day:02d})))}});
    """


def check_tests(active):
    for rs in active:
        if "#[cfg(test)]" not in rs.read_text().split("\n"):
            day = int(rs.name[3:5])
            rs.write_text(rs.read_text() + gen_test(day))


def gen_bench_file(days):
    res = map(gen_bench, days)
    (path / "benches" / "bench.rs").write_text(
        f"""extern crate advent_of_code_2020;

use advent_of_code_2020::utils::*;
use advent_of_code_2020::*;

use criterion::{{black_box, criterion_group, criterion_main, Criterion}};

fn bench_fi(c: &mut Criterion) {{{"".join(res)}
}}

criterion_group!(benches, bench_fi);
criterion_main!(benches);
"""
    )


def gen_lib(days):
    res = "\n".join([f"pub mod day{d:02d};" for d in days])
    (path / "src" / "lib.rs").write_text(
        """#[macro_use]
extern crate num_derive;

pub mod intcode;
pub mod utils;\n\n"""
        + res
    )


active = get_active_rs()
days = sorted(int(rs.name[3:5]) for rs in active)
check_tests(active)
gen_bench_file(days)
gen_lib(days)
# %%
