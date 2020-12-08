#%%
from utils import load

raw = load("input_day3.txt")

#%% Part 1
def process(seq: str):
    dir_dict = {
        "R": lambda t, x: (t[0] + x, t[1]),
        "L": lambda t, x: (t[0] - x, t[1]),
        "U": lambda t, x: (t[0], t[1] + x),
        "D": lambda t, x: (t[0], t[1] - x),
    }
    out = {0: (0, 0)}
    i = 0
    seq = seq.split(",")
    for step in seq:
        direction = step[0]
        amount = int(step[1:])
        for _ in range(amount):
            out[i + 1] = dir_dict[direction](out[i], 1)
            i += 1
    return out


grids = [set(process(seq).values()) for seq in raw]
intersections = set.intersection(*grids)
print(min((abs(loc[0]) + abs(loc[1]) for loc in intersections if loc != (0, 0))))

# %% Part 2
grids = [process(seq) for seq in raw]
dists = {loc: [0, 0] for loc in intersections}

for i in range(2):
    for step, loc in grids[i].items():
        if loc in intersections:
            dists[loc][i] = step

print(min([sum(dist) for loc, dist in dists.items() if loc != (0, 0)]))
# %%
