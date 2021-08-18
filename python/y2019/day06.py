#%%
import networkx as nx

from utils import load

data = load("day06.txt")


def build():
    G = nx.Graph()
    for edge in data:
        G.add_edge(*edge.split(")"))
    return G


#%% Part 1
def test06a(benchmark):
    assert benchmark(lambda: sum(nx.shortest_path_length(build(), "COM").values())) == 402879


#%% Part 2
def test06b(benchmark):
    assert benchmark(lambda: nx.shortest_path_length(build(), source="YOU", target="SAN") - 2) == 484


# %%
