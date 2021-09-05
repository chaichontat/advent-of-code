#%%
from typing import Tuple

import numpy as np

from utils import load

raw = load("day05.txt")

# %%
def binary(x: str, l: int, h: int, lowc: str, hic: str) -> int:
    for w in x:
        if w == lowc:
            h = int((l + h) / 2)
        elif w == hic:
            l = int((l + h + 1) / 2)
        else:
            raise ValueError("WTF")
    assert l == h
    return l


def process(x: str) -> Tuple[int, int]:
    return (binary(x[:7], 0, 127, "F", "B"), binary(x[-3:], 0, 7, "L", "R"))


def test1() -> int:
    out = 0
    for seat in raw:
        row, col = process(seat)
        if (id_ := row * 8 + col) > out:
            out = id_
    assert out == 935
    return out


# %%
def test2() -> None:
    arr = np.zeros(test1() + 1)
    for seat in raw:
        row, col = process(seat)
        arr[row * 8 + col] = 1

    assert np.where(arr == 0)[0][-1] == 743
