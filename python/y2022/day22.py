# %%
import re
from itertools import chain
from pathlib import Path

import numpy as np
from utils import fmap

raw = (
    Path("data/2022/" + __file__.split("/")[-1].split(".")[0] + ".txt")
    .read_text()[:-1]
    .split("\n\n")
)

# raw = """        ...#
#         .#..
#         #...
#         ....
# ...#.......#
# ........#...
# ..#....#....
# ..........#.
#         ...#....
#         .....#..
#         .#......
#         ......#.

# 10R5L5R10L4R5L5""".split(
#     "\n\n"
# )


mapp_str = raw[0].splitlines()
max_width = max(map(len, mapp_str))
mapp_str = fmap(lambda x: x + " " * (max_width - len(x)), mapp_str)

# %%
mapp = np.array([list(iter(x)) for x in mapp_str])


def traverse(ij: tuple[int, int] | np.ndarray, mode: tuple[int, int] | np.ndarray):
    ori = ij
    ij = np.array(ij)
    mode = np.array(mode)

    while True:
        ij += mode
        if not 0 <= ij[0] < mapp.shape[0]:
            ij[0] %= mapp.shape[0]
        if not 0 <= ij[1] < mapp.shape[1]:
            ij[1] %= mapp.shape[1]
        i, j = ij
        curr = mapp[i, j]
        if curr == ".":
            return (i, j)
        if curr == "#":
            return ori


nodes = {}

for i in range(mapp.shape[0]):
    for j in range(mapp.shape[1]):
        match mapp[i, j]:
            case " " | "#":
                continue
            case ".":
                nodes[(i, j)] = {}
            case _:
                raise ValueError(f"Unknown character {mapp[i, j]}")

        # look around and wrap around
        for di, dj in [(0, 1), (1, 0), (0, -1), (-1, 0)]:
            nodes[(i, j)][(di, dj)] = traverse((i, j), (di, dj))

# %%
DIR = {"L": (0, -1), "R": (0, 1), "U": (-1, 0), "D": (1, 0)}
dirs = "URDL"

pos = (0, re.search(r"([\.#])", mapp_str[0]).start(0))
heading = "R"


inss: list[str] = list(chain.from_iterable(re.findall(r"(\d+)([LR])", raw[1])))
if raw[-1][-1].isnumeric():
    inss.append(re.findall(r"(\d)+", raw[1])[-1])

for ins in inss:
    if ins.isnumeric():
        for _ in range(int(ins)):
            pos = nodes[pos][DIR[heading]]
        continue

    if ins == "L":
        heading = dirs[(dirs.index(heading) - 1) % len(dirs)]
    elif ins == "R":
        heading = dirs[(dirs.index(heading) + 1) % len(dirs)]
    else:
        raise ValueError(f"Unknown instruction {ins}")

ans = {"L": 2, "R": 0, "U": 3, "D": 1}
print(pos, heading)
# The final password is the sum of 1000 times the row, 4 times the column, and the facing.
print(1000 * (pos[0] + 1) + 4 * (pos[1] + 1) + ans[heading])
# %%
