#%%
from pathlib import Path


def get_active_rs():
    return [rs for rs in (path / "src").iterdir() if rs.name.startswith("day")]


path = Path(".")


sols = {
    1: (605364, 128397680),
    2: (447, 249),
    3: (265, 3154761400),
    4: (190, 121),
    5: (935, 743),
    6: (6521, 3305),
}


def gen_test(day: int, sols: tuple[int, int]) -> str:
    return f"""use super::utils::*;

pub fn part1(raw: &Vec<String>) -> usize {{0}}

pub fn part2(raw: &Vec<String>) -> usize {{0}}

#[cfg(test)]
mod tests {{
    use super::*;
    #[test]
    fn test1() {{
        assert_eq!(part1(&read("day{day:02d}.txt")), {sols[0]});
    }}

    #[test]
    fn test2() {{
        assert_eq!(part2(&read("day{day:02d}.txt")), {sols[1]});
    }}
}}"""


def gen_bench(day: int) -> str:
    return f"""
    c.bench_function("day{day:02d}a", |b| {{b.iter(|| day{day:02d}::part1(black_box(&read("day{day:02d}.txt"))))}});
    c.bench_function("day{day:02d}b", |b| {{b.iter(|| day{day:02d}::part2(black_box(&read("day{day:02d}.txt"))))}});"""


def check_tests(active):
    for rs in active:
        if "#[cfg(test)]" not in rs.read_text().split("\n"):
            day = int(rs.name[3:5])
            rs.write_text(rs.read_text() + gen_test(day, sols[day]))


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
    (path / "src" / "lib.rs").write_text("pub mod utils;\n\n" + res)


# %%
active = get_active_rs()
days = sorted(int(rs.name[3:5]) for rs in active)
check_tests(active)
gen_bench_file(days)
gen_lib(days)
# %%
