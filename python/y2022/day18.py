# %%
from collections import deque
from pathlib import Path

import numpy as np
from utils import fmap

raw = (
    Path("data/2022/" + __file__.split("/")[-1].split(".")[0] + ".txt")
    .read_text()[:-1]
    .split("\n")
)

# raw = """2,2,2
# 1,2,2
# 3,2,2
# 2,1,2
# 2,3,2
# 2,2,1
# 2,2,3
# 2,2,4
# 2,2,6
# 1,2,5
# 3,2,5
# 2,1,5
# 2,3,5""".splitlines()

arr = np.array(fmap(lambda x: tuple(fmap(int, x.split(","))), raw))
proc_list = list(map(tuple, arr))
proc_set = set(map(tuple, arr))

DIR = fmap(
    np.array, [[0, 0, 1], [0, 0, -1], [0, 1, 0], [0, -1, 0], [1, 0, 0], [-1, 0, 0]]
)

# %%


def calc_surface_area(proc: np.ndarray):
    total = 0
    p = set(map(tuple, proc))
    cleared = [0 for _ in range(len(proc))]
    for i in range(len(proc)):
        for j, d in enumerate(DIR):
            if tuple(proc[i] + d) not in p:
                cleared[i] |= 1 << j
        total += cleared[i].bit_count()
    return total, cleared


part1, cleared = calc_surface_area(arr)
print("Part 1:", part1)
# %%
bounding_box = np.min(arr, axis=0), np.max(arr, axis=0)


def test_bounding_box(pos: tuple[int, int, int]):
    return np.any(pos < bounding_box[0] - 1) or np.any(bounding_box[1] + 1 < pos)


def bfs(pos: tuple[int, int, int]):
    """Flood fill from outside the structure, count areas of contact"""
    visited = set()
    count = 0
    q = deque([pos])
    while q:
        p = q.popleft()
        visited.add(p)
        count += len([tuple(p_ + d) for d in DIR if (p_ := tuple(p + d)) in proc_set])
        q.extend(
            [
                p_
                for d in DIR
                if (p_ := tuple(p + d)) not in proc_set
                and not test_bounding_box(p_)
                and p_ not in visited
                and p_ not in q
            ]
        )

    return count, visited


print("Part 2:", bfs(tuple(bounding_box[0] - 1))[0])

# %%
