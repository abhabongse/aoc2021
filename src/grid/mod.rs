//! Custom implementation of grid data structure
//! TODO: Add grid type whose size cannot be determined at compile time
//! TODO: Add grid item iterators
//! TODO: Add math binary operations for grids
//! TODO: implements Philip N. Klein's style of Vector and Matrix library.

pub use fixed_grid::FixedGrid;
pub use util::{king_step_neighbors, orthogonal_neighbors};

/// A two-dimensional grid point, can be used to describe the shape of the grid
/// as well as describe an indexed position into a cell in the grid.
pub type GridPoint = (usize, usize);

mod fixed_grid;
mod util;
mod var_grid;
