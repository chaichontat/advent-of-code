#%%
import numpy as np

from utils import load

raw = load("day16.txt")


def parse(s: str):
    return np.array(list(s), dtype=int)


parsed = parse(raw[0])
# %%
def gen_mat(size: int, base=None):
    mat = np.zeros((size, size), dtype=int)
    if base is None:
        base = np.array([0, 1, 0, -1])
    for i in range(size):
        rep = np.repeat(base, i + 1)
        mat[i] = np.tile(rep, (size // rep.size) + 1)[1 : size + 1]
    return mat


def fft(arr: np.ndarray):
    mat = gen_mat(arr.size)
    for _ in range(100):
        arr = np.abs(mat @ arr) % 10
    return arr


def test1():
    assert "".join(map(str, fft(parsed)[:8])) == "27229269"


# %%
def find_hidden(s: np.ndarray):
    s = s.copy()
    for _ in range(100):
        cumsum = np.sum(s)
        for i in range(s.size):
            val = s[i]
            s[i] = cumsum % 10
            cumsum -= val
    return s


# This method is only valid for determining numbers at indices greater than half.
thr = int(raw[0][:7])
seq = (raw[0] * 10000)[thr:]
assert thr > len(raw[0]) * 10000 / 2


def test2():
    res = find_hidden(parse(seq))
    assert "".join(map(str, res[:8])) == "26857164"


# %%
