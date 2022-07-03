#%%
import numpy as np

from utils import read

ex = """0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
""".splitlines()


#%%
def parse(x: str) -> np.ndarray:
    f, _, t = x.split(" ")
    return np.fromiter(list(map(int, f.split(","))) + list(map(int, t.split(","))), dtype=np.uint16)


raw = read("../../data/2021/day05.txt")
# raw = ex
raw = np.array(list(map(parse, raw)))
m = np.max(raw)

# %%
def fromto(row: np.ndarray) -> list[np.ndarray]:
    row = row.copy()
    ori = row[:2]
    dst = row[2:]
    if ori[1] > dst[1]:
        ori, dst = dst, ori

    out = [ori.copy()]
    curr = ori
    add = np.array([1, 1], dtype=np.uint16) if dst[0] > ori[0] else np.array([-1, 1], dtype=np.uint16)

    while curr[0] != dst[0] or curr[1] != dst[1]:
        curr += add
        out.append(curr.copy())
    return out


arr = np.zeros((m + 1, m + 1), dtype=np.uint16)
for row in raw:
    ys = [min(row[1], row[3]), max(row[1], row[3]) + 1]
    xs = [min(row[0], row[2]), max(row[0], row[2]) + 1]

    if row[0] == row[2] or row[1] == row[3]:
        arr[np.s_[ys[0] : ys[1], xs[0] : xs[1]]] += 1
    else:
        for s in fromto(row):
            arr[np.s_[s[1], s[0]]] += 1


# %%
np.count_nonzero(arr > 1)
# %%
