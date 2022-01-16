use num::Integer;

use crate::grid::GridPoint;

/// Iterator for `(row, column)` indices over a grid
/// - TODO: Implement [`std::iter::DoubleEndedIterator`]
#[derive(Debug, Clone)]
pub struct GridIndices {
    /// Number of rows in the grid
    nrows: usize,
    /// Number of column in the grid
    ncols: usize,
    /// Number of items already consumed
    consumed: usize,
    /// Whether to iterator in row-major or column-major order
    order: DataOrder,
}

/// Ordering of data layout in memory
#[derive(Debug, Clone)]
enum DataOrder {
    /// Each element in a row is consecutive
    RowMajor,
    /// Each element in a column is consecutive
    ColumnMajor,
}

impl GridIndices {
    /// Row-major grid indices from shape
    pub fn row_major(shape: GridPoint<usize>) -> Self {
        GridIndices {
            nrows: shape.0,
            ncols: shape.1,
            consumed: 0,
            order: DataOrder::RowMajor,
        }
    }

    /// Column-major grid indices from shape
    pub fn column_major(shape: GridPoint<usize>) -> Self {
        GridIndices {
            nrows: shape.0,
            ncols: shape.1,
            consumed: 0,
            order: DataOrder::ColumnMajor,
        }
    }
}

impl Iterator for GridIndices {
    type Item = GridPoint<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.consumed < self.nrows * self.ncols {
            let (i, j) = match self.order {
                DataOrder::RowMajor => self.consumed.div_mod_floor(&self.ncols),
                DataOrder::ColumnMajor => {
                    let (j, i) = self.consumed.div_mod_floor(&self.nrows);
                    (i, j)
                }
            };
            self.consumed += 1;
            Some((i, j))
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.nrows * self.ncols - self.consumed;
        (size, Some(size))
    }
}

impl ExactSizeIterator for GridIndices {}
