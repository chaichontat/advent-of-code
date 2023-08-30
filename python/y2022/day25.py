# %%
import math
from pathlib import Path

raw = (
    Path("data/2022/" + __file__.split("/")[-1].split(".")[0] + ".txt")
    .read_text()[:-1]
    .splitlines()
)

KEY = {"2": 2, "1": 1, "0": 0, "-": -1, "=": -2}


def parse(snafu: str):
    acc = 0
    for i, c in enumerate(reversed(snafu)):
        acc += 5**i * KEY[c]
    return acc


# %%
SN = "=-012"


def stepup(snafu: str):
    if len(snafu) > 1 and snafu.startswith("0"):
        return stepup(snafu[1:])
    if snafu == "2":
        return "1="
    if snafu == "":
        return "1"
    if (c := snafu[-1]) in SN[:-1]:
        return snafu[:-1] + SN[SN.index(c) + 1]
    return stepup(snafu[:-1]) + "="


assert parse(stepup("12222222")) == parse("12222222") + 1


# %%
# 5 ** (p + 1) - 5**p == 4 * 5**p
def parse_back(r: int):
    p = math.floor(math.log(r, 5))
    out = ""
    while p >= 0:
        q, r = divmod(r, 5**p)
        if q == 4:
            out = stepup(out) + "-"
        if q == 3:
            out = stepup(out) + "="
        if q <= 2:
            out += str(q)
        p -= 1
    return out


print(parse_back(sum(map(parse, raw))))


# %%
