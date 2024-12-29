# %%
import re
from pathlib import Path

import numpy as np
from utils import fmap

raw = Path(
    "../../data/2024/" + __file__.split("/")[-1].split(".")[0] + ".txt"
).read_text()

# %%

m1 = np.array(fmap(lambda x: fmap(int, x), re.findall(r"mul\((\d+),(\d+)\)", raw)))
np.sum(m1[:, 0] * m1[:, 1])


# %%
dos = list(re.finditer(r"do\(\)|don't\(\)|mul\((\d+),(\d+)\)", raw))
combi = sorted(dos, key=lambda x: x.start())
# %%
do = True
curr = 0
for c in combi:
    if c.group(0) == "do()":
        do = True
    elif c.group(0) == "don't()":
        do = False
    elif do:
        curr += int(c.group(1)) * int(c.group(2))
print(curr)


# %%
