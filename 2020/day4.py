#%%
import re
from utils import load

raw = load("day4.txt", split="\n\n")

# %%
valid = list()
for line in raw:
    test = (
        len(re.findall("[bie]yr", line)) == 3,
        re.findall("hgt", line),
        len(re.findall("[he]cl", line)) == 2,
        re.findall("pid", line),
    )
    if all(test):
        valid.append(line)

len(valid)
# %%


def proc_hgt(x: str):
    if x.endswith("cm"):
        return 150 <= int(x[:-2]) <= 193
    elif x.endswith("in"):
        return 59 <= int(x[:-2]) <= 76
    return False


criteria = {
    "byr": lambda x: 1920 <= int(x) <= 2002,
    "iyr": lambda x: 2010 <= int(x) <= 2020,
    "eyr": lambda x: 2020 <= int(x) <= 2030,
    "hgt": proc_hgt,
    "hcl": lambda x: re.match("#[0-9a-f]{6}", x),
    "ecl": lambda x: x in "amb blu brn gry grn hzl oth".split(),
    "pid": lambda x: x.isnumeric() and len(x) == 9,
    "cid": lambda x: x,
}

valid2 = 0
for line in valid:
    fields = line.split()
    ok = True
    for field in fields:
        k, v = field.split(":")
        if not criteria[k](v):
            ok = False
            break
    if ok:
        valid2 += 1

print(valid2)
