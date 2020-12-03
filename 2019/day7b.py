from itertools import permutations

import numpy as np


def processor_coroutine(code):
    """
    Yield None means need input.
    Yield number means just outputting.
    """

    i = 0
    while i < len(code):
        instruction = f'{code[i]:05}'
        opcode = int(instruction[3:])

        if opcode in [1, 2, 7, 8]:  # Three parameters
            in1, in2, out = code[i + 1: i + 4]
            in1 = code[in1] if instruction[2] == '0' else in1
            in2 = code[in2] if instruction[1] == '0' else in2

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
            in1, in2 = code[i + 1: i + 3]
            in1 = code[in1] if instruction[2] == '0' else in1
            in2 = code[in2] if instruction[1] == '0' else in2

            if opcode == 5:
                i = in2 if in1 != 0 else i + 3
            elif opcode == 6:
                i = in2 if in1 == 0 else i + 3

        elif opcode == 3:
            try:
                x = yield 'Need Input!'
                code[code[i + 1]] = x
            except IndexError:
                raise IndexError('Not enough input!')
            else:
                i += 2
        elif opcode == 4:
            yield code[code[i + 1]]
            i += 2
        elif opcode == 99:
            return
        else:
            raise ValueError(opcode)


def get_thruster_output(code, phase):
    coroutines = [processor_coroutine(code.copy()) for _ in range(5)]
    for c, i in zip(coroutines, phase):  # Set phase
        c.send(None)
        c.send(i)

    x = coroutines[0].send(0)  # First
    next(coroutines[0])
    i = 1

    while True:
        coroutine = coroutines[i % 5]
        try:
            x = coroutine.send(x)
            next(coroutine)
        except StopIteration:
            if i % 5 == 4:
                return x
        i += 1


if __name__ == '__main__':
    acs_code = np.loadtxt('input_day7.txt', delimiter=",", dtype=np.int)

    output_vals = [get_thruster_output(acs_code, p) for p in permutations(range(5, 10))]
    print(max(output_vals))
