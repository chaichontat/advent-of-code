use std::collections::VecDeque;
use std::convert::TryFrom;

#[derive(FromPrimitive, Debug, PartialEq)]
enum OpCode {
    Add    = 1, // This is called a discriminant.
    Mul    = 2,
    In     = 3,
    Out    = 4,
    Jit    = 5, // Jump if true.
    Jif    = 6, // Jump if false.
    Lt     = 7,
    Eq     = 8,
    AdjRel = 9, // Adjust relative base.
    End    = 99,
}

#[derive(FromPrimitive, Debug, PartialEq)]
enum Mode {
    Pos = 0,
    Imm = 1,
    Rel = 2,
}

#[derive(Debug, Clone)]
pub struct IntCode {
    pub mem:    Vec<isize>,
    pub ptr:    usize,
    pub rel:    isize,
    pub done:   bool,
    pub input:  VecDeque<isize>,
    pub output: VecDeque<isize>,
}

impl From<&[isize]> for IntCode {
    fn from(mem: &[isize]) -> Self {
        IntCode {
            mem:    mem.to_vec(),
            ptr:    0,
            rel:    0,
            done:   false,
            input:  VecDeque::new(),
            output: VecDeque::new(),
        }
    }
}

impl From<&String> for IntCode {
    fn from(raw: &String) -> Self {
        let mem: Vec<isize> = raw
            .split(',')
            .map(|x| x.parse::<isize>().unwrap())
            .collect::<Vec<_>>();

        IntCode::from(&mem[..])
    }
}

impl IntCode {
    pub fn run(&mut self) {
        while !self.done {
            self.execute()
        }
    }

    pub fn run_pause(&mut self) {
        while !self.done && self.output.is_empty() {
            self.execute()
        }
    }

    pub fn run_pause_i(&mut self, i: usize) {
        for _ in 0..i {
            self.run_pause();
        }
    }

    pub fn push(&mut self, val: isize) {
        self.input.push_back(val);
    }

    pub fn pop(&mut self) -> Option<isize> {
        self.output.pop_front()
    }

    pub fn run_wait_input(&mut self) {
        while !self.done {
            let op: OpCode = num::FromPrimitive::from_isize(self.mem[self.ptr] % 100).unwrap();
            if op == OpCode::In && self.input.is_empty() {
                break;
            }
            self.execute();
        }
    }

    fn execute(&mut self) {
        let (op, p) = self.fetch_ins(self.ptr);
        let (op, modes) = self.parse_ins(op);
        if op == OpCode::End {
            self.done = true; // In case next instruction is negative.
            return;
        }

        let c0 = self.fetch_data(&modes[0], p[0]);
        let mut step = true;

        match op {
            OpCode::Add => self.set(
                c0 + self.fetch_data(&modes[1], p[1]),
                self.fetch_write(&modes[2], p[2]),
            ),
            OpCode::Mul => self.set(
                c0 * self.fetch_data(&modes[1], p[1]),
                self.fetch_write(&modes[2], p[2]),
            ),
            OpCode::In => {
                let test = self.input.pop_front().unwrap();
                self.set(test, self.fetch_write(&modes[0], p[0])) // Retain addr for writing.
            }
            OpCode::Out => self.output.push_back(c0),
            OpCode::Jit => {
                if c0 != 0 {
                    self.ptr = usize::try_from(self.fetch_data(&modes[1], p[1])).unwrap();
                    step = false;
                }
            }
            OpCode::Jif => {
                if c0 == 0 {
                    self.ptr = usize::try_from(self.fetch_data(&modes[1], p[1])).unwrap();
                    step = false;
                }
            }
            OpCode::Lt => self.op_lteq(
                OpCode::Lt,
                (c0, self.fetch_data(&modes[1], p[1])),
                self.fetch_write(&modes[2], p[2]),
            ),
            OpCode::Eq => self.op_lteq(
                OpCode::Eq,
                (c0, self.fetch_data(&modes[1], p[1])),
                self.fetch_write(&modes[2], p[2]),
            ),
            OpCode::AdjRel => self.rel += c0,
            OpCode::End => (),
        };
        if step {
            self.step(op);
        }
    }

    fn op_lteq(&mut self, op: OpCode, cmd: (isize, isize), p: usize) {
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
            OpCode::In | OpCode::Out | OpCode::AdjRel => 2,
            OpCode::End => 0,
        };
    }

    fn set(&mut self, to_set: isize, addr: usize) {
        if addr >= self.mem.len() {
            self.mem.resize(addr + 1, 0);
        }
        self.mem[addr] = to_set;
    }

    fn fetch_ins(&self, ptr: usize) -> (isize, [isize; 3]) {
        // Returns 0 when out of bound.
        let mut iter = self.mem[ptr..].iter();
        (*iter.next().unwrap_or(&0), [
            *iter.next().unwrap_or(&0),
            *iter.next().unwrap_or(&0),
            *iter.next().unwrap_or(&0),
        ])
    }

    fn fetch_data(&self, mode: &Mode, addr: isize) -> isize {
        match mode {
            Mode::Pos => *self.mem.get(usize::try_from(addr).unwrap()).unwrap_or(&0),
            Mode::Imm => addr,
            Mode::Rel => *self
                .mem
                .get(usize::try_from(addr + self.rel).unwrap())
                .unwrap_or(&0),
        }
    }

    fn fetch_write(&self, mode: &Mode, addr: isize) -> usize {
        match mode {
            Mode::Pos => usize::try_from(addr).unwrap(),
            Mode::Imm => unreachable!(),
            Mode::Rel => usize::try_from(addr + self.rel).unwrap(),
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
        assert_eq!(prep(&[1, 1, 1, 4, 99, 5, 6, 0, 99]), &[
            30, 1, 1, 4, 2, 5, 6, 0, 99
        ]);
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

    fn test_out(mem: &[isize], ins: &Vec<isize>, outs: &[isize]) {
        let mut ic = IntCode::from(mem);
        ic.input.append(&mut VecDeque::from(ins.clone()));
        ic.run();
        assert_eq!(ic.output, outs);
    }

    #[test]
    fn day9_rel_base() {
        let quine = [
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        test_out(&quine, &Vec::<isize>::new(), &quine);
        test_out(
            &[1102, 34915192, 34915192, 7, 4, 7, 99, 0],
            &Vec::<isize>::new(),
            &[1219070632396864],
        );
        test_out(&[104, 1125899906842624, 99], &Vec::<isize>::new(), &[
            1125899906842624,
        ]);
    }
}
