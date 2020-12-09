#%%
from utils import load

raw = load("day9.txt", parseint=True)[:-1]

# %% Part 1
def test(idx):
    for i in range(idx):
        for j in range(i, idx):
            if raw[idx] == raw[i] + raw[j]:
                return True
    return False


result = -1
for idx in range(25, len(raw)):
    if not test(idx):
        result = idx
        break
print(raw[result])
# %%
# Since sorted.
result2 = -1
for i in range(result):
    for j in range(i, result):
        if sum(raw[i:j]) == raw[result]:
            result2 = min(raw[i:j]) + max(raw[i:j])
            break
    else:  # Break outer loop.
        continue
    break

print(result2)
# %%
