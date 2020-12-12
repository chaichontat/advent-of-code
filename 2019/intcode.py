from collections import UserList, deque
from operator import add, eq, lt, mul
from typing import Optional


class LengtheningList(UserList):
    """Double size when accessing indices beyond size."""

    def lengthen(self):
        self.data = self.data + [0] * len(self.data)

    def __getitem__(self, item):
        while True:
            try:
                return self.data[item]
            except IndexError:
                self.lengthen()

    def __setitem__(self, key, value):
        while True:
            try:
                self.data[key] = value
                return
            except IndexError:
                self.lengthen()


class IntCode:
    n_params = {
        1: 3,  # Add
        2: 3,  # Multiply
        3: 1,  # Write
        4: 1,  # Read
        5: 2,  # Jump-if-true
        6: 2,  # Jump-if-false
        7: 3,  # Less than
        8: 3,  # Equals,
        9: 1,  # Relative jump
        99: 1,
    }

    def __init__(
        self, instructions: list[int], inputs: Optional[list] = None, pause_on_io=False
    ) -> None:
        self.ins = LengtheningList(instructions.copy())
        self.inputs = deque(inputs) if inputs is not None else deque()
        self.outputs = deque()
        self.curr_loc = 0
        self.pause_on_io = pause_on_io
        self.rel_loc = 0

    def __repr__(self):
        return f"Intcode({repr(self.ins)})"

    def execute(self) -> int:
        while True:
            cmd, *params = self.parse_op(*self.get_params())

            if cmd == 99:
                return 0
            elif cmd in [5, 6]:
                self.run_jumper(cmd, *params)
            elif cmd == 3:
                if len(self.inputs) == 0:
                    return 1
                self.run_opcode(cmd, *params)
            elif cmd in [1, 2, 4, 7, 8, 9]:
                self.run_opcode(cmd, *params)

            if self.pause_on_io and cmd == 4:
                return 2

    def run_opcode(self, cmd, *params):
        if cmd in [1, 2, 7, 8]:
            f = {1: add, 2: mul, 7: lt, 8: eq}
            self.ins[params[2]] = f[cmd](*params[:2])
        elif cmd == 3:
            self.ins[params[0]] = int(self.inputs.popleft())
        elif cmd == 4:
            self.outputs.append(int(params[0]))
        elif cmd == 9:
            self.rel_loc += params[0]
        else:
            raise ValueError
        self.curr_loc += self.n_params[cmd] + 1

    def run_jumper(self, cmd, *params):
        if cmd in [5, 6]:
            do_if_zero = True if cmd == 6 else False
            if (params[0] == 0) == do_if_zero:
                self.curr_loc = params[1]
            else:
                self.curr_loc += 3
        else:
            raise ValueError

    def get_params(self):
        op = f"{self.ins[self.curr_loc]:05}"
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
        if cmd == 99:
            return 99, *params

        if cmd != 3:
            for mode, param in zip(reversed(op[:-2]), params[:2]):
                if mode == "0":
                    proc = self.ins[param]
                elif mode == "1":
                    proc = param
                elif mode == "2":
                    proc = self.ins[self.rel_loc + param]
                else:
                    raise ValueError
                out.append(proc)

            if len(params) == 3:  # Write address.
                write_addr = params[2] + self.rel_loc if op[0] == "2" else params[2]
                out.append(write_addr)  # Write.

        else:  # 3 is the only opcode that has write addr as the first param.
            write_addr = params[0] + self.rel_loc if op[2] == "2" else params[0]
            out.append(write_addr)

        return cmd, *out
