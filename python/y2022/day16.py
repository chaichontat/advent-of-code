# %%
import re
from pathlib import Path

import networkx as nx
from utils import fmap

try:
    profile
except NameError:
    profile = lambda x: x

raw = (
    Path("data/2022/" + __file__.split("/")[-1].split(".")[0] + ".txt")
    .read_text()[:-1]
    .splitlines()
)

# raw = """Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
# Valve BB has flow rate=13; tunnels lead to valves CC, AA
# Valve CC has flow rate=2; tunnels lead to valves DD, BB
# Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
# Valve EE has flow rate=3; tunnels lead to valves FF, DD
# Valve FF has flow rate=0; tunnels lead to valves EE, GG
# Valve GG has flow rate=0; tunnels lead to valves FF, HH
# Valve HH has flow rate=22; tunnel leads to valve GG
# Valve II has flow rate=0; tunnels lead to valves AA, JJ
# Valve JJ has flow rate=21; tunnel leads to valve II""".splitlines()

valves = [re.findall(r"([A-Z][A-Z])", x) for x in raw]
flowrate = {v[0]: int(re.findall(r"(\d+)", x)[0]) for v, x in zip(valves, raw)}
nonzeros = {k for k, f in flowrate.items() if f > 0}
n_total = len(nonzeros)

G = nx.Graph()
G.add_nodes_from(fmap(lambda x: x[0], valves))
for valve, tunnels in zip(valves, fmap(lambda x: x[1:], valves)):
    [G.add_edge(valve[0], tunnel) for tunnel in tunnels]


# %%
@profile
def part1(paths: dict[str, dict[str, int]], deadline: int):
    total_paths: list[tuple[int, frozenset[str], int]] = []

    @profile
    def solver(curr_path: int, released: frozenset[str], costs: int, src: str):
        if (t := curr_path + len(released)) > deadline or len(released) == n_total:
            total_paths.append((curr_path, released, costs))
            return

        if not [
            solver(
                curr_path + paths[src][dst],
                frozenset(released | {dst}),
                costs + ttd * flowrate[dst],
                dst,
            )
            for dst in nonzeros - released
            if (ttd := deadline - (t + paths[src][dst])) >= 0
        ]:
            total_paths.append((curr_path, released, costs))

    solver(1, frozenset(), 0, "AA")
    return max(total_paths, key=lambda x: x[2]), total_paths


paths = {k: v for k, v in nx.all_pairs_shortest_path_length(G, cutoff=30)}
p1 = part1(paths, 30)
print("Part 1:", p1[0][2])


# %%
@profile
def part2(paths: dict[str, dict[str, int]], deadline: int):
    _, tps = part1(paths, deadline)

    # Remove permutations, keep the order that has the highest cost
    ok = {}
    for _, r, c in tps:
        for r1, c1 in ok.items():
            if r == r1:
                if c > c1:
                    ok[r] = c
                break
        else:
            ok[r] = c

    # To allow early termination. At an index, the earliest disjoint set has the highest cost.
    sok = sorted(ok, key=lambda x: -ok.get(x, 0))

    currmax = 0
    for i, r1 in enumerate(sok):
        for r2 in sok[i + 1 :]:
            if r1.isdisjoint(r2):
                currmax = max(currmax, ok[r1] + ok[r2])
                break
    return currmax


print("Part 2:", part2(paths, 26))

# %%
