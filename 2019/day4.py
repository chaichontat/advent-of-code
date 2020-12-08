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


print(sum([check(n) for n in range(171309, 643603)]))

#%% Part 2


def check(n):
    n = str(int(n))
    for i in range(len(n) - 1):
        if n[i] > n[i + 1]:  # Going from left to right, the digits never decrease.
            return False

    nums = Counter([n[i : i + 2] for i in range(len(n) - 1) if n[i] == n[i + 1]])
    for i in nums.values():
        if i == 1:
            return True
    return False


print(sum([check(n) for n in range(171309, 643603)]))
