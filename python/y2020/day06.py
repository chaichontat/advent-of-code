#%%
from utils import load

raw = load("day06.txt", split="\n\n")

# %%
def test1() -> None:
    assert sum([len(set(group) - {"\n"}) for group in raw]) == 6521


# %%
def intersect(group: str) -> int:
    return len(set.intersection(*[set(x) for x in group.split("\n")]))


def test2() -> None:
    assert sum(map(intersect, raw)) == 3305


# %%
