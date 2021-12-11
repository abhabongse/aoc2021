//! Custom implementation of grid data structure
//! TODO: Add grid type whose size cannot be determined at compile time
//! TODO: Add grid item iterators
//! TODO: Add math binary operations for grids

pub use fixed_grid::FixedGrid;
pub use util::{king_step_neighbors, orthogonal_neighbors};

/// A two-dimensional grid indexing type.
pub type GridIndex = (usize, usize);

mod fixed_grid;
mod util;
