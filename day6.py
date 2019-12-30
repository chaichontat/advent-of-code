from pathlib import Path

import networkx as nx

g = nx.Graph()

if __name__ == '__main__':
    data = Path('input_day6.txt').read_text()

    # Add nodes and edges.
    nodes = set(data.replace(')', '\n').split())
    g.add_nodes_from(nodes)
    for edge in data.split():
        g.add_edge(*edge.split(')'))

    dist: dict = nx.shortest_path_length(g, 'COM')

    # Part 1
    print(sum(dist.values()))

    # Part 2
    print(nx.shortest_path_length(g, source='YOU', target='SAN') - 2)
