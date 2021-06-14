#%%
import matplotlib.pyplot as plt
import numpy as np

from intcode import IntCode
from utils import load

raw = load("day13.txt", split=",", parseint=True)
# %%
ic = IntCode(raw)
ic.execute()
out = np.array(ic.outputs).reshape((-1, 3))
print(np.sum(out[:, 2] == 2))

#%% Part 2
class Arcade(IntCode):
    def __init__(self, *args, **kwargs) -> None:
        super().__init__(*args, **kwargs)
        self.img = None
        self.score = 0
        self.ball = None
        self.pad = None

    def run_game(self, input_=None):
        if input_ is not None:
            self.inputs.append(input_)
        out = self.execute()
        self.process()
        return out

    def process(self):
        out = np.array(self.outputs).reshape((-1, 3))
        self.outputs.clear()
        board = out.copy()
        if self.img is None:
            size = np.max(board, axis=0)[:2] + 1
            self.img = np.zeros(size)
        for i in range(board.shape[0]):
            x, y, v = board[i]
            if x == -1 and y == 0:
                self.score = v
            else:
                if v == 4:
                    self.ball = (x, y)
                if v == 3:
                    self.pad = (x, y)
                self.img[x, y] = v

        return self.img


def show(ic):
    s = ic.img.copy()
    s[s == 3] = 1
    plt.imshow(s, cmap="gray", origin="lower")


ic = Arcade(raw)
ic.ins[0] = 2
ic.run_game()
show(ic)

# %% Part 2
while not np.all(ic.img != 2):
    diff = ic.ball[0] - ic.pad[0]
    if diff != 0:
        diff /= abs(diff)
    ic.run_game(diff)
print(ic.score)

# %%
