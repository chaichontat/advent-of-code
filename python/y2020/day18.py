#%%
from __future__ import annotations

import re
from operator import add, mul
from typing import Iterator, NoReturn

from utils import load

raw = load("day18.txt")


def stepin(it: Iterator[str]) -> int:
    if (x := next(it)) == "(":
        return parse(it)
    return int(x)


def parse(it: Iterator[str]) -> int:
    cmds = {"+": add, "*": mul}
    curr = stepin(it)
    try:
        while (cmd := next(it)) != ")":
            curr = cmds[cmd](curr, stepin(it))
    except StopIteration:
        pass
    return curr


def test1() -> None:
    assert sum(parse(iter(line.replace(" ", ""))) for line in raw) == 29839238838303


# %%
class Swap(int):
    """Swap functions for *, + while maintaining order of operations."""

    def __new__(cls, value: int) -> Swap:
        return super(cls, cls).__new__(cls, value)

    def __add__(self, other: int) -> Swap:
        return self.__class__(super(Swap, self).__mul__(other))

    def __sub__(self, other: int) -> NoReturn:
        raise NotImplementedError

    def __mul__(self, other: int) -> Swap:
        return self.__class__(super(Swap, self).__add__(other))

    def __truediv__(self, other: int) -> NoReturn:
        raise NotImplementedError

    def __floordiv__(self, x: int) -> NoReturn:
        raise NotImplementedError

    def __repr__(self) -> str:
        return f"Swap({int(self)})"


def run_swap(x: str) -> int:
    x = x.translate(str.maketrans("*+", "+*"))
    return int(eval(re.sub(r"([0-9]+)", r"Swap(\1)", x)))


def test2() -> None:
    assert sum(run_swap(x) for x in raw) == 201376568795521


# %%

# %%
