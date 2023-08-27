# %%
from pathlib import Path

raw = (
    Path("data/2022/" + __file__.split("/")[-1].split(".")[0] + ".txt")
    .read_text()[:-1]
    .splitlines()
)
ins = [x.split(" ") for x in raw]


def runner(ins: list[list[str]]):
    register = 1
    for i in ins:
        match i[0]:
            case "noop":
                yield register
            case "addx":
                yield register
                yield register
                register += int(i[1])
            case x:
                raise ValueError(f"Unknown instruction {x}")


res = 0
for i, reg in enumerate(runner(ins), 1):
    if i in [20, 60, 100, 140, 180, 220]:
        res += reg * i

print(res)

# %%
out = "".join(
    "#" if reg - 1 <= i % 40 <= reg + 1 else " " for i, reg in enumerate(runner(ins))
)

for i in range(0, len(out), 40):
    print(out[i : i + 40])
# %%
