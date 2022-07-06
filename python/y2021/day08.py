#%%
from collections import Counter

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
raw = read("../../data/2021/day08.txt")
t = list(map(parse, raw))

# %%
def part1():
    cnt = 0
    for line in t:
        for n in line[1]:
            if 2 <= (u := n.bit_count()) <= 4 or u == 7:
                cnt += 1
    return cnt


# %%
# Each line always give all 10 digits.
#  0:      1:      2:      3:      4:
#  aaaa    ....    aaaa    aaaa    ....
# b    c  .    c  .    c  .    c  b    c
# b    c  .    c  .    c  .    c  b    c
#  ....    ....    dddd    dddd    dddd
# e    f  .    f  e    .  .    f  .    f
# e    f  .    f  e    .  .    f  .    f
#  gggg    ....    gggg    gggg    ....
#
# Count frequency of segments across all digits.
# The sum of said frequency of each digit is unique.
# Therefore, simply count all segments and sum it all up, then remap.
counts = dict(a=8, b=6, c=8, d=7, e=4, f=9, g=7)
segs = {
    0: "467889",
    1: "89",
    2: "47788",
    3: "77889",
    4: "6789",
    5: "67789",
    6: "467789",
    7: "889",
    8: "4677889",
    9: "677889",
}

segs = {sum(map(int, v)): str(k) for k, v in segs.items()}


def decode(x: str, code: str) -> int:
    c = Counter(x)
    nums = [sum(list(map(c.get, x))) for x in code.split(" ")]
    return int("".join(map(segs.get, nums)))


def fast_decode(x: str, code: str):
    # Repeated counts instead of Counter.
    ns = ("4725360918"[sum(map(x.count, r)) // 2 % 15 % 11] for r in code.split())
    return int("".join(ns))


print(sum(fast_decode(*x.split(" | ")) for x in raw))

# %%
