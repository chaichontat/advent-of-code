#%%
from pathlib import Path

import numpy as np


def process(s: str):
    """Convert raw data to an array of asteroid coordinates."""
    s = s.replace(".", "0")
    s = s.replace("#", "1")
    s = s.split()
    s = [list(t) for t in s]
    s = np.asarray(s, dtype=np.int)
    s = np.array(np.where(s == 1)).T
    s[:, [0, 1]] = s[:, [1, 0]]  # Swap x, y

    return s


def calc_polar(ori, target):
    """Returns l2 distance w/ angle in rad."""
    coords = ori.copy()
    coords[:, 0] -= target[0]
    coords[:, 1] -= target[1]
    return (
        np.sqrt(coords[:, 0] ** 2 + coords[:, 1] ** 2),
        np.arctan2(coords[:, 1], coords[:, 0]),
        ori[:, 0],
        ori[:, 1],
    )


def calc_num_asteroid(coords: np.ndarray, target: tuple):
    return len(np.unique(calc_polar(coords, target)[1]))


def get_largest_num(coords):
    n_max, coord_max = 0, tuple()
    for coord in coords:
        if (n_this := calc_num_asteroid(coords, coord)) > n_max:
            n_max = n_this
            coord_max = coord
    return n_max, coord_max


def sort_for_scan(arr):
    """Need input to be presorted with increasing angle and decreasing distance."""
    already = set()
    order = []
    i = arr.shape[0] - 1
    while len(order) < arr.shape[0]:

        if i not in order and arr[i, 1] not in already:
            order.append(i)
            already.add(arr[i, 1])

        i -= 1
        if i == -1:
            i += arr.shape[0]
            already = set()

    return order


path = Path(__file__).parent.parent / "data" / "day10.txt"
loc = path.read_text()


def test10a(benchmark):
    assert benchmark(lambda: get_largest_num(process(loc))[0]) == 269


# Part 2
def part2():
    s = process(loc)
    polar = np.array(calc_polar(s, get_largest_num(s)[1])).T  # 0 rad at (1, 0).
    polar[:, 1] *= -1  # Flip y-axis.
    polar[polar < 0] += 2 * np.pi  # Set range to [0, 2π].

    # Rotate
    polar[:, 1] -= np.pi / 2
    polar[polar[:, 1] <= 0, 1] += 2 * np.pi

    polar = polar[np.lexsort((-polar[:, 0], polar[:, 1]))]

    polar = polar[sort_for_scan(polar), ...]

    return polar[199, 2] * 100 + polar[199, 3]


def test10b(benchmark):
    assert benchmark(part2) == 612


# %%
