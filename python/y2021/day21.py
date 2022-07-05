#%%
from collections import Counter, defaultdict
from functools import cache
from itertools import product

from utils import read

raw = read("../../data/2021/day21.txt")
ps = [int(r.split(" ")[-1]) for r in raw]
# %%
def step1(start: int, idx: int) -> int:
    return (start - 1 + 3 * idx + 3) % 10 + 1


scores = [0, 0]
pos = ps.copy()
# pos = [4, 8]
curr = 1
rolled = 0
while True:
    for i in range(2):
        pos[i] = step1(pos[i], curr)
        scores[i] += pos[i]
        curr += 3
        rolled += 3
        if curr > 100:
            curr -= 100
        if scores[i] >= 1000:
            break
    else:
        continue
    break

print(min(scores) * rolled)


# %%
cnts = Counter([a + b + c for a, b, c in product(range(1, 4), repeat=3)])
f = lambda x: (x - 1) % 10 + 1


@cache
def step(pos: int):
    """New position and count after each die roll. Only 7 outcomes possible."""
    out = defaultdict(int)
    for k, v in cnts.items():
        out[f(pos + k)] += v
    return out


@cache
def scs(pos: tuple[int, int], scores: tuple[int, int], turn: bool) -> tuple[int, int]:
    """Returns numbers of wins given the current position, scores, and turn.
    Iterate through all new positions, if wins, add wins.
    Else, recurse with new position and new scores.
    """
    wins = [0, 0]
    for p, cnt in step(pos[turn]).items():
        if scores[turn] + p >= 21:
            wins[turn] += cnt
            continue

        if not turn:
            p_new = (p, pos[1])
            sc_new = (scores[0] + p, scores[1])
        else:
            p_new = (pos[0], p)
            sc_new = (scores[0], scores[1] + p)
        w = scs(p_new, sc_new, 1 - turn)
        wins[0] += cnt * w[0]
        wins[1] += cnt * w[1]

    return tuple(wins)


print(max(scs(tuple(ps), (0, 0), 0)))
# %%
