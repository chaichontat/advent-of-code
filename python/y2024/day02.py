# %%
from pathlib import Path

import numpy as np
from utils import fmap

raw = fmap(
    lambda x: np.fromstring(x, sep=" ", dtype=int),
    Path("../../data/2024/" + __file__.split("/")[-1].split(".")[0] + ".txt")
    .read_text()
    .splitlines(),
)


# %%
def is_safe(x: np.ndarray, thresh: int = 0) -> bool:
    diffs = np.diff(x)
    if not (np.signbit(diffs).all() or (~np.signbit(diffs)).all()):
        return False
    return np.all((1 <= np.abs(diffs)) & (np.abs(diffs) <= 3))


np.sum(fmap(is_safe, raw))

# %%
mine = []
cnt = 0
for x in raw:
    if is_safe(x):
        mine.append(tuple(x))
        cnt += 1
        continue
    for i in range(len(x)):
        if is_safe(np.delete(x, i)):
            mine.append(tuple(x))
            cnt += 1
            break
print(cnt)

# %%
