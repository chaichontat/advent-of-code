#%%
from utils import load

raw = load("day15.txt", split=",", parseint=True)

# %%
def run(stps: int) -> int:
    if stps <= len(raw):
        return raw[stps - 1]
    elif stps < 1:
        raise ValueError

    last_spoke = {x: i for i, x in enumerate(raw, start=1)}  # n: last spoke
    speaking = raw[-1]

    for i in range(len(raw), stps):
        last_spoke[speaking], speaking = i, i - last_spoke.get(speaking, i)

    return speaking


def test1() -> None:
    assert run(2020) == 1015


def test2() -> None:
    assert run(int(3e7)) == 201


# %%
