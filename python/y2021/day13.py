#%%
import io

import matplotlib.pyplot as plt
import numpy as np

from utils import read

raw = read("../../data/2021/day13.txt", raw=True)

# %%
coords, ins = raw.split("\n\n")
coords = np.loadtxt(io.StringIO(coords), delimiter=",", dtype=int)
coords = {tuple(x) for x in coords}
ins = [(n.split(" ")[-1][0], int(n.split(" ")[-1][2:])) for n in ins.splitlines()]

# %%
def fold(x: int, t: int):
    if x > t:
        return t - x + t
    return x


out = outold = coords
for d, t in ins:
    outold = out
    out = set()
    for x, y in outold:
        if d == "x":
            out.add((fold(x, t), y))
        elif d == "y":
            out.add((x, fold(y, t)))
        else:
            raise ValueError(d)

# %%
u = np.zeros((10, 40))
for x, y in out:
    u[y, x] = 1

plt.imshow(u, cmap="binary")
# %%
