#%%
import numpy as np
from utils import load

raw = load("day3.txt")[:-1]
block = len(raw[0])
# %%
def get_tree(x, y):
    return raw[y][x % block]


def run(modx, mody):
    count = 0
    for i in range(len(raw)):
        y = mody * i
        if y < len(raw):
            x = modx * i
            if get_tree(x, y) == "#":
                count += 1
    return count


# %%

np.prod([run(i, 1) for i in range(1, 8, 2)]) * run(1, 2)
# %%
