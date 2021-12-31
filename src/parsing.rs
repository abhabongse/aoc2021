//! Implements a trait extension for [`str`] primitive type which adds the method
//! [`quickparse`] to simplify fallible parsing with [`anyhow::Result`] return type.
//!
//! [`quickparse`]: QuickParse::quickparse
use std::str::FromStr;

use anyhow::anyhow;

/// Trait extension for [`str`] primitive type which adds [`quickparse`] method.
///
/// [`quickparse`]: QuickParse::quickparse
pub trait QuickParse {
    /// Shortcut method to parse a string into a target type that implements [`FromStr`].
    ///
    /// [`FromStr`]: std::str::FromStr
    fn quickparse<F>(&self) -> anyhow::Result<F>
    where
        F: FromStr;
}

impl QuickParse for str {
    fn quickparse<F>(&self) -> anyhow::Result<F>
    where
        F: FromStr,
    {
        self.parse().map_err(|_| {
            anyhow!(
                "cannot parse token for type {}: {}",
                std::any::type_name::<F>(),
                self
            )
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_int_ok() {
        let x: i64 = "-37".quickparse().unwrap();
        assert_eq!(x, -37);
        let x: usize = "683".quickparse().unwrap();
        assert_eq!(x, 683);
    }

    #[test]
    fn simple_int_err() {
        assert_eq!(
            "abc".quickparse::<isize>().unwrap_err().to_string(),
            "cannot parse token for type isize: abc"
        );
        assert_eq!(
            "-3".quickparse::<u32>().unwrap_err().to_string(),
            "cannot parse token for type u32: -3"
        );
    }
}
