#%%
import re
from itertools import product

from utils import load

raw = load("day14.txt", split=None)
cmds = re.sub(r"[^\w\s]", " ", raw).split()

# %%
def test1():
    words = iter(cmds)
    mem: dict[int, int] = dict()
    ones, zeros = -1, -1

    for word in words:
        if word == "mask":
            mask = next(words)
            ones = int(mask.replace("X", "0"), 2)  # All 0 except places to change to 1.
            zeros = int(mask.replace("X", "1"), 2)  # All 1 except places to change to 0.
            continue

        # Replace with ones then zeros.
        addr = int(next(words))
        mem[addr] = (int(next(words)) | ones) & zeros

    assert sum(mem.values()) == 15403588588538


#%%
def test2():
    words = iter(cmds)
    mem: dict[int, int] = dict()
    mask_1 = -1
    x_idx: list[int] = []

    for word in words:
        if word == "mask":
            mask = next(words)
            mask_1 = int(mask.replace("X", "0"), 2)
            x_idx = [i for i, c in enumerate(mask) if c == "X"]
            continue

        addr = int(next(words)) | mask_1
        val = int(next(words))

        for vals in product([0, 1], repeat=len(x_idx)):
            for i, v in zip(x_idx, vals):
                if v:
                    addr |= 2 ** (35 - i)  # All 0 except at i.
                else:
                    addr &= ~(2 ** (35 - i))  # All 1 except at i.
            mem[addr] = val

    assert sum(mem.values()) == 3260587250457


# %%
