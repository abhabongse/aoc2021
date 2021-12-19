use crate::grid::GridPoint;
use nalgebra::{Dim, Matrix, RawStorage};
use num::Integer;

/// Trait extension on [`nalgebra::Matrix`] struct type
pub trait MatrixExt {
    /// An iterator producing column-major indices of the matrix
    fn indices(&self) -> MatIndices;
}

impl<T, R, C, S> MatrixExt for Matrix<T, R, C, S>
where
    R: Dim,
    C: Dim,
    S: RawStorage<T, R, C>,
{
    fn indices(&self) -> MatIndices {
        MatIndices {
            shape: self.shape(),
            consumed: 0,
        }
    }
}

/// Column-major iterator for [`nalgebra::Matrix`] with row and column indices.
#[derive(Clone)]
pub struct MatIndices {
    shape: GridPoint,
    consumed: usize,
}

impl Iterator for MatIndices {
    type Item = GridPoint;

    fn next(&mut self) -> Option<Self::Item> {
        let (rows, cols) = self.shape;
        (self.consumed < rows * cols).then(|| {
            let (j, i) = self.consumed.div_mod_floor(&rows);
            self.consumed += 1;
            (i, j)
        })
    }
}
