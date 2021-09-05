#%%
import numpy as np
from scipy.ndimage import convolve

from utils import NDArrayInt, load

raw = load("day17.txt", split=None)
raw = raw.strip().replace(".", "0").replace("#", "1").split("\n")

# %%
def gen_kernel(dim: int) -> NDArrayInt:
    ker = np.ones((3,) * dim, dtype=bool)
    ker[(1,) * dim] = False
    return ker


def prepare_arr(dim: int) -> NDArrayInt:
    arr = np.zeros((1,) * (dim - 2) + (len(raw[0]), len(raw)), dtype=np.int8)
    arr[(0,) * (dim - 2)] = np.array([[int(y) for y in x] for x in raw])
    return arr


def run(n: int, dim: int) -> int:
    ker = gen_kernel(dim)
    arr = prepare_arr(dim)

    for _ in range(n):
        arr = np.pad(arr, 1)  # type:ignore [no-untyped-call]
        cnts = convolve(arr, weights=ker, mode="constant")
        arr = (~arr & (cnts == 3)) | (arr & ((cnts == 2) | (cnts == 3)))

    return np.sum(arr)  # type:ignore [no-any-return]


def test1() -> None:
    assert run(6, 3) == 223


def test2() -> None:
    assert run(6, 4) == 1884


# %%
