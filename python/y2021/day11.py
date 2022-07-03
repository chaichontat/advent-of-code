#%%
import numpy as np

from utils import read

parse = lambda x: np.fromiter(x, dtype=np.int8)
raw = np.array(read("../../data/2021/day11.txt", parse=parse))
ex = """5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
""".splitlines()

# %%
def neighbors(i: int, j: int) -> tuple[slice, slice]:
    return np.s_[max(0, i - 1) : min(i + 2, m), max(0, j - 1) : min(j + 2, n)]


m, n = raw.shape
curr = raw.copy()
cnts = 0
part1 = 0
i = 0
while True:
    i += 1
    curr += 1
    flashold = flashnew = curr > 9
    for idx in np.argwhere(flashnew):
        curr[neighbors(*idx)] += 1
        curr[idx[0], idx[1]] -= 1

    while True:
        flashnew = curr > 9
        flash = (flashnew ^ flashold) & flashnew
        if not flash.any():
            break
        for idx in np.argwhere(flash):
            curr[neighbors(*idx)] += 1
            curr[idx[0], idx[1]] -= 1
        flashold = flashnew

    curr[f := curr > 9] = 0
    cnts += np.count_nonzero(f)
    if i == 100:
        print(part1 := cnts)
    if f.all():
        print(i)
        break
