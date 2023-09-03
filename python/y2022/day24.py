# %%
import operator as op
from collections import deque
from functools import reduce
from pathlib import Path

import numpy as np

raw = (
    Path("data/2022/" + __file__.split("/")[-1].split(".")[0] + ".txt")
    .read_text()[:-1]
    .split("\n")
)

# raw = """#.######
# #>>.<^<#
# #.<..<<#
# #>v.><>#
# #<^v^^>#
# ######.#""".splitlines()


class Runner:
    DIRS = {
        k: np.array(v)
        for k, v in {"<": (0, -1), ">": (0, 1), "^": (-1, 0), "v": (1, 0)}.items()
    }

    def __init__(
        self, board: np.ndarray, *, start: tuple[int, int], end: tuple[int, int]
    ) -> None:
        self.board = board
        self.shape = self.board.shape

        self.poss = [np.transpose(np.where(self.board == x)) for x in self.DIRS.keys()]
        self.start = start
        self.end = end

    def step(self):
        for arr, d in zip(self.poss, self.DIRS.values()):
            arr += d
            arr[:, 0] %= self.shape[0]
            arr[:, 1] %= self.shape[1]
        return self.get_coords()

    def get_coords(self) -> set[tuple[int, int]]:
        return reduce(op.or_, [set(map(tuple, arr)) for arr in self.poss])

    def __str__(self) -> str:
        coords = self.get_coords()
        return "\n".join(
            "".join("#" if (y, x) in coords else "." for x in range(self.shape[1]))
            for y in range(self.shape[0])
        )

    def bfs(self, max_steps: int = 1000):
        steps: tuple[deque[tuple[tuple[int, int], int]], ...] = (
            deque([(self.start, 0)]),
            deque(),
        )
        i = 0

        for _ in range(max_steps):
            blizzards = self.step()
            checked = set()

            while steps[i & 1]:
                curr, dist = steps[i & 1].popleft()

                if curr not in blizzards:
                    steps[1 - (i & 1)].append((curr, dist))
                    checked.add(curr)

                curr = np.array(curr)

                for d in self.DIRS.values():
                    nxt = tuple(curr + d)
                    if nxt in checked:
                        continue
                    checked.add(nxt)

                    if nxt == self.end:
                        # need to put this here because exit is technically out of bounds
                        return i + 1

                    if nxt[0] < 0 or nxt[1] < 0:
                        continue
                    if nxt[0] >= self.shape[0] or nxt[1] >= self.shape[1]:
                        continue
                    if nxt not in blizzards:
                        steps[1 - (i & 1)].append((nxt, dist + 1))

            i += 1

        return -1


# %%
time = 0
board = np.array([list(r)[1:-1] for r in raw[1:-1]])
runner = Runner(board, start=(-1, 0), end=(board.shape[0], board.shape[1] - 1))
time += runner.bfs()
print("Part 1:", time)

# %%
runner.start, runner.end = runner.end, runner.start
time += runner.bfs()

# %%
runner.start, runner.end = runner.end, runner.start
time += runner.bfs()
print("Part 2:", time)
