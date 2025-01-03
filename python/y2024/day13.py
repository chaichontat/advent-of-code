# %%
import re
from pathlib import Path

import numpy as np

raw = """Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"""
raw = Path(
    "../../data/2024/" + __file__.split("/")[-1].split(".")[0] + ".txt"
).read_text()
raw = raw.split("\n\n")


def parse(s: str):
    match = re.match(
        r"""Button A: X([-+]?\d+), Y([-+]?\d+)
Button B: X([-+]?\d+), Y([-+]?\d+)
Prize: X=([-+]?\d+), Y=([-+]?\d+)""",
        s,
    )
    assert match is not None
    A = np.array([
        [int(match.group(1)), int(match.group(3))],
        [int(match.group(2)), int(match.group(4))],
    ])
    b = np.array([int(match.group(5)), int(match.group(6))])
    return A, b


acc = 0
for r in raw:
    A, b = parse(r)
    sol = np.linalg.solve(A, b + 10000000000000)
    # Floating point precision loss
    if np.allclose(np.round(sol), sol, rtol=1e-15):
        acc += sol[0] * 3 + sol[1]
acc

# %%


def cramer(A, b):
    """
    Solve 2x2 system Ax=b using integer Cramer's rule
    Only 0, 1, or infinite solutions for linear equations
    """
    a1, b1 = A[0]  # first row
    a2, b2 = A[1]  # second row
    c1, c2 = b  # target vector

    # fmt: off
    a_num   = b2 * c1 - b1 * c2
    a_den   = a1 * b2 - b1 * a2
    b_num   = a2 * c1 - a1 * c2
    b_den   = a2 * b1 - a1 * b2
    # fmt: on

    if a_num % a_den or b_num % b_den:
        return [0, 0]

    return [a_num // a_den, b_num // b_den]


acc = 0
for r in raw:
    A, b = parse(r)
    sol = cramer(A, b + 10000000000000)
    if np.allclose(np.round(sol), sol, rtol=1e-15):
        acc += sol[0] * 3 + sol[1]
acc

# %%
