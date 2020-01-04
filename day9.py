from collections import UserList

import numpy as np


class ConcatList(UserList):
    def __getitem__(self, item):
        while True:
            try:
                return self.data[item]
            except IndexError:
                self.data = self.data + [0] * (2 * len(self.data))
            else:
                break

    def __setitem__(self, key, value):
        while True:
            try:
                self.data[key] = value
            except IndexError:
                self.data = self.data + [0] * (2 * len(self.data))
            else:
                break


def processor_coroutine(code):
    """
    Yield None means need input.
    Yield number means just outputting.
    """
    code = ConcatList(code)
    i = 0
    rel_base = 0

    def get_index(mode, idx):
        if mode == '0':  # Positional
            return code[idx]
        elif mode == '1':  # Immediate
            return idx
        elif mode == '2':  # Relative
            return rel_base + code[idx]
        else:
            raise ValueError('Mode Unknown.')

    while i < len(code):
        instruction = f'{code[i]:05}'
        modes, opcode = instruction[:3], int(instruction[3:])
        p1, p2, p3 = [get_index(modes[2 - j], i + j + 1) for j in range(3)]

        if opcode in [1, 2, 7, 8]:  # Three parameters
            if opcode == 1:
                code[p3] = code[p1] + code[p2]
            elif opcode == 2:
                code[p3] = code[p1] * code[p2]
            elif opcode == 7:
                code[p3] = 1 if code[p1] < code[p2] else 0
            elif opcode == 8:
                code[p3] = 1 if code[p1] == code[p2] else 0
            i += 4

        elif opcode in [5, 6]:  # Two parameters
            if opcode == 5:
                i = code[p2] if code[p1] != 0 else i + 3
            elif opcode == 6:
                i = code[p2] if code[p1] == 0 else i + 3

        elif opcode in [3, 4, 9]:  # One parameter
            if opcode == 3:
                code[p1] = yield 'Need input!'
            elif opcode == 4:
                yield code[p1]
            elif opcode == 9:
                rel_base += code[p1]
            i += 2

        elif opcode == 99:
            return
        else:
            raise ValueError(opcode)


if __name__ == '__main__':
    code = np.loadtxt('input_day9.txt', delimiter=",", dtype=np.int64)

    # Part 1
    x = processor_coroutine(code)
    next(x)
    print(x.send(1))

    # Part 2
    x = processor_coroutine(code)
    next(x)
    print(x.send(2))

