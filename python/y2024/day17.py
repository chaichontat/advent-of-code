# %%
import re
from pathlib import Path

raw = """Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0"""
raw = raw = Path(
    "../../data/2024/" + __file__.split("/")[-1].split(".")[0] + ".txt"
).read_text()


def parse(data: str):
    a, b, c, program = re.findall(r"([\d+,]+)", data)
    return int(a), int(b), int(c), list(map(int, program.split(",")))


# %%


def _run(a: int, b: int, c: int, program: list[int]):
    ip = 0
    while 0 <= ip and ip + 1 < len(program):
        instruction = program[ip]
        operand = program[ip + 1]
        combo = (0, 1, 2, 3, a, b, c)[operand]
        match instruction:
            case 0:
                a = a >> combo
            case 1:
                b = b ^ operand
            case 2:
                b = combo % 8
            case 3:
                if a:
                    ip = operand
                    continue
            case 4:
                b = b ^ c
            case 5:
                yield combo % 8
            case 6:
                b = a >> combo
            case 7:
                c = a >> combo
            case _:
                raise NotImplementedError
        ip += 2


",".join(map(str, _run(*parse(raw))))
# %%
