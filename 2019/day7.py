from itertools import permutations

import numpy as np

from day5 import Intcode


class ACS(Intcode):
    def __init__(self, code, n):
        super().__init__(code)
        self.acs = code.copy()
        self.n = n

    def get_thruster_output(self, phase):
        assert len(phase) == self.n
        x = 0
        for p in phase:
            self.code = self.acs.copy()
            x = self.process_intcode([p, x])[0]
        return x


if __name__ == '__main__':
    acs_code = np.loadtxt('input_day7.txt', delimiter=",", dtype=np.int)
    acs = ACS(acs_code, 5)

    output_vals = [acs.get_thruster_output(p) for p in permutations(range(acs.n))]
    print(max(output_vals))
