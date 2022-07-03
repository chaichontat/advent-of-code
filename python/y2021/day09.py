#%%
import numpy as np

from utils import read

parse = lambda x: np.fromiter(x, dtype=np.uint8)
raw = np.array(read("../../data/2021/day09.txt", parse=parse))

# %%
