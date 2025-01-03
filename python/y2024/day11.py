# %%
from collections import defaultdict

from utils import load

# raw = """125 17""".split(" ")
raw = load(__file__).split(" ")

d = defaultdict(int)
for x in raw:
    d[x] += 1


def blink():
    # If the stone is engraved with the number 0, it is replaced by a stone engraved with the number 1.
    # If the stone is engraved with a number that has an even number of digits, it is replaced by two stones. The left half of the digits are engraved on the new left stone, and the right half of the digits are engraved on the new right stone. (The new numbers don't keep extra leading zeroes: 1000 would become stones 10 and 0.)
    # If none of the other rules apply, the stone is replaced by a new stone; the old stone's number multiplied by 2024 is engraved on the new stone.
    new = defaultdict(int)
    n = d.get("0", 0)
    new["1"] += n

    for k, v in d.items():
        if k == "0":
            continue
        if len(k) & 1 == 0:
            new[str(int(k[: len(k) // 2]))] += v
            new[str(int(k[len(k) // 2 :]))] += v
        else:
            new[str(int(k) * 2024)] += v

    return new


print(d)
for _ in range(75):
    d = blink()
# %%
print(sum(d.values()))
# %%
