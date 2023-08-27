# %%
from collections import deque
from operator import itemgetter
from pathlib import Path

import numpy as np

raw = Path("data/2022/" + __file__.split("/")[-1].split(".")[0] + ".txt").read_text()[
    :-1
]

st, ins = [x.split("\n") for x in raw.split("\n\n")]
stack = np.array([list(x[1::4]) for x in st[:-1]])

stack = list(map(lambda x: deque(filter(lambda x: x != " ", x)), stack.T))
cmds = list(map(lambda x: [int(y) for y in itemgetter(1, 3, 5)(x.split(" "))], ins))


def run_part1(stack: list[deque[str]], cmds: list[list[int]]):
    stack = stack.copy()
    for rep, src, dst in cmds:
        stack[dst - 1].extendleft(stack[src - 1].popleft() for _ in range(rep))

    return "".join(map(lambda x: x[0], stack))


def run_part2(stack: list[deque[str]], cmds: list[list[int]]):
    stack = stack.copy()
    for rep, src, dst in cmds:
        stack[dst - 1].extendleft(
            reversed([stack[src - 1].popleft() for _ in range(rep)])
        )

    return "".join(map(lambda x: x[0], stack))


print("Part 1:", run_part1(stack, cmds))
print("Part 2:", run_part2(stack, cmds))

# %%
