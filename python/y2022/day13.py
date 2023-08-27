# %%
from functools import cmp_to_key, reduce
from itertools import chain
from pathlib import Path
from typing import cast

from expression import Nothing, Option, Some

raw = (
    Path("data/2022/" + __file__.split("/")[-1].split(".")[0] + ".txt")
    .read_text()[:-1]
    .split("\n\n")
)

Rec = list["Rec"] | int

proc = cast(list[tuple[Rec, Rec]], [tuple(map(eval, x.split("\n"))) for x in raw])

# %%


def compare(a: Rec, b: Rec) -> Option[bool]:
    if isinstance(a, int) and isinstance(b, int):
        if a < b:
            return Some(True)
        elif a > b:
            return Some(False)
        else:
            return Nothing

    if isinstance(a, int):
        a = [a]
    if isinstance(b, int):
        b = [b]

    for x, y in zip(a, b):
        match compare(x, y):
            case Some(g):
                return Some(g)
            case _:
                ...

    if len(a) < len(b):
        return Some(True)
    if len(a) > len(b):
        return Some(False)
    return Nothing


res = [i for i, x in enumerate(proc, 1) if compare(*x).value]
print("Part 1:", sum(x for x in res))

# %%
indicator: Rec = [[[2]], [[6]]]
comm: Rec = [*chain.from_iterable(proc), *indicator]
# Reversed with the comparator function.
out = sorted(comm, key=cmp_to_key(lambda x, y: -compare(x, y).value * 2 + 1))

print(
    "Part 2:", reduce(lambda x, y: x * y, map(lambda x: out.index(x) + 1, indicator), 1)
)
# %%
