#%%
from utils import read

arr = read("../../data/2021/day02.txt")

def parse(x: str) -> complex:
    direction, n = x.split(" ")
    match direction:
        case "up":
            return complex(0, -int(n))
        case "down":
            return complex(0, int(n))
        case "forward":
            return complex(int(n), 0)
        case _:
            raise ValueError(f"Unknown direction: {direction}")


u = list(map(parse, arr))
aim = 0j
curr = 0j
for x in u:
    aim += x.imag
    curr += x.real + aim * x.real * 1j

print((s:=sum(u)).real, s.imag)
print(curr.real * curr.imag)
