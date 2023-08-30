# %%
import re
from pathlib import Path

from utils import fmap

raw = (
    Path("data/2022/" + __file__.split("/")[-1].split(".")[0] + ".txt")
    .read_text()[:-1]
    .splitlines()
)

proc = fmap(lambda x: fmap(int, re.findall(r"(\d+)", x)), raw)


# how many assignment pairs does one range fully contain the other?
print(
    "Part 1:",
    len(
        [
            None
            for x1, x2, y1, y2 in proc
            if x1 <= y1 <= y2 <= x2 or y1 <= x1 <= x2 <= y2
        ]
    ),
)

# how many assignment pairs do the ranges overlap?
print(
    "Part 2:",
    len([None for x1, x2, y1, y2 in proc if x1 <= y1 <= x2 or y1 <= x1 <= y2]),
)

# %%
# %%
