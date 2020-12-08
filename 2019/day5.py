#%%
from intcode import IntCode
from utils import load


raw = load("input_day5.txt", split=",", parseint=True)

#%% Part 1
ic = IntCode(raw)
ic.execute([1])
print(ic.outputs)

#%% Part 2
ic = IntCode(raw)
ic.execute([5])
print(ic.outputs)

# %%
