// TODO: Remove this module, stop using nalgebra, use homegrown grid
use nalgebra::{Dim, Matrix, RawStorage};
use num::Integer;

/// Trait extension on [`nalgebra::Matrix`] struct type.
pub trait MatrixExt<'a> {
    type EnumerateType;

    /// An iterator producing each element of the matrix as a pair of `(pos, &item)`
    /// where the index `pos` is, in turn, a pair of `(row, col)` indices.
    fn enumerate(&'a self) -> Self::EnumerateType;
}

impl<'a, T, R, C, S> MatrixExt<'a> for Matrix<T, R, C, S>
where
    T: 'a,
    R: 'a + Dim,
    C: 'a + Dim,
    S: 'a + RawStorage<T, R, C>,
{
    type EnumerateType = MatrixEnumerate<'a, T, R, C, S>;

    fn enumerate(&'a self) -> Self::EnumerateType {
        MatrixEnumerate {
            matrix: self,
            consumed: 0,
        }
    }
}

/// Column-major iterator for [`nalgebra::Matrix`] with row and column indices.
#[derive(Clone)]
pub struct MatrixEnumerate<'a, T, R, C, S> {
    matrix: &'a Matrix<T, R, C, S>,
    consumed: usize,
}

impl<'a, T, R, C, S> Iterator for MatrixEnumerate<'a, T, R, C, S>
where
    T: 'a,
    R: 'a + Dim,
    C: 'a + Dim,
    S: 'a + RawStorage<T, R, C>,
{
    type Item = ((usize, usize), &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let (rows, cols) = self.matrix.shape();
        (self.consumed < rows * cols).then(|| {
            let (j, i) = self.consumed.div_mod_floor(&rows);
            self.consumed += 1;
            ((i, j), &self.matrix[(i, j)])
        })
    }
}
