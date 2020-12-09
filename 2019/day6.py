#%%
import networkx as nx
from utils import load

data = load("input_day6.txt")

G = nx.Graph()
for edge in data:
    G.add_edge(*edge.split(")"))

#%% Part 1
dist: dict = nx.shortest_path_length(G, "COM")
print(sum(dist.values()))

#%% Part 2
print(nx.shortest_path_length(G, source="YOU", target="SAN") - 2)
