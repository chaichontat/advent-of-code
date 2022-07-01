#%%
import io

import numpy as np

from utils import read

#%%
raw = read("../../data/2021/day04.txt", raw=True).split("\n\n")
keys = np.fromiter(raw[0].split(","), dtype=np.uint8)
bingos = np.array([np.genfromtxt(io.StringIO(r), dtype=np.uint8) for r in raw[1:]], dtype=np.uint8)

# %% Part 1


def check_bingo(c: np.ndarray) -> np.ndarray | None:
    u = c.all(axis=1) | c.all(axis=2)
    res = np.argwhere(u)
    if not res.size:
        return None
    return res[0]


def get_unchecked(checked: np.ndarray, idx: int, k: int) -> np.ndarray:
    return np.sum(bingos[idx][~checked[idx]]) * k


checked = np.zeros_like(bingos, dtype=bool)
for k in keys:
    checked += bingos == k
    c = check_bingo(checked)
    if c is not None:
        print(get_unchecked(checked, c[0], k))
        break

# %% Part 2


def check_bingo2(c: np.ndarray) -> np.ndarray | None:
    u = c.all(axis=1) | c.all(axis=2)
    u = (~u).all(axis=1)
    res = np.argwhere(u)
    if not res.size:
        return None
    return res[0]


def part2():
    checked = np.ones_like(bingos, dtype=bool)
    for k in reversed(keys):
        checked &= ~(bingos == k)
        c = check_bingo2(checked)
        if c is not None:
            checked += bingos == k
            return get_unchecked(checked, c[0], k)


part2()
# %%
