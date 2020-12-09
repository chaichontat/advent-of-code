#%%
from itertools import permutations
from intcode import IntCode
from utils import load

raw = load("input_day7.txt", split=",", parseint=True)

#%% Part 1


def run_phase(seq):
    input_ = 0
    for phase in seq:
        ic = IntCode(raw, [phase, input_])
        ic.execute()
        input_ = ic.outputs[0]
    return input_


print(max([run_phase(seq) for seq in permutations(range(5))]))

#%% Part 2
# Return code 1 -> need input, 2 -> have output.


def run_loop(seq):
    n = len(seq)
    buffer = 0
    ics = [IntCode(raw, [seq[i]], pause_on_io=True) for i in range(n)]
    [ic.execute() for ic in ics]
    i = 0
    while True:
        ics[i % n].inputs.append(buffer)
        rc = ics[i % n].execute()
        if rc == 2:
            buffer = ics[i % n].outputs.pop()
        elif rc == 1:
            raise ValueError
        elif rc == 0:
            return buffer
        i += 1


print(max([run_loop(seq) for seq in permutations(range(5, 10))]))
# %%
