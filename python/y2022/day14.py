# %%
from functools import cmp_to_key, reduce
from itertools import chain, islice
from pathlib import Path
from typing import Iterable, TypeVar, cast

import matplotlib.pyplot as plt
import numpy as np
import seaborn as sns
from expression import Nothing, Option, Some
from utils import fmap

sns.set()
raw = (
    Path("data/2022/" + __file__.split("/")[-1].split(".")[0] + ".txt")
    .read_text()[:-1]
    .split("\n")
)

# raw = """498,4 -> 498,6 -> 496,6
# 503,4 -> 502,4 -> 502,9 -> 494,9""".splitlines()

res = fmap(lambda x: fmap(lambda y: fmap(int, y.split(",")), x.split(" -> ")), raw)


def window(seq: Iterable[T], n: int = 2) -> Iterable[tuple[T, ...]]:
    "Returns a sliding window (of width n) over data from the iterable"
    "   s -> (s0,s1,...s[n-1]), (s1,s2,...,sn), ...                   "
    it = iter(seq)
    result = tuple(islice(it, n))
    if len(result) == n:
        yield result
    for elem in it:
        result = result[1:] + (elem,)
        yield result


def gen_cave(arr: np.ndarray, p1: list[int], p2: list[int]):
    # x == x
    if p1[0] == p2[0]:
        arr[min(p1[1], p2[1]) : max(p1[1], p2[1]) + 1, p1[0]] = 9
    else:
        arr[p1[1], min(p1[0], p2[0]) : max(p1[0], p2[0]) + 1] = 9


# %%
arr = np.zeros((200, 1000), dtype=int)
T = TypeVar("T")

[gen_cave(arr, p1, p2) for row in res for p1, p2 in window(row)]


def step(arr: np.ndarray, pos: list[int]) -> Option[list[int]]:
    x, y = pos
    if arr[y, x] or y >= arr.shape[0] - 1:
        return Nothing

    if not arr[y + 1, x]:
        return step(arr, [x, y + 1])
    if not arr[y + 1, x - 1]:
        return step(arr, [x - 1, y + 1])
    if not arr[y + 1, x + 1]:
        return step(arr, [x + 1, y + 1])

    return Some(pos)


def run(arr: np.ndarray):
    arr = arr.copy()
    i = 0
    while True:
        try:
            x, y = step(arr, [500, 0]).value
        except ValueError:
            return arr, i
        else:
            i += 1
            arr[y, x] = 2


# %%
print("Part 1:", run(arr)[1])


# Floor
arr2 = arr.copy()
arr2[np.argwhere(arr2)[:, 0].max() + 2, :] = 9
print("Part 2:", run(arr2)[1])

# %%
