//! Custom implementation of grid data structure
//! TODO: Implements Philip N. Klein's style of Vector and Matrix library.

pub use self::nalgebra::MatrixExt;
pub use base::{king_adjacent, orth_adjacent, GridIndices, GridPoint};

mod base;
mod nalgebra;
