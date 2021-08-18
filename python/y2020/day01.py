# %%
from utils import load

data = load("day01.txt", parseint=True)

# %%
def test1():
    for i in range(len(data)):
        for j in range(i, len(data)):
            if data[i] + data[j] == 2020:
                assert data[i] * data[j] == 605364
                return

    raise ValueError("Failed.")


# %%
def test2():
    for i in range(len(data)):
        for j in range(i, len(data)):
            for k in range(j, len(data)):
                if data[i] + data[j] + data[k] == 2020:
                    assert data[i] * data[j] * data[k] == 128397680
                    return

    raise ValueError("Failed.")
