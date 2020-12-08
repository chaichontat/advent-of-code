#%%
import networkx as nx
from utils import load

raw = load("day7.txt")

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


# %%
import networkx as nx

G = nx.DiGraph()
for st in raw:
    head, tails, nums = parse(st)
    [G.add_edge(head, tail, n=num) for tail, num in zip(tails, nums) if num is not None]

assert nx.is_directed_acyclic_graph(G)
# %% Part 1
target = "shiny gold bag"
len(nx.ancestors(G, target))

# %% Part 2

for node in nx.dfs_postorder_nodes(G, target):
    G.nodes[node]["n"] = sum((G.nodes[n]["n"] + 1) * v["n"] for (n, v) in G[node].items())

print(G.nodes[target]["n"])
# %%
