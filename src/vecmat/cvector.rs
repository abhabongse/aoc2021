//! Implements basic vector type and utilities.
use std::ops::{Add, Index, IndexMut, Mul, Neg, Sub};

use itertools::izip;
use num::{Signed, Zero};

use crate::collect_array::CollectArray;
use crate::vecmat::CMatrix;

/// Vector whose size is known at compile time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CVector<T, const SIZE: usize>(pub(crate) [T; SIZE]);

impl<T, const SIZE: usize> CVector<T, SIZE> {
    /// Creates a new vector from a constant size array.
    pub fn new(elements: [T; SIZE]) -> Self {
        CVector(elements)
    }

    /// References elements of the vector as a slice.
    pub fn as_slice(&self) -> &[T] {
        self.0.as_slice()
    }

    /// References elements of the vector as a mutable slice.
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        self.0.as_mut_slice()
    }

    /// Clone elements of the vector into a new [`std::vec::Vec`].
    pub fn to_vec(&self) -> Vec<T>
    where
        T: Clone,
    {
        self.as_slice().to_vec()
    }

    /// Gets a reference to an element from the vector.
    pub fn get(&self, index: usize) -> Option<&T> {
        self.0.get(index)
    }

    /// Gets a mutable reference to an element from the vector.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.0.get_mut(index)
    }

    /// Computes the L1 norm of the vector.
    pub fn norm1(&self) -> T
    where
        T: Copy + Signed,
    {
        self.0
            .iter()
            .copied()
            .fold(T::zero(), |acc, x| acc + x.abs())
    }

    /// Computes the maximum norm of the vector.
    pub fn norm_max(&self) -> T
    where
        T: Copy + Signed + Ord,
    {
        self.0.iter().copied().map(|x| x.abs()).max().unwrap()
    }

    /// An iterator over the index and the value
    pub fn iter(&self) -> impl Iterator<Item = (usize, &T)> {
        self.0.iter().enumerate()
    }

    /// A mutable iterator over the index and the value
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (usize, &mut T)> {
        self.0.iter_mut().enumerate()
    }

    /// An iterator over the elements of the vector
    pub fn values(&self) -> impl Iterator<Item = &T> {
        self.0.iter()
    }

    /// A mutable iterator over the elements of the vector
    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.0.iter_mut()
    }
}

impl<T, const SIZE: usize> Index<usize> for CVector<T, SIZE> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl<T, const SIZE: usize> IndexMut<usize> for CVector<T, SIZE> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}

impl<T, const SIZE: usize> Add for CVector<T, SIZE>
where
    T: Copy + Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let result: [_; SIZE] = izip!(self.0.into_iter(), rhs.0.into_iter())
            .map(|(a, b)| a + b)
            .collect_exact()
            .unwrap();
        CVector(result)
    }
}

impl<T, const SIZE: usize> Sub for CVector<T, SIZE>
where
    T: Copy + Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let result: [_; SIZE] = izip!(self.0.into_iter(), rhs.0.into_iter())
            .map(|(a, b)| a - b)
            .collect_exact()
            .unwrap();
        CVector(result)
    }
}

impl<T, const SIZE: usize> Neg for CVector<T, SIZE>
where
    T: Copy + Neg<Output = T>,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        let result: [_; SIZE] = self.0.into_iter().map(|a| -a).collect_exact().unwrap();
        CVector(result)
    }
}

impl<T, const SIZE: usize> Zero for CVector<T, SIZE>
where
    T: Copy + Add<Output = T> + Zero,
{
    fn zero() -> Self {
        let result = [T::zero(); SIZE];
        CVector(result)
    }

    fn is_zero(&self) -> bool {
        self.0.iter().all(|a| a.is_zero())
    }
}

impl<T, const JSIZE: usize, const KSIZE: usize> Mul<CMatrix<T, JSIZE, KSIZE>> for CVector<T, JSIZE>
where
    T: Copy + Add<Output = T> + Zero + Mul<Output = T>,
{
    type Output = CVector<T, KSIZE>;

    fn mul(self, rhs: CMatrix<T, JSIZE, KSIZE>) -> Self::Output {
        let result: [_; KSIZE] = (0..KSIZE)
            .map(|k| {
                (0..JSIZE)
                    .map(|j| self.0[j] * rhs.0[j][k])
                    .fold(T::zero(), |acc, x| acc + x)
            })
            .collect_exact()
            .unwrap();
        CVector(result)
    }
}

impl<T> CVector<T, 1> {
    /// First member of the vector
    pub fn x(&self) -> T
    where
        T: Copy,
    {
        self.0[0]
    }
}

impl<T> CVector<T, 2> {
    /// First member of the vector
    pub fn x(&self) -> T
    where
        T: Copy,
    {
        self.0[0]
    }

    /// Second member of the vector
    pub fn y(&self) -> T
    where
        T: Copy,
    {
        self.0[1]
    }
}

impl<T> CVector<T, 3> {
    /// First member of the vector
    pub fn x(&self) -> T
    where
        T: Copy,
    {
        self.0[0]
    }

    /// Second member of the vector
    pub fn y(&self) -> T
    where
        T: Copy,
    {
        self.0[1]
    }

    /// Third member of the vector
    pub fn z(&self) -> T
    where
        T: Copy,
    {
        self.0[2]
    }
}

impl<T> CVector<T, 4> {
    /// First member of the vector
    pub fn x(&self) -> T
    where
        T: Copy,
    {
        self.0[0]
    }

    /// Second member of the vector
    pub fn y(&self) -> T
    where
        T: Copy,
    {
        self.0[1]
    }

    /// Third member of the vector
    pub fn z(&self) -> T
    where
        T: Copy,
    {
        self.0[2]
    }

    /// Fourth member of the vector
    pub fn w(&self) -> T
    where
        T: Copy,
    {
        self.0[3]
    }
}
