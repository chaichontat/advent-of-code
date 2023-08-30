# %%
import collections
import operator as op
from itertools import islice
from pathlib import Path
from typing import Iterable, Iterator, TypeVar

import networkx as nx
from utils import fmap, sliding_window

raw = (
    Path("data/2022/" + __file__.split("/")[-1].split(".")[0] + ".txt")
    .read_text()[:-1]
    .split("\n")
)

# raw = """root: pppw + sjmn
# dbpl: 5
# cczh: sllz + lgvd
# zczc: 2
# ptdq: humn - dvpt
# dvpt: 3
# lfqf: 4
# humn: 5
# ljgn: 2
# sjmn: drzm * dbpl
# sllz: 4
# pppw: cczh / lfqf
# lgvd: ljgn * ptdq
# drzm: hmdt - zczc
# hmdt: 32""".splitlines()

T = TypeVar("T")


G = nx.DiGraph()

OPS = {"+": op.add, "-": op.sub, "*": op.mul, "/": op.floordiv}
INV = {"+": op.sub, "-": op.add, "*": op.floordiv, "/": op.mul}


for name, dep in fmap(lambda x: x.split(": "), raw):
    G.add_node(name)
    try:
        src = int(dep)
    except ValueError:
        s1, ops, s2 = dep.split(" ")
        G.add_edge(s1, name)
        G.add_edge(s2, name)
        G.nodes[name]["op"] = ops
        # G.nodes[name]["pre"] = (s1, s2)
    else:
        G.nodes[name]["n"] = src


# %%


def part1(G: nx.DiGraph):
    for node in nx.topological_sort(G):
        if "op" not in G.nodes[node]:
            continue

        s1, s2 = G.predecessors(node)
        G.nodes[node]["n"] = OPS[G.nodes[node]["op"]](
            G.nodes[s1]["n"], G.nodes[s2]["n"]
        )

    return G


print(part1(G).nodes["root"]["n"])

# %%


path = nx.shortest_path(G, "humn", "root")[:-1]

other = [x for x in G.predecessors("root") if x != path[-1]][0]
G.nodes[path[-1]]["target"] = G.nodes[other]["n"]


for src, dst in sliding_window(reversed(path), 2):
    node = G.nodes[src]
    target = node["target"]
    p_names = list(G.predecessors(src))
    p = list(map(G.nodes.__getitem__, p_names))

    # a <> b = c
    a, b, c = p[0]["n"], p[1]["n"], target

    match node["op"]:
        case "+" | "*":
            if dst == p_names[0]:  # solve for a
                new_target = INV[node["op"]](c, b)
            else:
                new_target = INV[node["op"]](c, a)
        case "-":  # a - b = c
            if dst == p_names[0]:  # solve for a
                new_target = b + c
            else:
                new_target = a - c
        case "/":  # a / b = c
            if dst == p_names[0]:  # solve for a
                new_target = b * c
            else:
                new_target = a // c
        case _:
            raise ValueError(f"Unknown op {node['op']}")

    G.nodes[dst]["target"] = new_target

print(G.nodes["humn"]["target"])

# %%
