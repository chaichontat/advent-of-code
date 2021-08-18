#%%
from intcode import IntCode
from utils import load

raw = load("day02.txt", split=",", parseint=True)

#%% Part 1
def part1():
    ic = IntCode(raw)
    ic.ins[1], ic.ins[2] = 12, 2
    ic.execute()
    return ic.ins[0]


def test02a(benchmark):
    assert benchmark(part1) == 5110675


#%% Part 2
def part2():
    for i in range(100):
        for j in range(100):
            ic = IntCode(raw)
            ic.ins[1], ic.ins[2] = i, j
            ic.execute()
            if ic.ins[0] == 19690720:
                return 100 * i + j


def test02b(benchmark):
    assert benchmark(part2) == 4847


# %%
