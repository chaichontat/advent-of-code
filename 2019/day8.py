#%%
from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np

data = Path("input_day8.txt").read_text()[:-1]
data = np.array(list(data), dtype=int).reshape((-1, 6, 25))

# %% Part 1
clipped = np.clip(data, 0, 1)
idx_max = np.argmax(np.sum(clipped, axis=(1, 2)))
print(np.sum(data[idx_max] == 1) * np.sum(data[idx_max] == 2))

# %% Part 2 Highest non-two
flattened = np.zeros((6, 25))
for i in range(6):
    for j in range(25):
        flattened[i, j] = data[np.argmax(data[:, i, j] != 2), i, j]

plt.imshow(flattened)
plt.show()

# %%
