use num::Integer;

/// A two-dimensional grid point, can be used to describe the shape of the grid
/// as well as describe an indexed position into a cell in the grid.
pub type GridPoint = (usize, usize);

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
    pub fn row_major(shape: GridPoint) -> Self {
        GridIndices {
            nrows: shape.0,
            ncols: shape.1,
            consumed: 0,
            order: DataOrder::RowMajor,
        }
    }

    /// Column-major grid indices from shape
    pub fn column_major(shape: GridPoint) -> Self {
        GridIndices {
            nrows: shape.0,
            ncols: shape.1,
            consumed: 0,
            order: DataOrder::ColumnMajor,
        }
    }
}

impl Iterator for GridIndices {
    type Item = GridPoint;

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

/// Finds a sequence of up to four grid points which are orthogonally (horizontally or vertically)
/// adjacent to the given `pos` and are bound within the given rectangular `shape`,
pub fn orth_adjacent(pos: GridPoint, shape: GridPoint) -> Vec<GridPoint> {
    [(-1, 0), (1, 0), (0, -1), (0, 1)]
        .into_iter()
        .filter_map(|(di, dj)| {
            Some((
                clipped_add(pos.0, di, shape.0)?,
                clipped_add(pos.1, dj, shape.1)?,
            ))
        })
        .collect()
}

/// Finds a sequence of up to eight grid points which is a king's move away from the given `pos`
/// (i.e. horizontally, vertically, or diagonally adjacent grid points)
/// and are bound within the given rectangular `shape`.
pub fn king_adjacent(pos: GridPoint, shape: GridPoint) -> Vec<GridPoint> {
    [
        (-1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
    ]
    .into_iter()
    .filter_map(|(di, dj)| {
        Some((
            clipped_add(pos.0, di, shape.0)?,
            clipped_add(pos.1, dj, shape.1)?,
        ))
    })
    .collect()
}

fn clipped_add(a: usize, b: i64, size: usize) -> Option<usize> {
    let total = (a as i64) + b;
    (0..size as i64).contains(&total).then(|| total as usize)
}
