#%%
import numpy as np
from scipy import signal

from utils import read

raw = read("../../data/2021/day20.txt")
ex = """..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###
""".splitlines()

# raw = ex

alg = np.fromiter(map(lambda x: x == "#", raw[0]), dtype=int)
img = np.array([np.fromiter(map(lambda x: x == "#", r), dtype=int) for r in raw[2:]])
# %%
mat = np.array(
    [
        [256, 128, 64],
        [32, 16, 8],
        [4, 2, 1],
    ],
    dtype=int,
)


def enhance(arr: np.ndarray, i: int) -> np.ndarray:
    if i % 2 == 1 and alg[0]:
        t = signal.correlate2d(arr, mat, fillvalue=1).astype(int)
    else:
        t = signal.correlate2d(arr, mat).astype(int)
    return alg[t]


out = img
for i in range(50):
    out = enhance(out, i)
np.sum(out)

# %%
