#%%
from utils import load

raw = load("day9.txt", parseint=True)[:-1]

# %% Part 1
def run_test(idx):
    for i in range(idx):
        for j in range(i, idx):
            if raw[idx] == raw[i] + raw[j]:
                return True
    return False


def test1():
    for idx in range(25, len(raw)):
        if not run_test(idx):
            assert raw[idx] == 1038347917
            return idx


# %%
# Since sorted.
def test2():
    magic = test1()
    for i in range(magic):
        for j in range(i, magic):
            if sum(raw[i:j]) == raw[magic]:
                assert min(raw[i:j]) + max(raw[i:j]) == 137394018
                return


# %%
