# %%
import re
from itertools import chain
from pathlib import Path
from typing import Callable

import numpy as np
import numpy.typing as npt
from utils import fmap

raw = (
    Path("data/2022/" + __file__.split("/")[-1].split(".")[0] + ".txt")
    .read_text()[:-1]
    .split("\n\n")
)
Coord = tuple[int, int]
DEBUG = False

raw = (
    raw
    if not DEBUG
    else """        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5""".split(
        "\n\n"
    )
)

initial_mapping = (
    dict(U="ABAD", L="BCF", R="EDE", D="GFGC")
    if DEBUG
    else dict(U="EAB", L="CECA", R="DFDG", D="BGF")
)
SIDE_WIDTH = 4 if DEBUG else 50
NOT_INVERTED = "B" if DEBUG else "ABEFG"


mapp_str = raw[0].splitlines()
max_width = max(map(len, mapp_str))
mapp_str = fmap(lambda x: x + " " * (max_width - len(x)), mapp_str)

DIR = {"U": (-1, 0), "R": (0, 1), "D": (1, 0), "L": (0, -1)}
OPPOSITE = {"L": "R", "R": "L", "U": "D", "D": "U"}
dirs = "URDL"
HEADINGS = [DIR[d] for d in dirs]

# %%
mapp = np.array([list(iter(x)) for x in mapp_str])


def gen_mapping(initial_mapping: dict[str, str]):
    m_: dict[tuple[str, int], tuple[str, int]] = {}

    def find_match(direction: str, idx: int) -> tuple[str, int]:
        target = initial_mapping[direction][idx]
        for d_, e_ in initial_mapping.items():
            for j, edge_ in enumerate(e_):
                if direction == d_ and idx == j:
                    continue
                if edge_ == target:
                    return d_, j
        assert False

    for direction, edges in initial_mapping.items():
        for i, _ in enumerate(edges):
            m_[(direction, i)] = find_match(direction, i)

    return m_


# %%
mapping = gen_mapping(initial_mapping)

NEW_SIDE_COORD: dict[str, Callable[[int, int], Coord]] = {
    "L": lambda q, rem: (SIDE_WIDTH * q + rem, 0),
    "R": lambda q, rem: (SIDE_WIDTH * q + rem, mapp.shape[1] - 1),
    "U": lambda q, rem: (0, SIDE_WIDTH * q + rem),
    "D": lambda q, rem: (mapp.shape[0] - 1, SIDE_WIDTH * q + rem),
}


def transform(ij: Coord | np.ndarray, mode: Coord | np.ndarray):
    i, j = ij
    if i < 0 or i > mapp.shape[0] - 1:
        q, rem = divmod(j, SIDE_WIDTH)
        new_side, n = mapping[("U" if i < 0 else "D", q)]
    elif j < 0 or j > mapp.shape[1] - 1:
        q, rem = divmod(i, SIDE_WIDTH)
        new_side, n = mapping[("L" if j < 0 else "R", q)]
    else:
        raise ValueError(f"{ij} not at edge.")

    if initial_mapping[new_side][n] in NOT_INVERTED:
        new_ij = NEW_SIDE_COORD[new_side](n, rem)
    else:
        new_ij = NEW_SIDE_COORD[new_side](n, SIDE_WIDTH - rem - 1)
    new_mode = DIR[OPPOSITE[new_side]]
    return np.array(new_ij), np.array(new_mode)


def traverse(
    ij_: Coord | np.ndarray,
    mode_: Coord | np.ndarray,
    transformer: Callable[[npt.NDArray, npt.NDArray], tuple[npt.NDArray, npt.NDArray]],
) -> tuple[npt.NDArray, npt.NDArray]:
    ori = np.array(ij_)
    ij = np.array(ij_)
    old_mode = mode = np.array(mode_)

    while True:  # traverse through empty space
        ij += mode
        if not 0 <= ij[0] < mapp.shape[0] or not 0 <= ij[1] < mapp.shape[1]:
            ij, mode = transformer(ij, mode)
        i, j = ij
        curr = mapp[i, j]
        if curr == "#":  # don't change heading if hit wall
            return ori, old_mode
        if curr == ".":
            return ij, mode


def show(m_s: list[Coord]):
    m_ = np.array(list(map(list, mapp_str)))
    for i, j in m_s:
        m_[i, j] = "o"
    return "\n".join(map("".join, m_)) + "\n"


def run(
    transformer: Callable[[npt.NDArray, npt.NDArray], tuple[npt.NDArray, npt.NDArray]]
):
    pos = (0, re.search(r"([\.#])", mapp_str[0]).start(0))
    heading = (0, 1)

    inss: list[str] = list(chain.from_iterable(re.findall(r"(\d+)([LR])", raw[1])))
    if raw[-1][-1].isnumeric():
        inss.append(re.findall(r"(\d+)", raw[1])[-1])

    visited = set()
    for ins in inss:
        if ins.isnumeric():
            for _ in range(int(ins)):
                visited.add(tuple(pos))
                pos, heading = traverse(pos, heading, transformer)
            continue

        if ins == "L":
            heading = HEADINGS[(HEADINGS.index(tuple(heading)) - 1) % len(dirs)]
        elif ins == "R":
            heading = HEADINGS[(HEADINGS.index(tuple(heading)) + 1) % len(dirs)]
        else:
            raise ValueError(f"Unknown instruction {ins}")

    ans = {"L": 2, "R": 0, "U": 3, "D": 1}
    # The final password is the sum of 1000 times the row, 4 times the column, and the facing.
    return (
        1000 * (pos[0] + 1)
        + 4 * (pos[1] + 1)
        + ans[dirs[HEADINGS.index(tuple(heading))]]
    )


part1 = run(transformer=lambda ij, mode: (ij % mapp.shape, mode))
print("Part 1:", part1)
part2 = run(transformer=transform)
print("Part 2:", part2)

# %%
