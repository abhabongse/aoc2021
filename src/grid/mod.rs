//! Custom implementation of grid data structure
pub use crate::grid::adjacency::{KingAdjacent, OrthAdjacent};
pub use crate::grid::enumerate::GridIndices;
pub use crate::grid::nalgebra::MatrixExt;

/// A tuple-pair describing grid point represented by integer coordinates on cartesian plane.
pub type GridPoint<T> = (T, T);

mod adjacency;
mod enumerate;
mod nalgebra;
