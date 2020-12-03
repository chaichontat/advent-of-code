import matplotlib.pyplot as plt
import numpy as np

from day9 import processor_coroutine


class RingBuffer:
    def __init__(self, objs):
        self.curr_idx = 0
        self.objs = objs

    def __str__(self):
        return self.objs, self.curr_idx

    def curr_dir(self):
        return self.objs[self.curr_idx]

    def left(self):
        self.curr_idx -= 1
        if self.curr_idx < 0:
            self.curr_idx += len(self.objs)
        return self.objs[self.curr_idx]

    def right(self):
        self.curr_idx += 1
        if self.curr_idx >= len(self.objs):
            self.curr_idx -= len(self.objs)
        return self.objs[self.curr_idx]


class CountTile:
    def __init__(self, code):
        self.prog = processor_coroutine(code)
        next(self.prog)

        self.tiles = dict()
        self.curr_loc = np.array((0, 0))
        self.ring = RingBuffer([(0, 1), (1, 0), (0, -1), (-1, 0)])

    def step(self):
        # Paint
        to_send = self.tiles.setdefault(tuple(self.curr_loc), 0)
        paint = self.prog.send(to_send)
        self.tiles[tuple(self.curr_loc)] = paint

        # Turn and Step
        turn = self.prog.send(None)
        if turn == 0:
            self.curr_loc += self.ring.left()
        elif turn == 1:
            self.curr_loc += self.ring.right()
        else:
            raise ValueError

        self.prog.send(None)

    def step_till_done(self):
        while True:
            try:
                self.step()
            except StopIteration:
                return self.tiles


def show_paint(tiles: dict):
    xs, ys = set(), set()
    for x, y in tiles.keys():
        xs.add(x)
        ys.add(y)
    x_min, y_min = min(xs), min(ys)
    x_range = max(xs) - x_min
    y_range = max(ys) - y_min

    img = np.zeros((y_range + 1, x_range + 1), dtype=np.bool)
    for (x, y), color in tiles.items():
        img[-(y - y_min) + y_range, x - x_min] = color
    plt.imshow(img)
    plt.show()


if __name__ == '__main__':
    code = np.loadtxt('input_day11.txt', delimiter=",", dtype=np.int64)

    # Part 1
    count = CountTile(code)
    print(len(count.step_till_done()))

    # Part 2
    count = CountTile(code)
    count.tiles[(0, 0)] = 1
    img = count.step_till_done()
    show_paint(img)
