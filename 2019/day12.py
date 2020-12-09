#%%
import numpy as np
from itertools import combinations
from utils import load


raw = load("input_day12.txt")
poss = np.array([[int(y[2:]) for y in x[1:-1].split(", ")] for x in raw])


class Simulator:
    def __init__(self, poss, vels=None) -> None:
        self.poss = poss.copy()
        self.n = self.poss.shape[0]
        if vels is None:
            self.vels = np.zeros_like(poss, dtype=self.poss.dtype)
        assert self.poss.shape == self.vels.shape

    def simulate(self, j):
        for pair in combinations(range(self.n), r=2):
            if self.poss[pair[0], j] > self.poss[pair[1], j]:
                self.vels[pair[0], j] -= 1
                self.vels[pair[1], j] += 1
            elif self.poss[pair[0], j] < self.poss[pair[1], j]:
                self.vels[pair[0], j] += 1
                self.vels[pair[1], j] -= 1
        self.poss[:, j] += self.vels[:, j]

    def simulate_all(self):
        for i in range(self.poss.shape[1]):
            self.simulate(i)

    @property
    def energy(self):
        return np.sum(np.abs(self.poss).sum(axis=1) * np.abs(self.vels).sum(axis=1))


#%% Part 1
s = Simulator(poss)
for i in range(1000):
    s.simulate_all()
print(s.energy)


# %% Part 2
out = []
for ax in range(3):
    s = Simulator(poss)
    s.simulate(ax)
    i = 2
    while not np.allclose(s.poss, poss):
        s.simulate(ax)
        i += 1
    out.append(i)

print(np.lcm.reduce(out))
# %%
