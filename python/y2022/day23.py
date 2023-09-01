# %%
from collections import Counter, deque
from dataclasses import dataclass
from pathlib import Path

raw = (
    Path("data/2022/" + __file__.split("/")[-1].split(".")[0] + ".txt")
    .read_text()[:-1]
    .split("\n")
)

# raw = """..............
# ..............
# .......#......
# .....###.#....
# ...#...#.#....
# ....#...##....
# ...#.###......
# ...##.#.##....
# ....#..#......
# ..............
# ..............
# ..............""".splitlines()


# %%
# fmt: off
NEIGHBORS = {
    "N":  lambda x, y: (x    , y - 1),
    "NE": lambda x, y: (x + 1, y - 1),
    "E":  lambda x, y: (x + 1, y),
    "SE": lambda x, y: (x + 1, y + 1),
    "S":  lambda x, y: (x    , y + 1),
    "SW": lambda x, y: (x - 1, y + 1),
    "W":  lambda x, y: (x - 1, y),
    "NW": lambda x, y: (x - 1, y - 1),
}
# fmt: on


@dataclass
class Elves:
    elves: set[tuple[int, int]]

    def test_factory(self, tests: list[str], propose: str):
        def test(pos: tuple[int, int]) -> tuple[int, int] | None:
            if all(NEIGHBORS[direction](*pos) not in self.elves for direction in tests):
                return NEIGHBORS[propose](*pos)

        return test

    def __init__(self, elves: set[tuple[int, int]]):
        self.elves = elves
        self.tests = deque(
            [
                self.test_factory(["N", "NE", "NW"], "N"),
                self.test_factory(["S", "SE", "SW"], "S"),
                self.test_factory(["W", "NW", "SW"], "W"),
                self.test_factory(["E", "NE", "SE"], "E"),
            ]
        )
        self.blank_test = self.test_factory(list(NEIGHBORS.keys()), "N")

    def move(self):
        proposed = {}
        for elf in self.elves:
            if self.blank_test(elf) is not None:
                continue
            for scr in self.tests:
                if (prop := scr(elf)) is not None:
                    proposed[elf] = prop
                    break

        self.tests.rotate(-1)

        # Now move
        counter = Counter(proposed.values())
        for ori, prop in proposed.items():
            if counter[prop] > 1:
                continue

            assert prop not in self.elves
            self.elves.remove(ori)
            self.elves.add(prop)

    def __str__(self) -> str:
        min_x = 0
        max_x = max(x for x, y in self.elves)
        min_y = 0
        max_y = max(y for x, y in self.elves)

        return "\n".join(
            "".join(
                "#" if (x, y) in self.elves else "." for x in range(min_x, max_x + 1)
            )
            for y in range(min_y, max_y + 1)
        )


# %%
elves = {(x, y) for y, line in enumerate(raw) for x, c in enumerate(line) if c == "#"}
e = Elves(elves)

i = 0
while True:
    curr = e.elves.copy()
    e.move()
    i += 1

    if i == 10:
        min_x = min(x for x, y in e.elves)
        max_x = max(x for x, y in e.elves)
        min_y = min(y for x, y in e.elves)
        max_y = max(y for x, y in e.elves)
        rng = ((max_x + 1 - min_x), (max_y + 1 - min_y))
        print("Part 1:", rng[0] * rng[1] - len(e.elves))

    if curr == e.elves:
        print("Part 2:", i)
        break
# %%
