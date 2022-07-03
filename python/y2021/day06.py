#%%
import numpy as np

from utils import read

raw = list(map(int, read("../../data/2021/day06.txt")[0].split(",")))


def f(day: int) -> np.uint64:
    out = np.zeros(day + 1, dtype=np.uint64)
    ns = np.zeros(9, dtype=np.uint64)
    for n in raw:
        ns[n] += 1

    for i in range(9):
        out[i] = np.sum(ns)
        ns = np.roll(ns, -1)
        ns[6] += ns[8]

    for i in range(9, day + 1):
        out[i] = out[i - 7] + out[i - 9]

    return out[day]


print(f(80), f(256))

# %%
