# %%
from functools import cmp_to_key
from pathlib import Path

from utils import fmap

raw = """47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47""".split("\n\n")
raw = (
    Path("../../data/2024/" + __file__.split("/")[-1].split(".")[0] + ".txt")
    .read_text()
    .split("\n\n")
)
raw[1] = fmap(lambda x: fmap(int, x.split(",")), raw[1].splitlines())

# %%
ins = set(map(lambda x: tuple(map(int, x.split("|"))), raw[0].splitlines()))


# %%
# %%
def is_valid_sequence(sequence: list[int]):
    for i, node1 in enumerate(sequence[:-1]):
        # We don't care about the last node
        for node2 in sequence[i + 1 :]:
            if (node1, node2) not in ins:
                return 0
    return sequence[len(sequence) // 2]


# Check each update sequence
valid_count = 0
for update in raw[1]:
    valid_count += is_valid_sequence(update)
valid_count
# %%


# %%
def correction(update):
    return sorted(
        update,
        key=cmp_to_key(lambda x, y: -1 if (x, y) in ins else 1 if (y, x) in ins else 0),
    )


# %%
baddies = list(filter(lambda x: is_valid_sequence(x) == 0, raw[1]))
sum(is_valid_sequence(correction(b)) for b in baddies)
# %%
