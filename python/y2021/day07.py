#%%
import numpy as np
from pathlib import Path

raw = list(map(int, Path("../../data/2021/day07.txt").read_text()[:-1].split(",")))
# %%
np.sum(np.abs(raw - np.median(raw)))
# %%
test = np.array((-0.9, 0.001, 0.002, 0.001))

# %%
x = np.linspace(-0.1, 0.1, 100)
y = np.zeros(len(x))
z = np.zeros(len(x))
for i in range(len(x)):
    y[i] = 0.5 * np.sum((test - x[i]) ** 2 + np.abs(test - x[i]))
    z[i] = np.mean(test) + np.sum(np.sign(test - x[i])) / (2 * (len(test)))

plt.plot(x, y)
# plt.plot(x, z)
# %%
plt.plot(x, z)
# %%
from scipy.optimize import minimize

minimize(lambda x: 0.5 * np.sum((test - x) ** 2 + np.abs(test - x)), x0=-0.001, tol=1e-12)
# %%
