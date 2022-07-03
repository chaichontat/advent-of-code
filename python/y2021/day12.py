#%%
import numpy as np

from utils import read
import networkx as nx

parse = lambda x: x.split("-")
raw = read("../../data/2021/day12.txt", parse=parse)

# %%
G = nx.Graph()
G.add_edges_from(raw)
# %%
len(list(nx.algorithms.all_simple_paths(G, "start", "end")))
# %%
