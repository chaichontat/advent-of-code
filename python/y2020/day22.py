#%%
from copy import deepcopy

from utils import load

raw = load("day22.txt", split="\n\n")
players = {i: list(map(int, x.split(":\n")[1].split("\n"))) for i, x in enumerate(raw, start=1)}
Stacks = dict[int, list[int]]
# %%
def run(stacks: Stacks) -> tuple[Stacks, int]:
    stacks = deepcopy(stacks)
    while stacks[1] and stacks[2]:
        x, y = [x.pop(0) for x in stacks.values()]
        if x > y:
            stacks[1].extend((x, y))
        elif y > x:
            stacks[2].extend((y, x))
        else:
            raise ValueError
    return stacks, int(len(stacks[2]) > 0) + 1


# %%
def score(cards: list[int]) -> int:
    return sum(v * s for v, s in zip(cards, range(len(cards), 0, -1)))


def test1() -> None:
    res, winner = run(players)
    assert score(res[winner]) == 32677


# %%
def check_already(already: set[tuple[tuple[int, ...], ...]], stacks: Stacks) -> bool:
    curr = tuple(tuple(cards) for cards in stacks.values())
    if curr in already:
        return True
    already.add(curr)
    return False


def modify_stack(stacks: Stacks, winner: int, x: int, y: int) -> None:
    if winner == 1:
        stacks[1].extend((x, y))
    elif winner == 2:
        stacks[2].extend((y, x))
    else:
        raise ValueError


def recurse(stacks: Stacks) -> tuple[Stacks, int]:
    stacks = deepcopy(stacks)
    already: set[tuple[tuple[int, ...], ...]] = set()
    while stacks[1] and stacks[2]:
        if check_already(already, stacks):
            return stacks, 1

        x, y = stacks[1].pop(0), stacks[2].pop(0)
        if len(stacks[1]) >= x and len(stacks[2]) >= y:
            _, winner = recurse({1: stacks[1][:x], 2: stacks[2][:y]})
        else:
            winner = int(y > x) + 1

        modify_stack(stacks, winner, x, y)

    return stacks, int(bool(stacks[2])) + 1


def test2() -> None:
    res, winner = recurse(players)
    assert score(res[winner]) == 33661


# %%
