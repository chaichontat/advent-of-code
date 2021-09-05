#%%
from collections import defaultdict

from utils import load

raw = load("day25.txt", parseint=True)

# %%
cache: defaultdict[int, list[int]] = defaultdict(lambda: [1])


def transform(subject: int, loop_size: int) -> int:
    cs = cache[subject]
    try:
        return cs[loop_size]
    except IndexError:
        while len(cs) < loop_size + 1:
            cs.append(cs[-1] * subject % 20201227)
    return cs[loop_size]


def get_loop(subject: int, pub: int) -> int:
    i = 1
    while transform(subject, i) != pub:
        i += 1
    return i


def test1() -> None:
    loop_card = get_loop(7, raw[0])
    assert transform(raw[1], loop_card) == 15217943


#%%
