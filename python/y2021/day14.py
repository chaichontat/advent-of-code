#%%
from collections import Counter, defaultdict

from utils import read

raw = read("../../data/2021/day14.txt")
ex = """NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C
""".splitlines()
# raw = ex
pol = raw[0]

keys = {}
for k in raw[2:]:
    keys[k[:2]] = (k[0] + k[6], k[6] + k[1])

# %%
counts = defaultdict(int)
for i in range(len(pol) - 1):
    counts[pol[i : i + 2]] += 1
for i in range(40):
    for k, v in counts.copy().items():
        try:
            tochange = keys[k]
        except KeyError:
            continue
        counts[k] -= v
        counts[tochange[0]] += v
        counts[tochange[1]] += v


# %%
c = Counter()
for k, v in counts.items():
    c[k[1]] += v
comm = c.most_common()
print(comm[0][1] - comm[-1][1])
# %%
