use std::collections::VecDeque;
use std::convert::TryFrom;

#[derive(FromPrimitive, Debug, PartialEq)]
enum OpCode {
    Add = 1, // This is called a discriminant.
    Mul = 2,
    In = 3,
    Out = 4,
    Jit = 5, // Jump if true.
    Jif = 6, // Jump if false.
    Lt = 7,
    Eq = 8,
    End = 99,
}

#[derive(FromPrimitive, Debug)]
enum Mode {
    Pos = 0,
    Imm = 1,
}

#[derive(Debug)]
pub struct IntCode {
    pub mem: Vec<isize>,
    pub ptr: usize,
    pub done: bool,
    pub input: VecDeque<isize>,
    pub output: VecDeque<isize>,
}

impl From<&[isize]> for IntCode {
    fn from(mem: &[isize]) -> Self {
        IntCode {
            mem: mem.to_vec(),
            ptr: 0,
            done: false,
            input: VecDeque::new(),
            output: VecDeque::new(),
        }
    }
}

impl IntCode {
    pub fn run(&mut self) -> &Vec<isize> {
        while !self.done {
            self.execute()
        }
        &self.mem
    }

    fn execute(&mut self) {
        let (op, p) = self.fetch_ins(self.ptr);
        let (op, modes) = self.parse_ins(op);
        let cmd = (
            self.fetch_data(&modes[0], p.0),
            self.fetch_data(&modes[1], p.1),
        );
        let mut step = true;

        match op {
            OpCode::Add => self.set(cmd.0 + cmd.1, p.2),
            OpCode::Mul => self.set(cmd.0 * cmd.1, p.2),
            OpCode::In => {
                let test = self.input.pop_front().unwrap();
                self.set(test, p.0) // Retain addr for writing.
            }
            OpCode::Out => self.output.push_back(cmd.0),
            OpCode::End => {
                self.done = true;
                return;
            }
            OpCode::Jit => {
                if cmd.0 != 0 {
                    self.ptr = usize::try_from(cmd.1).unwrap();
                    step = false;
                }
            }
            OpCode::Jif => {
                if cmd.0 == 0 {
                    self.ptr = usize::try_from(cmd.1).unwrap();
                    step = false;
                }
            }
            OpCode::Lt => self.op_lteq(OpCode::Lt, cmd, p.2),
            OpCode::Eq => self.op_lteq(OpCode::Eq, cmd, p.2),
        };
        if step {
            self.step(op);
        }
    }

    fn op_lteq(&mut self, op: OpCode, cmd: (isize, isize), p: isize) {
        if (cmd.0 < cmd.1 && op == OpCode::Lt) || (cmd.0 == cmd.1 && op == OpCode::Eq) {
            self.set(1, p)
        } else {
            self.set(0, p)
        }
    }

    fn step(&mut self, op: OpCode) {
        self.ptr += match op {
            OpCode::Add | OpCode::Mul | OpCode::Lt | OpCode::Eq => 4,
            OpCode::Jit | OpCode::Jif => 3,
            OpCode::In | OpCode::Out => 2,
            OpCode::End => 0,
        };
    }

    fn set(&mut self, to_set: isize, addr: isize) {
        let addr = usize::try_from(addr).unwrap();
        if addr >= self.mem.len() {
            self.mem.resize(addr + 1, 0);
        }
        self.mem[addr] = to_set;
    }

    fn fetch_ins(&self, ptr: usize) -> (isize, (isize, isize, isize)) {
        // Returns 0 when out of bound.
        let mut iter = self.mem[ptr..].iter();
        (
            *iter.next().unwrap_or(&0),
            (
                *iter.next().unwrap_or(&0),
                *iter.next().unwrap_or(&0),
                *iter.next().unwrap_or(&0),
            ),
        )
    }

    fn fetch_data(&self, mode: &Mode, addr: isize) -> isize {
        match mode {
            Mode::Pos => *self.mem.get(usize::try_from(addr).unwrap()).unwrap_or(&0),
            Mode::Imm => addr,
        }
    }

    fn parse_ins(&self, mut ins: isize) -> (OpCode, [Mode; 3]) {
        let mut modes = [Mode::Pos, Mode::Pos, Mode::Pos];
        let op: OpCode = num::FromPrimitive::from_isize(ins % 100).unwrap();
        ins /= 100;
        for mode in &mut modes {
            *mode = num::FromPrimitive::from_isize(ins % 10).unwrap();
            ins /= 10;
        }
        (op, modes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn prep(mem: &[isize]) -> Vec<isize> {
        let mut ic = IntCode::from(mem);
        ic.run();
        ic.mem
    }

    #[test]
    fn day2_addmul() {
        assert_eq!(prep(&[1, 0, 0, 0, 99]), &[2, 0, 0, 0, 99]);
        assert_eq!(prep(&[2, 3, 0, 3, 99]), &[2, 3, 0, 6, 99]);
        assert_eq!(prep(&[2, 4, 4, 5, 99, 0]), &[2, 4, 4, 5, 99, 9801]);
        assert_eq!(
            prep(&[1, 1, 1, 4, 99, 5, 6, 0, 99]),
            &[30, 1, 1, 4, 2, 5, 6, 0, 99]
        );
    }

    fn test_inout(mem: &[isize], codes: &[isize], expects: &[isize]) {
        for (c, o) in codes.iter().zip(expects) {
            let mut ic = IntCode::from(mem);
            ic.input.push_front(*c);
            ic.run();
            assert_eq!(ic.output.pop_back().unwrap(), *o);
        }
    }

    #[test]
    fn day5_inout_pmode() {
        test_inout(&[3, 0, 4, 0, 99], &[5], &[5]);

        test_inout(&[3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], &[8, 7], &[1, 0]); // Position mode Eq.
        test_inout(&[3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], &[7, 8], &[1, 0]); // Position mode Lt.
        test_inout(&[3, 3, 1108, -1, 8, 3, 4, 3, 99], &[8, 7], &[1, 0]); // Immediate mode Eq.
        test_inout(&[3, 3, 1107, -1, 8, 3, 4, 3, 99], &[7, 8], &[1, 0]); // Immediate mode Lt.

        let jump_pos = &[3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        test_inout(jump_pos, &[0, 1], &[0, 1]);
        let jump_imm = &[3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
        test_inout(jump_imm, &[0, 1], &[0, 1]);
        let larger = &[
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];
        test_inout(larger, &[7, 8, 9], &[999, 1000, 1001]);
    }
}
