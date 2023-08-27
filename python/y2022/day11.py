# %%
import math
import re
from collections import defaultdict, deque
from pathlib import Path

raw = (
    Path("data/2022/" + __file__.split("/")[-1].split(".")[0] + ".txt")
    .read_text()[:-1]
    .split("\n\n")
)

# raw = """Monkey 0:
#   Starting items: 79, 98
#   Operation: new = old * 19
#   Test: divisible by 23
#     If true: throw to monkey 2
#     If false: throw to monkey 3

# Monkey 1:
#   Starting items: 54, 65, 75, 74
#   Operation: new = old + 6
#   Test: divisible by 19
#     If true: throw to monkey 2
#     If false: throw to monkey 0

# Monkey 2:
#   Starting items: 79, 60, 97
#   Operation: new = old * old
#   Test: divisible by 13
#     If true: throw to monkey 1
#     If false: throw to monkey 3

# Monkey 3:
#   Starting items: 74
#   Operation: new = old + 3
#   Test: divisible by 17
#     If true: throw to monkey 0
#     If false: throw to monkey 1""".split(
#     "\n\n"
# )


def extract(monkey: str):
    data = monkey.split("\n")[1:]
    return {
        "items": deque(map(int, re.findall(r"\d+", data[0]))),
        "ops": eval("lambda old: " + data[1].split(" = ")[1]),
        "test": int(data[2].split(" ")[-1]),
        "true": int(data[3].split(" ")[-1]),
        "false": int(data[4].split(" ")[-1]),
    }


processed = list(map(extract, raw))

rounds = 10000


# curr = 0
def run(rounds: int, reduction: int):
    normalizer = math.lcm(*[x["test"] for x in processed])
    counts = defaultdict(int)
    for _ in range(rounds):
        for curr in range(len(processed)):
            while items := processed[curr]["items"]:
                old = processed[curr]["ops"](items.popleft())
                new = (old % normalizer) // reduction
                if new % processed[curr]["test"] == 0:
                    processed[processed[curr]["true"]]["items"].append(new)
                else:
                    processed[processed[curr]["false"]]["items"].append(new)
                counts[curr] += 1

    *_, x, y = sorted(counts.values())
    return x * y


print("Part 1:", run(20, 3))
print("Part 2:", run(10000, 1))

# %%
