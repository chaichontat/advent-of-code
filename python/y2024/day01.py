# %%
from functools import reduce
from pathlib import Path
from typing import Iterable

import numpy as np
from utils import fmap

raw = Path(
    "../../data/2024/" + __file__.split("/")[-1].split(".")[0] + ".txt"
).read_text()[:-1]
raw = np.fromstring(raw, sep="\t", dtype=int).reshape(-1, 2)

# %%
arr = np.sort(raw, axis=0)
np.sum(np.abs(arr[:, 0] - arr[:, 1]))


# %%
right, cnts = np.unique(arr[:, 1], return_counts=True)
bins = np.digitize(arr[:, 0], right)


def find_matching_indices(arr1, arr2):
    idx = np.searchsorted(arr2, arr1)
    idx[~((idx < len(arr2)) & (arr1 == arr2[idx]))] = -1
    return idx


# %%

right, cnts = np.unique(arr[:, 1], return_counts=True)
res = find_matching_indices(arr[:, 0], right)
idxs = res[res > -1]

np.sum(right[idxs] * cnts[idxs])
# %%
