//! Custom implementation of grid data structure
//! TODO: Implements Philip N. Klein's style of Vector and Matrix library.

pub use self::nalgebra::MatrixExt;
pub use util::{king_adjacent, orth_adjacent, GridPoint};

mod nalgebra;
mod util;
