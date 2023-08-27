use num::{PrimInt, Signed};
use num_complex::Complex;
use strum_macros::EnumString;

pub type Coord = Complex<isize>;

#[derive(Debug, PartialEq, Eq, EnumString, Clone, Copy)]
pub enum Dir {
    U,
    D,
    L,
    R,
}

impl Dir {
    pub fn turn(&self, turn: Turn) -> Self {
        Dir::from(Complex::<i8>::from(*self) * Complex::from(turn))
    }
}

impl<T: Signed> From<Complex<T>> for Dir {
    fn from(x: Complex<T>) -> Self {
        if x == Complex::new(T::zero(), T::one()) {
            Self::U
        } else if x == Complex::new(T::zero(), -T::one()) {
            Self::D
        } else if x == Complex::new(-T::one(), T::zero()) {
            Self::L
        } else if x == Complex::new(T::one(), T::zero()) {
            Self::R
        } else {
            unreachable!("Invalid Dir");
        }
    }
}

impl<T: Signed> From<Dir> for Complex<T> {
    fn from(x: Dir) -> Self {
        match x {
            Dir::U => Complex::new(T::zero(), T::one()),
            Dir::D => Complex::new(T::zero(), -T::one()),
            Dir::L => Complex::new(-T::one(), T::zero()),
            Dir::R => Complex::new(T::one(), T::zero()),
        }
    }
}

#[derive(Debug, PartialEq, Eq, EnumString, Clone, Copy)]
pub enum Turn {
    L,
    R,
    N,
}

impl<T: Signed> From<Turn> for Complex<T> {
    fn from(x: Turn) -> Self {
        match x {
            Turn::L => Complex::new(T::zero(), T::one()),
            Turn::R => Complex::new(T::zero(), -T::one()),
            Turn::N => Complex::new(T::one(), T::zero()),
        }
    }
}
