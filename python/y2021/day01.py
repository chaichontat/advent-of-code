#%%
import numpy as np

from utils import read

arr = read("../../data/2021/day01.txt", parse=True)


def count_increase(arr: np.ndarray) -> int:
    return np.sum(arr[1:] - arr[:-1] > 0)


t = np.convolve(arr, np.ones(3, dtype=int), "valid")
print(count_increase(arr), count_increase(t))

# %%
