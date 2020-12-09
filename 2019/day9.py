#%%
from intcode import IntCode
from utils import load

raw = load("input_day9.txt", split=",", parseint=True)

#%% Part 1

ic = IntCode(raw, [1])
ic.execute()
print(ic.outputs)

#%% Part 2

ic = IntCode(raw, [2])
ic.execute()
print(ic.outputs)
# %%
