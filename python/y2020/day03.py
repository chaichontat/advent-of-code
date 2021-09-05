#%%
import numpy as np

from utils import load

raw = load("day03.txt")
block = len(raw[0])

# %%
def get_tree(x: int, y: int) -> str:
    return raw[y][x % block]


def run(modx: int, mody: int) -> int:
    count = 0
    for i in range(len(raw)):
        y = mody * i
        if y < len(raw):
            x = modx * i
            if get_tree(x, y) == "#":
                count += 1
    return count


# %%
def test1() -> None:
    assert run(3, 1) == 265


def test2() -> None:
    prod: int = np.prod([run(i, 1) for i in range(1, 8, 2)])
    assert prod * run(1, 2) == 3154761400


# %%
