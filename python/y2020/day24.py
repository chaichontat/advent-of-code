#%%
import re

from utils import load

raw = load("day24.txt")

direction = {
    "e": 1.0,
    "se": 0.5 - 1j,
    "sw": -0.5 - 1j,
    "w": -1.0,
    "nw": -0.5 + 1j,
    "ne": 0.5 + 1j,
}


def run_init():
    # black: 0 white: 1
    locs = set()
    for line in raw:
        locs ^= {sum(direction[step] for step in re.findall(r"([ns]?[ew])", line))}
    return locs


def test1():
    locs = run_init()
    assert len(locs) == 528


# %%
def neighbors(x: complex):
    return {x + d for d in direction.values()}


def test2():
    locs = run_init()
    for _ in range(100):
        locs = set(
            x
            for x in set.union(*(neighbors(b) for b in locs))
            if (nn := len(locs & neighbors(x))) == 2 or (x in locs and nn == 1)
        )

    assert len(locs) == 4200


# %%
