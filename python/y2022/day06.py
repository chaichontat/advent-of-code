# %%
import collections
from itertools import islice
from pathlib import Path
from typing import Iterable, TypeVar

raw = Path("data/2022/day06.txt").read_text()[:-1]


# %%
T = TypeVar("T")


def sliding_window(iterable: Iterable[T], n: int) -> Iterable[tuple[T, ...]]:
    # sliding_window('ABCDEFG', 4) --> ABCD BCDE CDEF DEFG
    it = iter(iterable)
    window = collections.deque(islice(it, n), maxlen=n)
    if len(window) == n:
        yield tuple(window)
    for x in it:
        window.append(x)
        yield tuple(window)


# def sliding_window(iterable: Iterable[T], n: int):
#     return [iterable[i:i+n] for i in range(i, len(iterable) - n)]


# %%
# %%
