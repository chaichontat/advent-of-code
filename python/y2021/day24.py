#%%
from collections import deque
from utils import read

parse = lambda x: x.split(" ")
raw = read("../../data/2021/day24.txt", parse=parse)
# %%
def f(x: str) -> int:
    if x.isalpha():
        return globals()[x]
    return int(x)


w, x, y, z = 0, 0, 0, 0
for ins, *args in raw:
    match ins:
        case "inp":
            globals()[args[0]] = f(input())
        case "add":
            globals()[args[0]] += f(args[1])
        case "mul":
            globals()[args[0]] *= f(args[1])
        case "div":
            globals()[args[0]] //= f(args[1])
        case "mod":
            globals()[args[0]] %= f(args[1])
        case "eql":
            globals()[args[0]] = f(args[0]) == f(args[1])
        case _:
            raise ValueError(f"Unknown instruction: {ins}")

# %%
