# %%
from pathlib import Path

from utils import fmap

raw = (
    Path("data/2022/" + __file__.split("/")[-1].split(".")[0] + ".txt")
    .read_text()[:-1]
    .splitlines()
)

# raw = """R 5
# U 8
# L 8
# D 3
# R 17
# D 10
# L 25
# U 20""".splitlines()

raw = fmap(lambda x: x.split(" "), raw)
# %%


def move_tail(hpos: tuple[int, int], tpos: tuple[int, int]):
    d = (tpos[0] - hpos[0], tpos[1] - hpos[1])
    correction = (d[0] // abs(d[0]) if d[0] else 0, d[1] // abs(d[1]) if d[1] else 0)

    if all(-1 <= x <= 1 for x in d):
        return tpos

    if any(x < -1 or x > 1 for x in d):
        return (tpos[0] - correction[0], tpos[1] - correction[1])

    if d[0]:
        return (tpos[0] - correction[0], tpos[1])
    if d[1]:
        return (tpos[0], tpos[1] - correction[1])

    assert False


def move_head(h: tuple[int, int], direction: str):
    if direction == "R":
        return (h[0], h[1] + 1)
    elif direction == "L":
        return (h[0], h[1] - 1)
    elif direction == "U":
        return (h[0] - 1, h[1])
    elif direction == "D":
        return (h[0] + 1, h[1])
    assert False


visited = set()
head = (0, 0)
tail: tuple[int, int] = (0, 0)
visited.add(tail)

for direction, steps in raw:
    steps = int(steps)
    for _ in range(steps):
        head = move_head(head, direction)
        tail = move_tail(head, tail)
        assert all(-1 <= x <= 1 for x in [head[0] - tail[0], head[1] - tail[1]])
        visited.add(tail)

print("Part 1:", len(visited))

# %%
visited = set()
visited.add((0, 0))
chain: list[tuple[int, int]] = [(0, 0)] * 10
for direction, steps in raw:
    steps = int(steps)
    for _ in range(steps):
        chain[0] = move_head(chain[0], direction)
        for i in range(1, len(chain)):
            h, t = chain[i - 1], chain[i]
            chain[i] = move_tail(h, t)
        visited.add(chain[-1])

print("Part 2:", len(visited))
