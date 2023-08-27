# %%
import re
from pathlib import Path

import numpy as np
import seaborn as sns
from shapely import MultiPolygon, Polygon
from utils import fmap

sns.set()
raw = (
    Path("data/2022/" + __file__.split("/")[-1].split(".")[0] + ".txt")
    .read_text()[:-1]
    .split("\n")
)

# raw = """Sensor at x=2, y=18: closest beacon is at x=-2, y=15
# Sensor at x=9, y=16: closest beacon is at x=10, y=16
# Sensor at x=13, y=2: closest beacon is at x=15, y=3
# Sensor at x=12, y=14: closest beacon is at x=10, y=16
# Sensor at x=10, y=20: closest beacon is at x=10, y=16
# Sensor at x=14, y=17: closest beacon is at x=10, y=16
# Sensor at x=8, y=7: closest beacon is at x=2, y=10
# Sensor at x=2, y=0: closest beacon is at x=2, y=10
# Sensor at x=0, y=11: closest beacon is at x=2, y=10
# Sensor at x=20, y=14: closest beacon is at x=25, y=17
# Sensor at x=17, y=20: closest beacon is at x=21, y=22
# Sensor at x=16, y=7: closest beacon is at x=15, y=3
# Sensor at x=14, y=3: closest beacon is at x=15, y=3
# Sensor at x=20, y=1: closest beacon is at x=15, y=3""".splitlines()

coords = np.array(fmap(lambda x: fmap(int, re.findall(r"-?\d+", x)), raw))


# %%
def hamming(a: list[int], b: list[int]) -> np.ndarray:
    return sum(abs(x - y) for x, y in zip(a, b))


spots = fmap(tuple, coords[:, :2].tolist())
beacons = fmap(tuple, coords[:, 2:].tolist())
closest_dist_ = [
    hamming([coord[0], coord[1]], [coord[2], coord[3]]) for coord in coords
]
closest_dist = {}
for b, dist in zip(spots, closest_dist_):
    if b in closest_dist:
        closest_dist[b] = max(closest_dist[b], dist)
    else:
        closest_dist[b] = dist

# %%


def run_part1(y_: int):
    rng = [
        min(x - d for (x, y), d in closest_dist.items()),
        max(x + d for (x, y), d in closest_dist.items()),
    ]

    y_ = 10

    ok = {*range(rng[0], rng[1] + 1)}
    start = len(ok)
    for (x, y), dist in closest_dist.items():
        if (remaining := dist - abs(y - y_)) < 0:
            continue
        [ok.discard(x) for x in range(x - remaining, x + remaining + 1)]

    [ok.add(x) for x, y in beacons if y == y_]  # Remove beacons
    return start - len(ok)


print("Part 1:", run_part1(2000000))


# %%


def run_part2(bound: int):
    p = MultiPolygon(
        [
            Polygon(
                [(x - dist, y), (x + dist, y), (x, y - dist), (x, y + dist)]
            ).convex_hull
            for (x, y), dist in closest_dist.items()
        ]
    )

    bound = 4000000
    x, y = (
        Polygon([(0, 0), (0, bound), (bound, 0), (bound, bound)])
        .convex_hull.difference(p.buffer(0.7))  # sqrt(2) / 2
        .exterior.xy
    )
    return round(np.mean(x)) * bound + round(np.mean(y))


print("Part 2:", run_part2(4000000))
# %%
