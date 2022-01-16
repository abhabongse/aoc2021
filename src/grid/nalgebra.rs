use crate::grid::enumerate::GridIndices;
use nalgebra::{Dim, Matrix, RawStorage};

/// Trait extension on [`nalgebra::Matrix`] struct type
pub trait MatrixExt {
    /// An iterator producing column-major indices of the matrix
    fn indices(&self) -> GridIndices;
}

impl<T, R, C, S> MatrixExt for Matrix<T, R, C, S>
where
    R: Dim,
    C: Dim,
    S: RawStorage<T, R, C>,
{
    fn indices(&self) -> GridIndices {
        GridIndices::column_major(self.shape())
    }
}
