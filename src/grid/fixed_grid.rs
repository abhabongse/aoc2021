use std::ops::{Index, IndexMut};

use anyhow::{bail, ensure};
use itertools::iproduct;

use super::GridIndex;

/// A heap-allocated, two-dimensional grid structure with static size.
///
/// # Implementation Note
/// This is my attempt to provide an alternative implementation
/// for a fix-sized two-dimensional array, with proper index API.
#[derive(Debug, Clone)]
pub struct FixedGrid<T, const R: usize, const C: usize>(Box<[T]>);

impl<T, const R: usize, const C: usize> FixedGrid<T, R, C> {
    /// Transforms a two-dimensional index into a flatten index.
    fn transform_index(&self, index: GridIndex) -> anyhow::Result<usize> {
        let (i, j) = index;
        if i >= R {
            bail!(
                "incorrect row index {} (must be between {} and {})",
                i,
                0,
                R - 1
            );
        } else if j >= C {
            bail!(
                "incorrect column index {} (must be between {} and {})",
                j,
                0,
                C - 1
            );
        } else {
            Ok(i * C + j)
        }
    }

    /// Returns a reference to an item in the grid; `None` if out of bounds.
    pub fn get(&self, index: GridIndex) -> Option<&<Self as Index<GridIndex>>::Output> {
        let index = self.transform_index(index).ok()?;
        self.0.get(index)
    }

    /// Returns a mutable reference to an item in the grid; `None` if out of bounds.
    pub fn get_mut(&mut self, index: GridIndex) -> Option<&mut <Self as Index<GridIndex>>::Output> {
        let index = self.transform_index(index).ok()?;
        self.0.get_mut(index)
    }

    /// An iterator which produces row-major indices of the grid.
    pub fn indices_by_row(&self) -> Box<dyn Iterator<Item = GridIndex>> {
        Box::new(iproduct!(0..R, 0..C))
    }

    /// An iterator which produces column-major indices of the grid.
    pub fn indices_by_column(&self) -> Box<dyn Iterator<Item = GridIndex>> {
        Box::new(iproduct!(0..C, 0..R).map(|(j, i)| (i, j)))
    }
}

impl<T, const R: usize, const C: usize> TryFrom<Box<[T]>> for FixedGrid<T, R, C> {
    type Error = anyhow::Error;

    fn try_from(value: Box<[T]>) -> Result<Self, Self::Error> {
        ensure!(
            value.len() == R * C,
            "data size does not fit with grid shape"
        );
        Ok(FixedGrid(value))
    }
}

impl<T, const R: usize, const C: usize> TryFrom<Vec<Vec<T>>> for FixedGrid<T, R, C> {
    type Error = anyhow::Error;

    fn try_from(value: Vec<Vec<T>>) -> Result<Self, Self::Error> {
        let mut grid = Vec::with_capacity(R * C);
        ensure!(
            value.len() == R,
            "number of rows {} does not match the expected {}",
            value.len(),
            R
        );
        for (i, mut row) in value.into_iter().enumerate() {
            ensure!(
                row.len() == C,
                "number of items {} does not match the expected {} for row {}",
                row.len(),
                C,
                i + 1
            );
            grid.append(&mut row);
        }
        Ok(FixedGrid(grid.into_boxed_slice()))
    }
}

impl<T, const R: usize, const C: usize> Index<GridIndex> for FixedGrid<T, R, C> {
    type Output = T;

    fn index(&self, index: GridIndex) -> &Self::Output {
        let index = self.transform_index(index).unwrap();
        &self.0[index]
    }
}

impl<T, const R: usize, const C: usize> IndexMut<(usize, usize)> for FixedGrid<T, R, C> {
    fn index_mut(&mut self, index: GridIndex) -> &mut Self::Output {
        let index = self.transform_index(index).unwrap();
        &mut self.0[index]
    }
}
