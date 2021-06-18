use super::utils::*;
use std::convert::TryFrom;

#[derive(FromPrimitive, Debug)]
enum OpCode {
    Add = 1, // This is called a discriminant.
    Mul = 2,
    End = 99,
}

#[derive(Debug)]
pub struct IntCode {
    pub mem: Vec<isize>,
    pub ptr: usize,
    pub done: bool,
}

impl From<&[isize]> for IntCode {
    fn from(mem: &[isize]) -> Self {
        IntCode {
            mem: mem.to_vec(),
            ptr: 0,
            done: false,
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
        // let mut iter = self.mem[self.ptr..].iter().map(|x| self.mem[usize::try_from(*x).unwrap()]);
        let (op, in1, in2, out) = self.fetch_ins(self.ptr);
        let op: Option<OpCode> = num::FromPrimitive::from_isize(op);
        match op {
            Some(OpCode::Add) => self.set(self.fetch_data(in1) + self.fetch_data(in2), out),
            Some(OpCode::Mul) => self.set(self.fetch_data(in1) * self.fetch_data(in2), out),
            Some(OpCode::End) => {
                self.done = true;
            }
            None => panic!("Unknown Opcode"),
        };
    }

    fn set(&mut self, to_set: isize, addr: isize) {
        let addr = usize::try_from(addr).unwrap();
        self.mem[addr] = to_set;
        self.ptr += 4;
    }

    fn fetch_ins(&self, ptr: usize) -> (isize, isize, isize, isize) {
        // Returns 0 when out of bound.
        let mut iter = self.mem[ptr..].iter();
        (
            *iter.next().unwrap_or(&0),
            *iter.next().unwrap_or(&0),
            *iter.next().unwrap_or(&0),
            *iter.next().unwrap_or(&0),
        )
    }

    fn fetch_data(&self, addr: isize) -> isize {
        self.mem[usize::try_from(addr).unwrap()]
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
}
