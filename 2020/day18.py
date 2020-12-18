#%%
import re
from operator import add, mul
from typing import Iterable

from utils import load

raw = load("day18.txt")


def stepin(it: Iterable[str]) -> int:
    if (x := next(it)) == "(":
        return parse(it)
    return int(x)


def parse(it: Iterable[str]) -> int:
    cmds = {"+": add, "*": mul}
    curr = stepin(it)
    try:
        while (cmd := next(it)) != ")":
            curr = cmds[cmd](curr, stepin(it))
    except StopIteration:
        pass
    return curr


def test1():
    assert sum(parse(iter(line.replace(" ", ""))) for line in raw) == 29839238838303


# %%
class Swap(int):
    """Swap functions for *, + while maintaining order of operations."""

    def __new__(cls, value):
        return super(cls, cls).__new__(cls, value)

    def __add__(self, other):
        return self.__class__(super(Swap, self).__mul__(other))

    def __sub__(self, other):
        return self.__class__(super(Swap, self).__div__(other))

    def __mul__(self, other):
        return self.__class__(super(Swap, self).__add__(other))

    def __div__(self, other):
        return self.__class__(super(Swap, self).__sub__(other))

    def __repr__(self):
        return f"Swap({int(self)})"


def run_swap(x: str) -> int:
    x = x.translate(str.maketrans("*+", "+*"))
    return int(eval(re.sub(r"(\d+)", r"Swap(\1)", x)))


def test2():
    assert sum(run_swap(x) for x in raw) == 201376568795521


# %%
