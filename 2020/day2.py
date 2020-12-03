# %%
from utils import load
from collections import Counter

raw = load("day2.txt")

# %% Part 1

valid = 0
for x in raw[:-1]:
    num, char, pwd = x.split(" ")

    num = [int(i) for i in num.split("-")]
    char = char[0]

    if num[0] <= Counter(pwd)[char] <= num[1]:
        valid += 1

print(valid)

# %% Part 2

valid = 0
for x in raw[:-1]:
    num, char, pwd = x.split(" ")

    num = [int(i) for i in num.split("-")]
    char = char[0]

    if (pwd[num[0] - 1] == char) != (pwd[num[1] - 1] == char):  # XOR
        valid += 1

print(valid)

# %%
