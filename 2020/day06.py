#%%
from utils import load

raw = load("day06.txt", split="\n\n")

# %%
def test1():
    assert sum([len(set(group) - {"\n"}) for group in raw]) == 6521


# %%
def test2():
    assert (
        sum([len(set.intersection(*[set(x) for x in group.split("\n")])) for group in raw])
        == 3305
    )


# %%
