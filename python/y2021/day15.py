#%%
import heapq

import numpy as np

from utils import read

raw = read("../../data/2021/day15.txt")
ex = """1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581
""".splitlines()

# raw = ex
arr = np.array([np.fromiter(r, dtype=np.uint8) for r in raw])
# %%


def neighbors(a: np.ndarray, i: int, j: int) -> list[tuple[int, int]]:
    out = []
    if i > 0:
        out.append((i - 1, j))
    if i < a.shape[0] - 1:
        out.append((i + 1, j))
    if j > 0:
        out.append((i, j - 1))
    if j < a.shape[1] - 1:
        out.append((i, j + 1))
    return out


def dijkstra(arr: np.ndarray) -> int:
    h = []
    seen = set()
    dist = np.ones_like(arr, dtype=np.uint32) * np.iinfo(np.uint16).max
    dist[0, 0] = 0
    for i in range(arr.shape[0]):
        for j in range(arr.shape[1]):
            heapq.heappush(h, (dist[i, j], (i, j)))

    while h:
        d, loc = heapq.heappop(h)
        nh = neighbors(arr, *loc)
        for n in nh:
            if n in seen:
                continue
            seen.add(n)
            if arr[n] < dist[n]:
                dist[n] = arr[n] + d
                heapq.heappush(h, (dist[n], n))
    return dist[-1, -1]


# %%
def addone(x: np.ndarray) -> np.ndarray:
    return np.clip((x + 1) % 10, 1, 9)


# %%
out = arr
temp = arr
for i in range(4):
    temp = addone(temp)
    out = np.hstack([out, temp])
temp = out
for i in range(4):
    temp = addone(temp)
    out = np.vstack([out, temp])

# %%
dijkstra(out)
# %%
