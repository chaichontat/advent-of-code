#%%
import re
from typing import Iterable

import numpy as np
from scipy.sparse import csr_matrix
from scipy.sparse.csgraph import maximum_bipartite_matching

from utils import load

dic, my, nearby = load("day16.txt", split="\n\n")
#%%
ranges = [tuple(map(int, re.findall("\d+", x))) for x in dic.split("\n")]
my = np.fromstring(my.split("\n")[1], dtype=int, sep=",")
nearby = np.array([list(map(int, x.split(","))) for x in nearby.split("\n")[1:]])

#%%
def match(arr: np.ndarray, val: Iterable[int], out=None) -> np.ndarray:
    """Check if each value in arr is between (val[0], val[1]) in vals."""
    if out is None:
        out = np.zeros_like(arr, dtype=bool)
    out += np.logical_and(val[0] <= arr, arr <= val[1])
    out += np.logical_and(val[2] <= arr, arr <= val[3])
    return out


# %%
valid_arr = np.zeros_like(nearby, dtype=bool)
for vals in ranges:
    match(nearby, vals, valid_arr)


def test1():
    """Sum all values that are not valid in any criteria."""
    assert np.sum(~valid_arr * nearby) == 22977


#%%
def test2():
    v_tics = nearby[np.all(valid_arr, axis=1)]  # Valid tickets.
    n_cols = len(ranges)

    adj = np.zeros((n_cols, n_cols), dtype=bool)  # "Adjacency matrix"
    for i, vals in enumerate(ranges):
        adj[:, i] = np.all(match(v_tics, vals), axis=0)

    matches = maximum_bipartite_matching(csr_matrix(adj))
    targets = [i for i, s in enumerate(dic.split("\n")) if s.startswith("departure")]
    assert np.prod(my[matches[targets]]) == 998358379943
