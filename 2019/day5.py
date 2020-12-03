from copy import copy

import numpy as np


class Intcode:

    def __init__(self, code):
        self.code = copy(code)

    def __repr__(self):
        return f'Intcode({repr(self.code)})'

    def process_intcode(self, ins=None):
        """
        ABCDE
         1002

        DE - two-digit opcode,      02 == opcode 2
         C - mode of 1st parameter,  0 == position mode
         B - mode of 2nd parameter,  1 == immediate mode
         A - mode of 3rd parameter,  0 == position mode,
                                          omitted due to being a leading zero
        """
        i = 0
        in_idx = 0
        output = list()
        while i < len(self.code):
            instruction = f'{self.code[i]:05}'
            opcode = int(instruction[3:])

            if opcode in [1, 2, 7, 8]:  # Three parameters
                in1, in2, out = self.code[i + 1: i + 4]
                in1 = self.code[in1] if instruction[2] == '0' else in1
                in2 = self.code[in2] if instruction[1] == '0' else in2

                if opcode == 1:
                    self.code[out] = in1 + in2
                elif opcode == 2:
                    self.code[out] = in1 * in2
                elif opcode == 7:
                    self.code[out] = 1 if in1 < in2 else 0
                elif opcode == 8:
                    self.code[out] = 1 if in1 == in2 else 0

                i += 4

            elif opcode in [5, 6]:  # Two parameters, index
                in1, in2 = self.code[i + 1: i + 3]
                in1 = self.code[in1] if instruction[2] == '0' else in1
                in2 = self.code[in2] if instruction[1] == '0' else in2

                if opcode == 5:
                    i = in2 if in1 != 0 else i + 3
                elif opcode == 6:
                    i = in2 if in1 == 0 else i + 3

            elif opcode == 3:
                try:
                    self.code[self.code[i + 1]] = ins[in_idx]
                except IndexError:
                    raise IndexError('Not enough input!')
                else:
                    in_idx += 1
                    i += 2
            elif opcode == 4:
                output.append(self.code[self.code[i + 1]])
                i += 2
            elif opcode == 99:
                return output if output else self.code
            else:
                raise ValueError(opcode)


if __name__ == '__main__':
    intcode = np.loadtxt('input_day5.txt', delimiter=",", dtype=np.int)

    o = Intcode(intcode)
    print(o.process_intcode([1, ]))
