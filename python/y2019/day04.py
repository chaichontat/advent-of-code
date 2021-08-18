#%%
from collections import Counter


#%% Part 1
def check(n):
    ok = False
    n = str(int(n))
    for i in range(len(n) - 1):
        if n[i] > n[i + 1]:  # Going from left to right, the digits never decrease.
            return False
        if n[i] == n[i + 1]:  # Two adjacent digits are the same.
            ok = True
    return ok


def test04a(benchmark):
    assert benchmark(lambda: sum([check(n) for n in range(171309, 643603)])) == 1625


#%% Part 2
def check2(n):
    n = str(int(n))
    for i in range(len(n) - 1):
        if n[i] > n[i + 1]:  # Going from left to right, the digits never decrease.
            return False

    nums = Counter([n[i : i + 2] for i in range(len(n) - 1) if n[i] == n[i + 1]])
    for i in nums.values():
        if i == 1:
            return True
    return False


def test04b(benchmark):
    assert benchmark(lambda: sum([check2(n) for n in range(171309, 643603)])) == 1111


# %%
