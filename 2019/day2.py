import numpy as np


def process_intcode(intcode: np.ndarray):
    assert intcode[0] in [1, 2, 99]
    intcode = intcode.copy()

    for i in range(0, len(intcode), 4):
        if intcode[i] in [1, 2]:
            process_opcode(intcode, i)
        elif intcode[i] == 99:
            return intcode
        else:
            raise Exception


def process_opcode(intcode, idx):
    opcode, loc1, loc2, loc_out = intcode[idx: idx + 4]
    ops = np.add if opcode == 1 else np.multiply
    intcode[loc_out] = ops(intcode[loc1], intcode[loc2])


if __name__ == '__main__':
    intcode = np.loadtxt('input_day2.txt', delimiter=",", dtype=np.int)
    intcode[1], intcode[2] = 12, 2

    # Part 1
    print(process_intcode(intcode)[0])

    # Part 2
    for i in range(100):
        for j in range(100):
            intcode[1], intcode[2] = i, j
            if process_intcode(intcode)[0] == 19690720:
                print(100 * i + j)
                break
