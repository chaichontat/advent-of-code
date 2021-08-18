#%%
from intcode import IntCode
from utils import load

raw = load("day09.txt", split=",", parseint=True)

#%% Part 1
def run(x):
    ic = IntCode(raw, x)
    ic.execute()
    return ic.outputs[-1]


def test09a(benchmark):
    assert benchmark(run, [1]) == 2789104029


def test09b(benchmark):
    assert benchmark(run, [2]) == 32869
