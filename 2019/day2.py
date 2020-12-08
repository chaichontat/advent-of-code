#%%
from intcode import IntCode
from utils import load

raw = load("input_day2.txt", split=",", parseint=True)

#%% Part 1
ic = IntCode(raw)
ic.ins[1], ic.ins[2] = 12, 2
ic.execute()
print(ic.ins[0])

#%% Part 2
for i in range(100):
    for j in range(100):
        ic = IntCode(raw)
        ic.ins[1], ic.ins[2] = i, j
        ic.execute()
        if ic.ins[0] == 19690720:
            print(100 * i + j)
            break

# %%
