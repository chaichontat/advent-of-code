# %%
from collections import deque
from functools import reduce
from pathlib import Path

import numpy as np
from utils import fmap

raw = list(
    map(
        int,
        Path("data/2022/" + __file__.split("/")[-1].split(".")[0] + ".txt")
        .read_text()[:-1]
        .split("\n"),
    )
)

# raw = list(
#     map(
#         int,
#         """1
# 2
# -3
# 3
# -2
# 0
# 4
# """.splitlines(),
#     )
# )
# %%


def get_idx(lis: list[tuple[int, int]], idx: int):
    for i, r in enumerate(lis):
        if r[1] == idx:
            return i
    assert False


def get(w: list[tuple[int, int]], zero: int, i: int):
    return w[(zero + i) % len(w)][0]


# %%
def mix(w: list[tuple[int, int]]):
    working = w.copy()
    for i in range(len(working)):
        idx = get_idx(working, i)
        assert working[idx][1] == i
        idx += (popped := working.pop(idx))[0]
        working.insert(u if (u := idx % len(working)) != 0 else len(working), popped)

    zidx = working.index((0, raw.index(0)))
    return sum(get(working, zidx, i) for i in (1000, 2000, 3000)), working


w = [(r, i) for i, r in enumerate(raw)]
print(mix(w)[0])


# %%
part2 = 0, [(x * 811589153, i) for x, i in w]
for _ in range(10):
    part2 = mix(part2[1])


print(part2[0])
# %%
