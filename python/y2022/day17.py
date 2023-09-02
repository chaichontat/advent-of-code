# %%
from itertools import cycle
from pathlib import Path
from typing import Any

import numpy as np

raw = Path("data/2022/" + __file__.split("/")[-1].split(".")[0] + ".txt").read_text()[
    :-1
]

dic = {">": 1, "<": -1}
# raw = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"
inss = tuple(map(lambda x: dic[x], raw))

# %%
# The tall, vertical chamber is exactly seven units wide.
# Each rock appears so that
# - its left edge is two units away from the left wall
# - its bottom edge is three units above the highest rock in the room (or the floor, if there isn't one).

rocks: tuple[dict[str, Any], ...] = (
    {"pos": np.array([(0, 0), (1, 0), (2, 0), (3, 0)]), "width": 4, "height": 1},
    {
        "pos": np.array([(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)]),
        "width": 3,
        "height": 3,
    },
    {
        "pos": np.array([(0, 2), (1, 2), (2, 0), (2, 1), (2, 2)]),
        "width": 3,
        "height": 3,
    },
    {"pos": np.array([(0, 0), (0, 1), (0, 2), (0, 3)]), "width": 1, "height": 4},
    {"pos": np.array([(0, 0), (0, 1), (1, 0), (1, 1)]), "width": 2, "height": 2},
)


def show(cs: set[tuple[int, int]], temp: list[tuple[int, int]] | None = None):
    if temp is not None:
        cs = cs | set(map(tuple, temp))
    min_y = min(y for _, y in cs)
    return "\n".join(
        "".join("#" if (x, y) in cs else "." for x in range(7)) for y in range(min_y, 1)
    )


def simulate(n: int):
    coords = {(i, 0) for i in range(7)}
    insc = cycle(inss)
    mhs = []
    for i in range(n):
        min_height = min(y for _, y in coords)
        mhs.append(-min_height)
        rock = rocks[i % 5]
        offset = (2, min_height - 3 - rock["height"])
        pos = rock["pos"] + offset
        for ins in insc:
            if set(map(tuple, pos + (ins, 0))) & coords:
                ...
            elif not (np.any(pos[:, 0] + ins < 0) or np.any(pos[:, 0] + ins >= 7)):
                pos = pos + (ins, 0)

            new_pos = pos + (0, 1)
            if set(map(tuple, new_pos)) & coords:
                coords |= set(map(tuple, pos))
                break
            pos = new_pos
    return mhs


sim = np.array(simulate(3000))
print("Part 1:", sim[2022])

# %%


def part2(sim: np.ndarray, n: int, steady: int = 500, δ: int = 100):
    diff = np.diff(np.array(sim))
    cs = np.nonzero(
        [
            np.all(diff[steady : steady + δ] == diff[steady + i : steady + δ + i])
            for i in range(len(diff) - steady - δ)
        ]
    )[0]

    cycle = cs[1] - cs[0]
    return np.sum(
        [
            np.sum(diff[:steady]),  # before stationary
            np.sum(diff[steady : steady + cycle]) * ((n - steady) // cycle),
            np.sum(diff[steady : steady + (n - steady) % cycle]),  # remaining
        ]
    )


print("Part 2:", part2(sim, 1_000_000_000_000))
