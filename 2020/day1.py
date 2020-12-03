# %%
from utils import load

data = load("day1.txt", parseint=True)

# %% Part 1


def run():
    for i in range(len(data)):
        for j in range(i, len(data)):
            if data[i] + data[j] == 2020:
                return data[i] * data[j]

    raise ValueError("Failed.")


print(run())
# %% Part 2


def run():
    for i in range(len(data)):
        for j in range(i, len(data)):
            for k in range(j, len(data)):
                if data[i] + data[j] + data[k] == 2020:
                    return data[i] * data[j] * data[k]

    raise ValueError("Failed.")


print(run())
# %%
