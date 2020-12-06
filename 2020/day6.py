#%%
from utils import load

raw = load("day6.txt", split="\n\n")
# %% Part 1

sum([len(set(group) - {"\n"}) for group in raw])
# %% Part 2

sum([len(set.intersection(*[set(x) for x in group.split("\n")])) for group in raw])
# %%
