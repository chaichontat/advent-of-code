# %%
from pathlib import Path

import networkx as nx
import numpy as np

raw = (
    Path("data/2022/" + __file__.split("/")[-1].split(".")[0] + ".txt")
    .read_text()[:-1]
    .splitlines()
)


arr = np.vectorize(ord)(list(map(list, raw)))
start = np.argwhere(arr == ord("S"))[0]
end = np.argwhere(arr == ord("E"))[0]

arr[*end] = ord("z")
arr[*start] = ord("a")
arr -= ord("a")
# %%

G = nx.DiGraph()


def check_one_less(i: int, j: int):
    if i > 0 and arr[i - 1, j] - arr[i, j] <= 1:
        G.add_edge((i, j), (i - 1, j))
    if i < arr.shape[0] - 1 and arr[i + 1, j] - arr[i, j] <= 1:
        G.add_edge((i, j), (i + 1, j))
    if j > 0 and arr[i, j - 1] - arr[i, j] <= 1:
        G.add_edge((i, j), (i, j - 1))
    if j < arr.shape[1] - 1 and arr[i, j + 1] - arr[i, j] <= 1:
        G.add_edge((i, j), (i, j + 1))


[G.add_node((i, j)) for i in range(arr.shape[0]) for j in range(arr.shape[1])]

for i in range(arr.shape[0]):
    for j in range(arr.shape[1]):
        check_one_less(i, j)

print("Part 1:", nx.shortest_path_length(G, tuple(start), tuple(end)))
# %%
min_dist = np.inf
for x in np.argwhere(arr == 0):
    try:
        min_dist = min(min_dist, nx.shortest_path_length(G, tuple(x), tuple(end)))
    except nx.NetworkXNoPath:
        ...
print("Part 2:", min_dist)


# %%
