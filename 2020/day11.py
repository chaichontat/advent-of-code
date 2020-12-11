#%%
import numpy as np
from scipy.signal import convolve2d

from utils import load

raw = load("day11.txt", split=None)

raw = raw.replace("#", "1,")
raw = raw.replace(".", "0,")
raw = raw.replace("L", "-1,")
arr = np.array([line[:-1].split(",") for line in raw.split("\n") if line], dtype=int)

# %%
def count_convolve(arr):
    return convolve2d(arr == 1, np.ones((3, 3)), mode="same", boundary="fill") - (arr == 1)


def step(arr, thr_survive=3, counter=count_convolve):
    nbrs_count = counter(arr)
    occ = ((arr == -1) & (nbrs_count == 0)) | ((arr == 1) & (nbrs_count <= thr_survive))
    return (-1 * (occ == 0) + occ) * np.abs(arr)


# %%
def calc_equilibrium(arr, **kwargs):
    arr = arr.copy()
    old_count = -1
    count = 0
    while count != old_count:
        old_count = count
        arr = step(arr, **kwargs)
        count = np.sum(arr == 1)
    return count


def test1():
    assert calc_equilibrium(arr) == 2329


# %% Part 2
h, w = arr.shape


def parse(arr, i, j):
    v = arr[i, j]
    if v == 0:
        return -1
    elif v == -1:
        return False
    elif v == 1:
        return True
    else:
        raise ValueError


def calc_neighbor_los(arr, r, c):  # Line of sight.
    nbs = {x: -1 for x in ["n", "ne", "e", "se", "s", "sw", "w", "nw"]}
    for i in reversed(range(0, r)):
        if nbs["n"] == -1:
            nbs["n"] = parse(arr, i, c)
        if nbs["nw"] == -1 and c + i - r >= 0:
            nbs["nw"] = parse(arr, i, c + i - r)
        if nbs["ne"] == -1 and c - i + r < arr.shape[1]:
            nbs["ne"] = parse(arr, i, c - i + r)

    for i in range(r + 1, arr.shape[0]):
        if nbs["s"] == -1:
            nbs["s"] = parse(arr, i, c)
        if nbs["sw"] == -1 and c - i + r >= 0:
            nbs["sw"] = parse(arr, i, c - i + r)
        if nbs["se"] == -1 and c + i - r < arr.shape[1]:
            nbs["se"] = parse(arr, i, c + i - r)

    for j in reversed(range(0, c)):
        if nbs["w"] == -1:
            nbs["w"] = parse(arr, r, j)
        else:
            break

    for j in range(c + 1, arr.shape[1]):
        if nbs["e"] == -1:
            nbs["e"] = parse(arr, r, j)
        else:
            break

    return sum([x for x in nbs.values() if x == True])


def count_see(arr):
    count = np.zeros_like(arr)
    for i in range(h):
        for j in range(w):
            if arr[i, j] != 0:
                count[i, j] = calc_neighbor_los(arr, i, j)
    return count


# %%
def test2():
    assert calc_equilibrium(arr, thr_survive=4, counter=count_see) == 2138


# %%

# %%
