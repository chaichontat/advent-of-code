# %%
from collections import Counter

from utils import load

raw = load("day02.txt")

# %% Part 1
def test1() -> None:
    valid = 0
    for x in raw:
        num, char, pwd = x.split(" ")

        num_ = [int(i) for i in num.split("-")]
        char = char[0]
        if num_[0] <= Counter(pwd)[char] <= num_[1]:
            valid += 1

    assert valid == 447


# %% Part 2


def test2() -> None:
    valid = 0
    for x in raw:
        num, char, pwd = x.split(" ")

        n = [int(i) for i in num.split("-")]
        char = char[0]

        if (pwd[n[0] - 1] == char) != (pwd[n[1] - 1] == char):  # XOR
            valid += 1

    assert valid == 249


# %%
