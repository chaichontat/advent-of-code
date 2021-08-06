use std::ops::{Index, IndexMut};
use std::slice::{ChunksExact, ChunksExactMut};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Array2D<T: Default, const N: usize> {
    a:    [T; N],
    xlen: usize,
}

impl<T: Default + Copy, const N: usize> Array2D<T, N> {
    pub fn new(xlen: usize) -> Array2D<T, N> {
        Array2D { a: [T::default(); N], xlen }
    }
}

impl<T: Default + Copy, const N: usize> Index<[usize; 2]> for Array2D<T, N> {
    type Output = T;

    fn index(&self, idx: [usize; 2]) -> &Self::Output {
        &self.a[idx[0] * self.xlen + idx[1]]
    }
}

impl<T: Default + Copy, const N: usize> IndexMut<[usize; 2]> for Array2D<T, N> {
    fn index_mut(&mut self, idx: [usize; 2]) -> &mut Self::Output {
        &mut self.a[idx[0] * self.xlen + idx[1]]
    }
}

impl<T: Default + Copy, const N: usize> Index<usize> for Array2D<T, N> {
    type Output = [T];

    fn index(&self, idx: usize) -> &Self::Output {
        &self.a[idx * self.xlen..(idx + 1) * self.xlen]
    }
}

impl<T: Default + Copy, const N: usize> IndexMut<usize> for Array2D<T, N> {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.a[idx * self.xlen..(idx + 1) * self.xlen]
    }
}

impl<'a, T: Default, const N: usize> IntoIterator for &'a Array2D<T, N> {
    type IntoIter = ChunksExact<'a, T>;
    type Item = &'a [T];

    fn into_iter(self) -> Self::IntoIter {
        self.a.chunks_exact(self.xlen)
    }
}

impl<'a, T: Default, const N: usize> IntoIterator for &'a mut Array2D<T, N> {
    type IntoIter = ChunksExactMut<'a, T>;
    type Item = &'a mut [T];

    fn into_iter(self) -> Self::IntoIter {
        self.a.chunks_exact_mut(self.xlen)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Vec2D<T> {
    v:     Vec<T>,
    y_len: usize,
    x_len: usize,
}

impl<T: Default + Clone> Vec2D<T> {
    pub fn new(dim: [usize; 2]) -> Self {
        let v = vec![T::default(); dim[0] * dim[1]];
        Vec2D { v, x_len: dim[1], y_len: dim[0] }
    }

    pub fn dim(&self) -> (usize, usize) {
        (self.y_len, self.x_len)
    }

    /// # Safety
    /// Index must be in [0, self.y_len * self.x_len).
    pub unsafe fn getu(&self, idx: [usize; 2]) -> &T {
        self.v.get_unchecked(idx[0] * self.x_len + idx[1])
    }

    /// # Safety
    /// Index must be in [0, self.y_len * self.x_len).
    pub unsafe fn get_unchecked_mut(&mut self, idx: [usize; 2]) -> &mut T {
        self.v.get_unchecked_mut(idx[0] * self.x_len + idx[1])
    }

    /// # Safety
    /// Index must be in [0, self.y_len).
    pub unsafe fn get_row_u(&self, idx: usize) -> &[T] {
        self.v.get_unchecked(idx * self.x_len..(idx + 1) * self.x_len)
    }

    /// # Safety
    /// Index must be in [0, self.y_len).
    pub unsafe fn get_row_unchecked_mut(&mut self, idx: usize) -> &mut [T] {
        self.v.get_unchecked_mut(idx * self.x_len..(idx + 1) * self.x_len)
    }
}

impl<T> Index<[usize; 2]> for Vec2D<T> {
    type Output = T;

    fn index(&self, idx: [usize; 2]) -> &Self::Output {
        &self.v[idx[0] * self.x_len + idx[1]]
    }
}

impl<T> IndexMut<[usize; 2]> for Vec2D<T> {
    fn index_mut(&mut self, idx: [usize; 2]) -> &mut Self::Output {
        &mut self.v[idx[0] * self.x_len + idx[1]]
    }
}

impl<T> Index<usize> for Vec2D<T> {
    type Output = [T];

    fn index(&self, idx: usize) -> &Self::Output {
        &self.v[idx * self.x_len..(idx + 1) * self.x_len]
    }
}

impl<T> IndexMut<usize> for Vec2D<T> {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.v[idx * self.x_len..(idx + 1) * self.x_len]
    }
}

impl<'a, T> IntoIterator for &'a Vec2D<T> {
    type IntoIter = ChunksExact<'a, T>;
    type Item = &'a [T];

    fn into_iter(self) -> Self::IntoIter {
        self.v.chunks_exact(self.x_len)
    }
}

impl<'a, T> IntoIterator for &'a mut Vec2D<T> {
    type IntoIter = ChunksExactMut<'a, T>;
    type Item = &'a mut [T];

    fn into_iter(self) -> Self::IntoIter {
        self.v.chunks_exact_mut(self.x_len)
    }
}

mod tests {
    use super::{Array2D, Vec2D};
    use crate::utils::printt;

    #[test]
    fn test_whatamidoing() {
        let mut x = Vec2D::<u8>::new([5, 4]);
        for u in &mut x {
            u.copy_from_slice(&[1, 2, 3, 4]);
        }
    }

    #[test]
    fn test_arr2d() {
        let mut x = Array2D::<u8, 10>::new(5);
        for u in &mut x {
            u.copy_from_slice(&[1, 2, 3, 4, 5]);
        }
    }
}
