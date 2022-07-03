#%%
import numpy as np

from utils import read

parse = lambda x: np.fromstring(x, dtype=np.uint16, sep=",")
ex = parse(
    """16,1,2,0,4,2,7,1,2,14
""".splitlines()[
        0
    ]
)

raw = read("../../data/2021/day07.txt", parse=parse)[0]
# raw = ex
# %%
# Minimize l1-norm.
print(np.sum(np.abs(raw - np.median(raw))))
# %%
def sum_of_nearly_squares(arr: np.ndarray, mean: int) -> int:
    return np.sum(np.abs(arr - mean) * (np.abs(arr - mean) + 1)) / 2


mean = np.floor(raw.mean())
min(sum_of_nearly_squares(raw, mean), sum_of_nearly_squares(raw, mean + 1))

# %%
