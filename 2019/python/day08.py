#%%
from pathlib import Path

import numpy as np

path = Path(__file__).parent.parent / "data" / "day08.txt"
data = path.read_text()[:-1]


def part1():
    arr = np.array(list(data), dtype=int).reshape((-1, 6, 25))
    clipped = np.clip(arr, 0, 1)
    idx_max = np.argmax(np.sum(clipped, axis=(1, 2)))
    return np.sum(arr[idx_max] == 1) * np.sum(arr[idx_max] == 2)


def test08a(benchmark):
    assert benchmark(part1) == 1950


# %% Part 1

# %% Part 2 Highest non-two


def part2():
    arr = np.array(list(data), dtype=int).reshape((-1, 6, 25))
    flattened = np.zeros((6, 25))
    for i in range(6):
        for j in range(25):
            flattened[i, j] = arr[np.argmax(arr[:, i, j] != 2), i, j]


def test08b(benchmark):
    benchmark(part2)  # Image recognition.


# %%
