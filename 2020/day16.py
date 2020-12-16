#%%
from operator import itemgetter
from typing import Iterable

import numpy as np

from utils import load

raw = load("day16.txt")

#%%
dic = dict()
nearby = list()
my = ()
raws = iter(raw)

for line in raws:
    if not line:
        continue

    if "or" in line:
        line = line.split(": ")
        name = line[0]
        vals = line[1].split(" ")
        dic[name] = [[int(y) for y in x.split("-")] for x in itemgetter(0, 2)(vals)]
        continue

    if line == "your ticket:":
        my = np.fromstring(next(raws), dtype=int, sep=",")
        continue

    if line == "nearby tickets:":
        for x in raws:
            nearby.append(list(map(int, x.split(","))))

nearby = np.array(nearby)
# %%
def match(arr: np.ndarray, vals: Iterable[Iterable[int]]) -> np.ndarray:
    """Check if each value in arr is between (val[0], val[1]) in vals."""
    out = np.zeros_like(arr, dtype=bool)
    for val in vals:
        out += np.logical_and(val[0] <= arr, arr <= val[1])
    return out


# %%
valid_arr = np.zeros_like(nearby, dtype=bool)
for criterion, vals in dic.items():
    valid_arr += match(nearby, vals)


def test1():
    """Sum all values that are not valid in any criteria."""
    assert np.sum(~valid_arr * nearby) == 22977


# %%
def test2():
    v_tics = nearby[np.where(np.sum(valid_arr, axis=1) == nearby.shape[1])[0]]  # Valid tickets.

    valid_c: list[list[int, list[int]]] = [  # Criterion: valid columns.
        [
            i,
            list(np.where(np.sum(match(v_tics, vals), axis=0) == v_tics.shape[0])[0]),
        ]
        for i, vals in enumerate(dic.values())
    ]

    # Get unique criterion: col pair. Then, remove said col from all other criteria.
    matches: dict[int, int] = dict()
    while valid_c:
        valid_c = sorted(valid_c, key=lambda x: len(x[1]))
        criterion, col = valid_c.pop(0)
        assert len(col) == 1
        col = col[0]
        matches[criterion] = col
        for t in valid_c:
            try:
                t[1].remove(col)
            except ValueError:
                pass

    targets = [i for i, criterion in enumerate(dic.keys()) if criterion.startswith("departure")]
    assert np.prod(my[list(map(matches.get, targets))]) == 998358379943


# %%
