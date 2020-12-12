#%%
from utils import load

raw = load("day12.txt")

cmds, amts = map(tuple, zip(*([x[0], int(x[1:])] for x in raw)))
turns = {"L": 1j, "R": -1j}
moves = {"N": 1j, "S": -1j, "E": 1, "W": -1}


def test1():
    pos = 0 + 0j
    heading = 1
    for cmd, amt in zip(cmds, amts):
        if cmd == "F":
            pos += heading * amt
        else:
            heading *= turns.get(cmd, 1) ** (amt // 90)
            pos += moves.get(cmd, 0) * amt
    assert abs(pos.real) + abs(pos.imag) == 2847


# %%


def test2():
    pos = 0 + 0j
    wp = 10 + 1j
    for cmd, amt in zip(cmds, amts):
        if cmd == "F":
            pos += wp * amt
        else:
            wp *= turns.get(cmd, 1) ** (amt // 90)
            wp += moves.get(cmd, 0) * amt
    assert abs(pos.real) + abs(pos.imag) == 29839


# %%
