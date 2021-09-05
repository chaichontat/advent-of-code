#%%
import copy
import io
from itertools import combinations, product
from math import prod
from typing import Callable, Optional

import numpy as np
import numpy.typing as npt
from scipy.signal import correlate2d

from utils import NDArrayInt, load

NDArrayBool = npt.NDArray[np.bool_]
tiles = {int((u := tile.split("\n"))[0][5:9]): u[1:] for tile in load("day20.txt", split="\n\n")}


def borders(tile: list[str]) -> dict[str, str]:
    """D4 combined with flip operations."""
    return {
        "U": tile[0],
        "D": tile[-1],
        "L": "".join(line[0] for line in tile),
        "R": "".join(line[-1] for line in tile),
    }


def initial_match(t1: list[str], t2: list[str]) -> bool:
    return any(b1 == b2 or b1 == b2[::-1] for b1, b2 in product(borders(t1).values(), borders(t2).values()))


init_match = list()
counts = {x: 0 for x in tiles}
for i, j in combinations(tiles, 2):
    if initial_match(tiles[i], tiles[j]):
        counts[i] += 1
        counts[j] += 1
        init_match.append((i, j))


def test1() -> None:
    assert prod(x for x in counts if counts[x] == 2) == 45079100979683


# We have an adjacency list.
# Now, we have to embed said list into a grid.

#%%


class Jigsaw:
    d4: dict[int, Callable[[NDArrayBool], NDArrayBool]] = {
        0: lambda x: x,  # Identity
        1: lambda x: np.rot90(x, k=1),  # type: ignore [no-untyped-call, no-any-return] # Rotations.
        2: lambda x: np.rot90(x, k=2),  # type: ignore [no-untyped-call, no-any-return]
        3: lambda x: np.rot90(x, k=3),  # type: ignore [no-untyped-call, no-any-return]
        4: lambda x: np.fliplr(x),  # type: ignore [no-untyped-call, no-any-return] # Flips.
        5: lambda x: np.flipud(x),  # type: ignore [no-untyped-call, no-any-return]
        6: lambda x: np.fliplr(np.rot90(x)),  # type: ignore [no-untyped-call, no-any-return]  # Diagonal flips.
        7: lambda x: np.flipud(np.rot90(x)),  # type: ignore [no-untyped-call, no-any-return]
    }

    sides = {
        "U": np.s_[0, :],
        "L": np.s_[:, 0],
        "R": np.s_[:, -1],
        "D": np.s_[-1, :],
    }

    pairs = {"D": "U", "R": "L", "L": "R", "U": "D"}

    def __init__(self, tiles: dict[int, NDArrayBool]) -> None:
        self.tiles = tiles
        self.size = int(np.sqrt(len(tiles)))

        self.rotations: dict[int, int] = {}
        self.chosen: dict[int, NDArrayBool] = {}
        self.contacts = {k: {"U": 0, "L": 0, "R": 0, "D": 0} for k in tiles}  # U L R D
        self.exp_edges = 2 * (self.size ** 2 - self.size)

    def pairings(self, a1: int, a2: int, el: Optional[int] = None) -> Optional[str]:
        a1_ = self.chosen[a1]
        a2_ = self.chosen[a2] if el is None else self.d4[el](self.tiles[a2])

        for p1, p2 in self.pairs.items():  # UDLR
            if self.contacts[a1][p1] or self.contacts[a2][p2]:
                continue

            if np.all(a1_[self.sides[p1]] == a2_[self.sides[p2]]):
                return p1
        return None

    def check_new(self, node: int, working: int) -> bool:
        """Find aligned edges by flipping tile[working] around."""
        for n in self.d4:
            if pair := self.pairings(node, working, n):
                self.contacts[node][pair] = working
                self.contacts[working][self.pairs[pair]] = node

                self.rotations[working] = n
                self.chosen[working] = self.d4[n](self.tiles[working])
                return True
        return False

    def check_existing(self, node: int, working: int) -> bool:
        """Find matches on all 4 sides."""
        if pair := self.pairings(node, working):
            self.contacts[node][pair] = working
            self.contacts[working][self.pairs[pair]] = node
            return True
        return False

    def match(self, node: int, working: int) -> None:
        # Node orientation must be known.
        assert node in self.chosen

        if working not in self.chosen:
            assert self.check_new(node, working)
        else:
            assert self.check_existing(node, working)

    def run(self, edges: list[tuple[int, int]]) -> None:
        edges = copy.deepcopy(edges)
        self.chosen = {(k := edges[0][0]): self.tiles[k]}
        self.rotations[k] = 0

        while edges:  # Cycle
            working = edges.pop(0)
            if working[0] in self.chosen:
                self.match(*working)
            elif working[1] in self.chosen:
                self.match(*tuple(reversed(working)))
            else:
                edges.append(working)


raw = load("day20.txt", split=None).replace("#", "1 ").replace(".", "0 ").strip().split("\n\n")


def process_np() -> dict[int, NDArrayBool]:
    tiles: dict[int, NDArrayBool] = dict()
    for tile in raw:
        n, arr = tile.strip().split(":\n")
        n = int(n.split()[1])
        with io.StringIO(arr) as f:
            tiles[n] = np.loadtxt(f, dtype=bool)  # type: ignore [no-untyped-call]
    return tiles


jig = Jigsaw(process_np())
jig.run(init_match)


#%% Assemble.
def recurse(n: int, hdg: str) -> list[int]:
    if (u := jig.contacts[n][hdg]) == 0:
        return [n]
    return [n] + recurse(u, hdg)


dim_chunk = len(tiles[1783]) - 2
n_chunk = int(np.sqrt(len(tiles)))
dim = dim_chunk * n_chunk
img = np.zeros((dim, dim))

# Find top left and make first column.
topleft = [k for k, v in jig.contacts.items() if v["U"] == 0 and v["L"] == 0][0]
lefts = recurse(topleft, "D")

for i in range(0, dim, dim_chunk):
    # Then, fill each row.
    target = recurse(lefts[i // dim_chunk], "R")
    for j in range(0, dim, dim_chunk):
        img[i : i + dim_chunk, j : j + dim_chunk] = jig.chosen[target[j // dim_chunk]][1:-1, 1:-1]

#%% Find monster.
kernel = (
    """                  # 
#    ##    ##    ###
 #  #  #  #  #  #   """.replace(
        " ", "."
    )
    .replace("#", "1 ")
    .replace(".", "0 ")
    .strip()
)

with io.StringIO(kernel) as f:
    kernel: npt.NDArray[np.bool_] = np.loadtxt(f, dtype=bool)  # type: ignore [no-redef, no-untyped-call]


def test2() -> None:
    size_monster = np.sum(kernel)
    n_monster = [
        np.sum(correlate2d(op(img), kernel, mode="valid") == size_monster) for op in Jigsaw.d4.values()
    ]

    assert np.sum(img) - max(n_monster) * size_monster == 1946


# %%
