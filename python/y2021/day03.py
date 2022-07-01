#%%
import numpy as np

from utils import read

arr = read("../../data/2021/day03.txt")
ex = """00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
""".splitlines()

# arr = ex
# %%

vals = np.array([int(x, 2) for x in arr], dtype=np.uint16)
n = len(arr[0])


def most_common(x: np.ndarray, switch: bool = False):
    counts = np.zeros(n, dtype=np.uint16)
    for i in range(n):
        counts[i] = np.sum((x & (1 << i)).astype(bool))
    # Python bool must use not, otherwise turns into -1.
    return (counts >= len(x) - counts) ^ (not switch)


# %%
count, inv = 0, 0
for i, x in enumerate(most_common(vals)):
    count += x << i  # Ignore 0.
    inv += ~x << i
print(count * inv)


#%%
def find(switch: bool) -> int:
    out = vals
    for i in reversed(range(n)):
        mc = most_common(out, switch)
        idx = (out & (1 << i)).astype(bool) ^ (~mc[i])  # Invert if not x.
        out = out[idx]
        if len(out) == 1:
            return int(out[0])

    raise ValueError("No solution found.")


print(find(False) * find(True))

# %%
