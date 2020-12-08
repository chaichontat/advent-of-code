from math import prod
from operator import eq, lt, add, mul
from typing import Iterable, Optional


class IntCode:
    n_params = {
        1: 3,  # Add
        2: 3,  # Multiply
        3: 1,  # Write
        4: 1,  # Read
        5: 2,  # Jump-if-true
        6: 2,  # Jump-if-false
        7: 3,  # Less than
        8: 3,  # Equals
        99: 1,
    }

    def __init__(self, instructions: list[int]) -> None:
        self.ins = instructions.copy()
        self.inputs: Optional[Iterable] = None
        self.outputs = list()
        self.curr_loc = 0

    def __repr__(self):
        return f"Intcode({repr(self.ins)})"

    def execute(self, inputs: Optional[list] = None):
        if inputs is not None:
            self.inputs = iter(inputs)

        while True:
            cmd, *params = self.parse_op(*self.get_params())
            if cmd == 99:
                return
            elif cmd in [1, 2, 3, 4, 7, 8]:
                self.run_opcode(cmd, *params)
            elif cmd in [5, 6]:
                self.run_jumper(cmd, *params)

    def run_opcode(self, cmd, *params):
        if cmd in [1, 2, 7, 8]:
            f = {1: add, 2: mul, 7: lt, 8: eq}
            self.ins[params[2]] = f[cmd](*params[:2])
        elif cmd == 3:
            self.ins[params[0]] = next(self.inputs)
        elif cmd == 4:
            self.outputs.append(params[0])
        else:
            raise ValueError
        self.curr_loc += self.n_params[cmd] + 1

    def run_jumper(self, cmd, *params):
        do_if_zero = True if cmd == 6 else False
        if (params[0] == 0) == do_if_zero:
            self.curr_loc = params[1]
        else:
            self.curr_loc += 3

    def get_params(self):
        op = f"{self.ins[self.curr_loc]:04}"
        params = self.ins[self.curr_loc + 1 : self.curr_loc + 1 + self.n_params[int(op[-2:])]]
        return op, *params

    def parse_op(self, op, *params):
        """
        The opcode is a two-digit number based only on the ones and tens digit of the value,
        that is, the opcode is the rightmost two digits of the first value in an instruction.
        Parameter modes are single digits, one per parameter, read right-to-left from the opcode.
        """

        out = list()
        cmd = int(op[-2:])
        if cmd == 3:
            return cmd, *params
        if cmd == 99:
            return 99, *params

        for mode, param in zip(reversed(op[:-2]), params[:2]):
            if mode == "0":
                proc = self.ins[param]
            elif mode == "1":
                proc = param
            else:
                raise ValueError
            out.append(proc)

        if len(params) == 3:
            out.append(params[2])  # Write.
        return cmd, *out