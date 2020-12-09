# %%
from collections import Counter

from utils import load

raw = load("day2.txt")

# %% Part 1
def test1():
    valid = 0
    for x in raw:
        num, char, pwd = x.split(" ")

        num = [int(i) for i in num.split("-")]
        char = char[0]

        if num[0] <= Counter(pwd)[char] <= num[1]:
            valid += 1

    assert valid == 447


# %% Part 2


def test2():
    valid = 0
    for x in raw:
        num, char, pwd = x.split(" ")

        num = [int(i) for i in num.split("-")]
        char = char[0]

        if (pwd[num[0] - 1] == char) != (pwd[num[1] - 1] == char):  # XOR
            valid += 1

    assert valid == 249


# %%
