# %%
from collections import defaultdict
from pathlib import Path

raw = (
    Path("data/2022/" + __file__.split("/")[-1].split(".")[0] + ".txt")
    .read_text()[:-1]
    .splitlines()
)
cmds = [x.split(" ") for x in raw]

# %%
cwd = [""]
dirs = defaultdict(int)

for c in cmds:
    if c[0].isnumeric():
        for i in range(1, len(cwd) + 1):
            dirs["/".join(cwd[:i])] += int(c[0])
        continue

    if c[0] == "dir":
        continue

    match c[1]:
        case "ls":
            continue
        case "cd":
            if c[2] == "..":
                cwd.pop()
                assert cwd
            elif c[2] == "/":
                cwd = [""]
            else:
                cwd.append(c[2])
            continue


# %%
print("Part 1:", sum(x for x in dirs.values() if x < 100000))

needed = 30000000 - (70000000 - dirs[""])
print("Part 2:", min(x for x in dirs.values() if x > needed))
# %%
