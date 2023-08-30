# %%
from functools import reduce
from pathlib import Path
from typing import Iterable

from utils import fmap

raw = (
    Path("data/2022/" + __file__.split("/")[-1].split(".")[0] + ".txt")
    .read_text()[:-1]
    .splitlines()
)


def convert(a: str):
    return (ord(a) - 96) if a.islower() else (ord(a) - 64 + 26)


def run(lines: Iterable[Iterable[str]]):
    return sum(
        convert(next(iter(reduce(lambda x, y: x & y, fmap(set, line)))))
        for line in lines
    )


# %%
print("Part 1:", run((x[: len(x) // 2], x[len(x) // 2 :]) for x in raw))

# %%
print("Part 2:", run(raw[i : i + 3] for i in range(0, len(raw), 3)))

# %%
