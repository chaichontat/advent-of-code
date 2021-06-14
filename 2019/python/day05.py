#%%
from intcode import IntCode
from utils import load

raw = load("day05.txt", split=",", parseint=True)

#%% Part 1
def run(x):
    ic = IntCode(raw, x)
    ic.execute()
    return ic.outputs[-1]


def test05a(benchmark):
    assert benchmark(run, [1]) == 15426686


def test05b(benchmark):
    assert benchmark(run, [5]) == 11430197
