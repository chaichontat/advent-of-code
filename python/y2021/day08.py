#%%
import numpy as np

from utils import read


def conv_digit(x: str) -> int:
    out = 0
    for c in x:
        out |= 1 << (ord(c) - ord("a"))
    return out


def proc(x: str) -> np.ndarray:
    return np.array(list((map(conv_digit, x.split(" ")))), dtype=np.uint8)


parse = lambda x: (proc(x[:58]), proc(x[61:]))
raw = read("../../data/2021/day08.txt", parse=parse)


# %%
def part1():
    cnt = 0
    for line in raw:
        for n in line[1]:
            if 2 <= (u := n.bit_count()) <= 4 or u == 7:
                cnt += 1
    return cnt


#     ys.extend(list(map(len, line[1])))
# c = Counter(ys)
# sum([c[2], c[4], c[3], c[7]])
# %%
# Maximum bipartite matching function
