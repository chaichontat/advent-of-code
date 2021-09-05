#%%
from utils import load

raw = load("day09.txt", parseint=True)

# %%
def run_test(idx: int) -> bool:
    for i in range(idx - 25, idx):
        for j in range(i + 1, idx):
            if raw[idx] == raw[i] + raw[j]:
                return True
    return False


target = -1
for idx in range(25, len(raw)):
    if not run_test(idx):
        target = idx
        break


def test1() -> None:
    assert raw[target] == 1038347917


# %%
def test2() -> None:
    cumsum = dict()  # Sum: index.
    curr_sum = 0

    for i in range(len(raw)):
        cumsum[curr_sum] = i
        curr_sum += raw[i]
        if (start_sum := curr_sum - raw[target]) in cumsum:  # DP.
            vals = raw[cumsum[start_sum] : i + 1]
            assert min(vals) + max(vals) == 137394018
            break


# %%
