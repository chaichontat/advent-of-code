#%%
import networkx as nx

from utils import load

raw = load("day07.txt")
target = "shiny gold bag"

# %%
def parse(t: str):
    t = t.replace(" contain", ",").replace(".", "").replace("bags", "bag").split(",")
    head = t[0]
    tails = [" ".join(x.strip().split(" ")[1:]) for x in t[1:]]
    nums = []
    for x in t[1:]:
        if (n := x.strip().split(" ")[0]).isnumeric():
            nums.append(int(n))
        else:
            nums.append(None)
    return head, tails, nums


G = nx.DiGraph()
for st in raw:
    head, tails, nums = parse(st)
    [G.add_edge(head, tail, n=num) for tail, num in zip(tails, nums) if num is not None]

assert nx.is_directed_acyclic_graph(G)

# %% Part 1
def test1():
    assert len(nx.ancestors(G, target)) == 213


# %% Part 2
def test2():
    for node in nx.dfs_postorder_nodes(G, target):
        G.nodes[node]["n"] = sum((G.nodes[n]["n"] + 1) * v["n"] for (n, v) in G[node].items())
    assert G.nodes[target]["n"] == 38426
