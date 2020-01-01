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

    def get_value(mode, param):  # Not for 'write' instructions.
        if mode == '0':  # Positional
            return code[param]
        elif mode == '1':  # Immediate
            return param
        elif mode == '2':  # Relative
            return code[rel_base + param]
        else:
            raise ValueError('Mode Unknown.')

    def get_value_out(mode, param):  # 'Position' IS the index value.
        if mode == '0':
            return param
        elif mode == '2':  # Relative
            return rel_base + param
        else:
            raise ValueError('Out Mode Unknown.')

    while i < len(code):
        instruction = f'{code[i]:05}'
        opcode = int(instruction[3:])

        if opcode in [1, 2, 7, 8]:  # Three parameters
            in1, in2 = [get_value(instruction[2 - j], code[i + j + 1]) for j in range(2)]
            out = get_value_out(instruction[0], code[i + 3])

            if opcode == 1:
                code[out] = in1 + in2
            elif opcode == 2:
                code[out] = in1 * in2
            elif opcode == 7:
                code[out] = 1 if in1 < in2 else 0
            elif opcode == 8:
                code[out] = 1 if in1 == in2 else 0

            i += 4

        elif opcode in [5, 6]:  # Two parameters, index
            in1, in2 = [get_value(instruction[2 - j], code[i + j + 1]) for j in range(2)]

            if opcode == 5:
                i = in2 if in1 != 0 else i + 3
            elif opcode == 6:
                i = in2 if in1 == 0 else i + 3

        elif opcode in [3, 4, 9]:  # One parameter:
            in1 = get_value(instruction[2], code[i + 1])
            if opcode == 3:
                out = get_value_out(instruction[0], code[i + 3])
                try:
                    x = yield 'Need input!'
                    code[out] = x
                except IndexError:
                    raise IndexError('Not enough input!')
            elif opcode == 4:
                yield in1
            elif opcode == 9:
                rel_base += in1
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
