# %%
from pathlib import Path

import numpy as np
from scipy.signal import correlate2d
from utils import fmap

raw = np.array(
    fmap(
        lambda x: list(map(ord, x)),
        (
            Path("../../data/2024/" + __file__.split("/")[-1].split(".")[0] + ".txt")
            .read_text()
            .splitlines()
        ),
    )
)

xmas = np.array(list(map(ord, "XMAS")))
correct = sum(x * y for x, y in zip(xmas, xmas))
correct_rev = sum(x * y for x, y in zip(xmas, reversed(xmas)))


# %% Part 1
def is_ok(x: np.ndarray) -> bool:
    return np.sum(x == correct) + np.sum(x == correct_rev)


mode = "valid"
horz = correlate2d(raw, xmas.reshape(1, -1), mode=mode)
vert = correlate2d(raw, xmas.reshape(-1, 1), mode=mode)
diag = correlate2d(raw, np.diag(xmas), mode=mode)
revdiag = correlate2d(raw, np.diag(xmas)[::-1], mode=mode)

np.sum(list(map(is_ok, [horz, vert, diag, revdiag])))

# %% Part 2

mas = np.diag(list(map(ord, "MAS")))
mas += mas[:, ::-1]
mas[1, 1] -= mas[1, 1] // 2


correct = sum(x * y for x, y in zip(mas.flat, mas.flat))
correct_rev = sum(x * y for x, y in zip(mas.flat, mas[::-1].flat))

# %%
diag = correlate2d(raw, mas, mode=mode)
diagT = correlate2d(raw, mas.T, mode=mode)

np.sum(list(map(is_ok, [diag, diagT])))

# %%
