#%%
from collections import Counter
from functools import lru_cache

from utils import load

raw = load("day10.txt", parseint=True)
raw.append(0)
raw.append(max(raw) + 3)
s = sorted(raw)


def test1():
    count = Counter([s[i + 1] - s[i] for i in range(len(s) - 1)])
    assert count[1] * count[3] == 1755


# %%
@lru_cache
def recurse(curr_max, idx):
    if idx == len(s) - 1:
        return 1

    out = 0
    for i in range(idx, len(s)):
        if s[i] <= curr_max + 3:
            out += recurse(s[i], i + 1)
        else:
            break
    return out


def test2():
    assert recurse(s[0], 1) == 4049565169664


# %%
