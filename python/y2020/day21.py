#%%
import numpy as np
from scipy.sparse import csr_matrix
from scipy.sparse.csgraph import maximum_bipartite_matching

from utils import load

raw = load("day21.txt")

Plates = list[set[str]]


def parse() -> tuple[Plates, Plates]:
    plates, algs = zip(*[x[:-1].split(" (contains ") for x in raw])
    plates = [set(x.split(" ")) for x in plates]
    plates_algs = [set(x.split(", ")) for x in algs]
    return plates, plates_algs


plates, plates_algs = parse()

#%%
alg_candidates = {}
for alg in set.union(*plates_algs):  # All allergens.
    s = [ing for ing, al in zip(plates, plates_algs) if alg in al]
    alg_candidates[alg] = set.intersection(*s)

ing_al = set.union(*alg_candidates.values())  # Ingredients w/ potential allergens.


def test1() -> None:
    assert sum(len(x - ing_al) for x in plates) == 2410


# %%
def test2() -> None:
    ingr, algs = list(ing_al), list(alg_candidates)  # Keys

    adj = np.zeros((len(ingr), len(algs)), dtype=bool)
    for alg, ings in alg_candidates.items():
        for ing in ings:
            adj[ingr.index(ing), algs.index(alg)] = True

    matches = maximum_bipartite_matching(csr_matrix(adj))
    x = {ingr[m]: algs[i] for i, m in enumerate(matches)}

    assert ",".join(sorted(x, key=x.get)) == "tmp,pdpgm,cdslv,zrvtg,ttkn,mkpmkx,vxzpfp,flnhl"  # type: ignore [arg-type]
