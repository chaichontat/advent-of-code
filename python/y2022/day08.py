# %%
from functools import reduce
from pathlib import Path

import numpy as np

raw = np.array(
    list(
        map(
            lambda x: [int(y) for y in iter(x)],
            (
                Path("data/2022/" + __file__.split("/")[-1].split(".")[0] + ".txt")
                .read_text()[:-1]
                .splitlines()
            ),
        )
    )
)


# %%
def make_slice(i: int, j: int):
    return [np.s_[:i, j], np.s_[i, :j], np.s_[(i + 1) :, j], np.s_[i, (j + 1) :]]


visible = 2 * raw.shape[0] + 2 * raw.shape[1] - 4
for i in range(1, raw.shape[0] - 1):
    for j in range(1, raw.shape[1] - 1):
        curr = raw[i, j]
        slices = make_slice(i, j)
        if any(raw[s].max() < curr for s in slices):
            visible += 1
print(visible)


# %%
def get_dist(x: np.ndarray, curr: int, reverse: bool = False):
    # Need to reverse depending on perspective
    first = np.nonzero(x[:: (-1 if reverse else 1)] >= curr)[0]
    return first[0] if first.size else len(x)


n = 0
for i in range(1, raw.shape[0] - 1):
    for j in range(1, raw.shape[1] - 1):
        curr = raw[i, j]
        slices = make_slice(i, j)
        n = max(
            n,
            reduce(
                lambda x, y: x * y,
                (
                    get_dist(raw[s], curr, rev)
                    for s, rev in zip(slices, [True, True, False, False])
                ),
                1,
            ),
        )
print(n)

# %%
