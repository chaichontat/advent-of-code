#%%
from __future__ import annotations

from math import prod

from utils import load

raw = list(map(int, load("day23.txt")[0]))


def mod1(a: int, m: int) -> int:
    return x if (x := a % m) != 0 else m


def pick(st: dict[int, int], c: int) -> tuple[int, ...]:
    return st[c], st[st[c]], st[st[st[c]]]


def iterate(stack: dict[int, int], start: int, step: int = 1_000_000) -> list[int]:
    out = [start]
    u = start
    i = 0
    while (u := stack[u]) not in out and i < step:
        out.append(u)
        i += 1
    return out


# %%
def run(stack: dict[int, int], start: int, iters: int) -> dict[int, int]:
    stack = stack.copy()
    curr = start
    for _ in range(iters):
        picked = pick(stack, curr)
        stack[curr] = stack[picked[-1]]
        dest = curr
        while (dest := mod1(dest - 1, len(stack))) in picked:
            ...
        stack[dest], stack[picked[-1]], = (
            picked[0],
            stack[dest],
        )
        curr = stack[curr]
    return stack


def test1() -> None:
    stack = {raw[i]: raw[i + 1] for i in range(len(raw) - 1)}
    stack[raw[-1]] = raw[0]
    out = run(stack, raw[0], 100)
    assert "".join(map(str, iterate(out, 1)))[1:] == "82635947"


def test2() -> None:
    stack = {i: i + 1 for i in range(len(raw) + 1, int(1e6))}
    stack |= {raw[i]: raw[i + 1] for i in range(len(raw) - 1)}
    stack[raw[len(raw) - 1]] = len(raw) + 1
    stack[int(1e6)] = raw[0]
    out = run(stack, raw[0], int(1e7))
    assert prod(iterate(out, 1, step=2)[1:]) == 157047826689


# %%
